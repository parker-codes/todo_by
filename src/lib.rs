use chrono::{NaiveDate, Utc};
use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, LitStr};

#[proc_macro]
pub fn todo_by(item: TokenStream) -> TokenStream {
    let input = item.clone();
    let input = parse_macro_input!(input as LitStr);
    let input_str = input.value();
    let date_str = input_str.trim_matches('"');
    let date = NaiveDate::parse_from_str(&date_str, "%Y-%m-%d").expect("Not a valid TODO date");
    let now = Utc::now().date_naive();

    let mut output = TokenStream::new();

    if date < now {
        let error_message = format!("TODO by {} has already passed", date_str);
        output.extend::<TokenStream>(
            quote! {
                compile_error!(#error_message);
            }
            .into(),
        );
    }

    output.extend(item);

    output
}
