use proc_macro::TokenStream;

#[proc_macro_attribute]
pub fn reexpand(_attr: TokenStream, tokens: TokenStream) -> TokenStream {
    let new = tokens.into_iter().collect();
    println!("Recollected: {:?}", new);
    new
}
