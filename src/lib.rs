//! Compile-time lifetimes for comments.

use cargo_toml::Manifest;
use chrono::{NaiveDate, Utc};
use proc_macro::TokenStream;
use quote::quote;
use semver::{Version, VersionReq};
use syn::{parse::Parse, parse_macro_input, LitStr};

struct TodoByArgs {
    date: NaiveDate,
    comment: Option<String>,
}

impl Parse for TodoByArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let date_str = input.parse::<LitStr>()?.value();
        let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").expect("Not a valid TODO date");
        let comment = if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?; // Skip the comma
            Some(input.parse::<LitStr>()?.value())
        } else {
            None
        };

        Ok(Self { date, comment })
    }
}

/// A macro to set a lifetime for a TODO, with an optional comment.
///
/// The date is provided as {year}-{month}-{day}. Once the date has passed, the compiler
/// will throw an error.
///
/// # Examples
/// ```
/// # use todo_by::todo_by;
/// todo_by!("2030-01-01");
/// todo_by!("2029-05-22", "Make this a constant for better perf");
/// ```
///
/// If the date has not yet passed, the macro will expand to nothing - no bloat.
#[proc_macro]
pub fn todo_by(item: TokenStream) -> TokenStream {
    let TodoByArgs { date, comment } = parse_macro_input!(item as TodoByArgs);
    let now = Utc::now().date_naive();

    if date < now {
        // Format into human-readable date like "Jan 1, 2022"
        let date_str = date.format("%b %-d, %Y").to_string();

        let error_message = if let Some(comment) = comment {
            format!("TODO by {date_str} has passed: {comment}")
        } else {
            format!("TODO by {date_str} has passed")
        };
        return quote! { compile_error!(#error_message); }.into();
    }

    TokenStream::new()
}

struct TodoByVersionArgs {
    version: VersionReq,
    comment: Option<String>,
}

impl Parse for TodoByVersionArgs {
    fn parse(input: syn::parse::ParseStream) -> syn::Result<Self> {
        let version_str = input.parse::<LitStr>()?.value();
        let version = VersionReq::parse(&version_str).unwrap();
        let comment = if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?; // Skip the comma
            Some(input.parse::<LitStr>()?.value())
        } else {
            None
        };

        Ok(Self { version, comment })
    }
}

/// A macro to set a lifetime for a TODO, with an optional comment.
///
/// The semantic version is provided as a requirement. Once the version in the Cargo.toml fails
/// to meed the requirement, the compiler will throw an error.
///
/// # Examples
/// ```
/// # use todo_by::todo_by_version;
/// todo_by_version!("<1.0.0");
/// todo_by_version!("<2.0.0", "Make this a constant for better perf");
/// ```
///
/// If the version requiremnt is satisified, the macro will expand to nothing - no bloat.
#[proc_macro]
pub fn todo_by_version(item: TokenStream) -> TokenStream {
    let TodoByVersionArgs { version, comment } = parse_macro_input!(item as TodoByVersionArgs);

    let nul_version = Version::parse("0.0.0").unwrap();
    let now_version = match Manifest::from_path("Cargo.toml") {
        Ok(a) => match a.package {
            Some(p) => match p.version.get() {
                Ok(v) => Version::parse(v).unwrap(),
                Err(_) => nul_version, // can't happen
            },
            None => nul_version, // can't happen
        },
        Err(_) => nul_version, // can't hapen
    };

    if !version.matches(&now_version) {
        let ver_str = version.to_string();

        let error_message = if let Some(comment) = comment {
            format!("TODO b4 {ver_str} not satisfied: {comment}")
        } else {
            format!("TODO b4 {ver_str} not satisfied")
        };
        return quote! { compile_error!(#error_message); }.into();
    }

    TokenStream::new()
}
