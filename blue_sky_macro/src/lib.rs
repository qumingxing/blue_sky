#[macro_use]
extern crate quote;
extern crate proc_macro;
use proc_macro::TokenStream;
use syn::punctuated::Punctuated;
use syn::{parse_macro_input, Ident, ItemFn, Lit, LitStr, Token};

#[proc_macro_attribute]
pub fn route(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args: Vec<LitStr> =
        parse_macro_input!(attr with Punctuated::<LitStr, Token![,]>::parse_terminated)
            .into_iter()
            .collect();
    if args.len() != 2 {
        return TokenStream::from(
            quote! { compile_error!("Expected two arguments, HTTP method and path"); },
        );
    }
    let method = args[0].value();
    let path = args[1].value();
    let input = parse_macro_input!(item as ItemFn);
    let fn_name = &input.sig.ident;
    let register_method = format!("{}_{}", "register_route", fn_name.to_string());
    let register_method = Ident::new(register_method.as_str(), input.sig.ident.span());
    let expanded = quote! {
        #input

        impl crate::web_handler::WebHandler {
            pub fn #register_method(router:&mut crate::router::Router) {
                let f: fn(&crate::server::HttpRequest)->crate::router::Response = #fn_name;
                router.add_route(#method, #path, f);
            }
        }
    };

    TokenStream::from(expanded)
}
