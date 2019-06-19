extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use quote::ToTokens;
use syn;

//##########################################
// #[derive] mode macros

#[proc_macro_derive(HelloMacro)]
pub fn hello_macro_derive(input: TokenStream) -> TokenStream {
    // Construct a representation of Rust code as a syntax tree
    // that we can manipulate
    let ast: syn::DeriveInput = syn::parse(input).unwrap();

    // Build the trait implementation
    impl_hello_macro(&ast)
}

fn impl_hello_macro(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl HelloMacro for #name {
            fn hello_macro() -> String {
                format!("Hello, Macro! My name is {}", stringify!(#name))
            }
        }
    };
    gen.into()
}

//##########################################
// Attribute-like macros

#[proc_macro_attribute]
pub fn hello(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // syn::ItemFn requires feature "full"
    let input = syn::parse_macro_input!(item as syn::ItemFn);
    let name = &input.ident;

    // Our input function is always equivalent to returning 42, right?
    let result = quote! {
        fn #name() -> u32 { 42 }
    };
    result.into()
}

#[proc_macro_attribute]
pub fn struct_extension(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // syn::ItemStruct requires feature "full"
    let input = syn::parse_macro_input!(item as syn::ItemStruct);

    let name = &input.ident;

    let result = match input.fields {
        syn::Fields::Named(ref fields) => {
            let fields = &fields.named;
            quote! {
                struct #name {
                    #fields
                    append: String,
                }
            }
        }
        syn::Fields::Unnamed(ref _fields) => panic!("not support now!"),
        syn::Fields::Unit => panic!("not support now!"),
    };

    result.into()
}

#[proc_macro_attribute]
pub fn impl_trait(_attr: TokenStream, item: TokenStream) -> TokenStream {
    // syn::ItemImpl requires feature "full"
    let mut input = syn::parse_macro_input!(item as syn::ItemImpl);

    let result = quote! {
        fn hello_macro() -> String {
            "hello".to_owned()
        }
    }
    .into();

    let result = syn::parse_macro_input!(result as syn::ImplItemMethod);

    input.items.push(syn::ImplItem::Method(result));

    input.into_token_stream().into()
}

//##########################################
// Function-like macros

#[proc_macro]
pub fn func_macro(input: TokenStream) -> TokenStream {
    let input = syn::parse_macro_input!(input as syn::LitStr);

    let r = quote! {
        fn hello_macro() -> String {
            #input.to_owned()
        }
    };

    r.into()
}

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
