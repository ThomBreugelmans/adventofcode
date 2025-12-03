use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse::{Parse, ParseStream},
    punctuated::Punctuated,
    Ident, LitInt, Token,
};

/// Represents the arguments inside the #[solution(...)] attribute.
struct SolutionArgs {
    year: LitInt,
    day: LitInt,
    part: LitInt,
}

impl Parse for SolutionArgs {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        // Use Punctuated::parse_terminated to handle the comma-separated list
        let fields: Punctuated<SolutionField, Token![,]> =
            Punctuated::parse_terminated(input)?;

        // Now extract the values from the parsed fields
        let mut year: Option<LitInt> = None;
        let mut day: Option<LitInt> = None;
        let mut part: Option<LitInt> = None;

        for field in fields {
            match field.name.to_string().as_str() {
                "year" => year = Some(field.value),
                "day" => day = Some(field.value),
                "part" => part = Some(field.value),
                _ => return Err(input.error("Unexpected attribute argument")),
            }
        }

        Ok(SolutionArgs {
            year: year.ok_or_else(|| input.error("Missing 'year' argument"))?,
            day: day.ok_or_else(|| input.error("Missing 'day' argument"))?,
            part: part.ok_or_else(|| input.error("Missing 'part' argument"))?,
        })
    }
}

/// Helper struct for parsing a single `key=value` pair.
struct SolutionField {
    name: Ident,
    _eq: Token![=], // The '=' token
    value: LitInt,  // The integer literal value
}

impl Parse for SolutionField {
    fn parse(input: ParseStream) -> syn::Result<Self> {
        Ok(SolutionField {
            name: input.parse()?, // Parses the identifier (e.g., 'year')
            _eq: input.parse()?,  // Parses the '=' token
            value: input.parse()?, // Parses the integer literal (e.g., '2025')
        })
    }
}

#[proc_macro_attribute]
pub fn solution(attr: TokenStream, item: TokenStream) -> TokenStream {
    // 1. Parse the attribute tokens into our defined structure
        let args = syn::parse_macro_input!(attr as SolutionArgs);

        // 2. Parse the function definition
        let input_fn = syn::parse_macro_input!(item as syn::ItemFn);
        let func_name = &input_fn.sig.ident;
        let func_name_str = func_name.to_string();

        // 3. Extract the values (LitInt implements Display/ToTokens)
        let year = args.year;
        let day = args.day;
        let part = args.part;

        // 4. Generate the registration code (similar to the inventory example)
        let submission = quote! {
            inventory::submit! {
                lib::Solution {
                    year: #year,
                    day: #day,
                    part: #part,
                    name: #func_name_str,
                    func: #func_name,
                }
            }
        };

        // 5. Return the expanded code
        let expanded = quote! {
            #input_fn
            #submission
        };

        expanded.into()
}
