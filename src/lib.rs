use chrono::{NaiveDate, Utc};
use proc_macro::TokenStream;
use quote::quote;
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
