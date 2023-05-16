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
        let version = VersionReq::parse(&version_str).expect("Not a valid semver requirement");
        let comment = if input.peek(syn::Token![,]) {
            input.parse::<syn::Token![,]>()?; // Skip the comma
            Some(input.parse::<LitStr>()?.value())
        } else {
            None
        };

        Ok(Self { version, comment })
    }
}

fn current_version_str() -> Result<String, cargo_toml::Error> {
    // TryBuild test lines
    // let cur_file = std::path::PathBuf::from("Cargo.toml");
    // println!("{:?}", std::fs::canonicalize(&cur_file));
    Manifest::from_path("Cargo.toml")?
        .package
        .ok_or(cargo_toml::Error::Other("no package"))?
        .version
        .get()
        .cloned()
}

fn current_version() -> Version {
    Version::parse(&current_version_str().unwrap_or("0.0.0".to_owned())).unwrap()
}

/// A macro to set a lifetime for a TODO based on your Cargo.toml version, with an optional
/// comment.
///
/// The semantic version is provided as a requirement. Once the version in your Cargo.toml
/// meets the requirement, the compiler will throw an error.
///
/// # Examples
/// ```
/// # use todo_by::todo_while_version;
/// todo_while_version!("<1.3.1");
/// todo_while_version!("<=2.0.0", "Need to release this before v2 or else it will be incompatible");
/// ```
///
/// If the version requirement is not satisified, the macro will expand to nothing - no bloat.
#[proc_macro]
pub fn todo_while_version(item: TokenStream) -> TokenStream {
    let TodoByVersionArgs { version, comment } = parse_macro_input!(item as TodoByVersionArgs);
    let current_version = current_version();

    if !version.matches(&current_version) {
        let version_str = version.to_string();

        let error_message = if let Some(comment) = comment {
            format!("TODO version requirement '{version_str}' not satisfied by {current_version}: {comment}")
        } else {
            format!("TODO version requirement '{version_str}' not satisfied by {current_version}")
        };
        return quote! { compile_error!(#error_message); }.into();
    }

    TokenStream::new()
}
