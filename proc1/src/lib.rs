use proc_macro::TokenStream;
use quote::quote;

#[proc_macro]
pub fn empty_proc_macro(_input: TokenStream) -> TokenStream {
    TokenStream::new()
}

#[proc_macro]
pub fn identity_proc_macro(input: TokenStream) -> TokenStream {
    input
}

#[proc_macro]
pub fn make_answer(_item: TokenStream) -> TokenStream {
    "fn answer() -> u32 { 42 }".parse().unwrap()
}

#[proc_macro]
pub fn println_proc_macro(input: TokenStream) -> TokenStream {
    println!("Processing println_proc_macro {}", input.to_string());
    input
}

#[proc_macro]
pub fn string_proc_macro(input: TokenStream) -> TokenStream {
    let str = input.to_string().replace("\"", "\\\"");
    let code = format!("println!(\"Code: {}\")", str);
    println!("{}", code);
    code.parse().unwrap()
}

#[proc_macro_attribute]
pub fn attribute_proc_macro(input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    println!("Attribue proc macro input: {}", input.to_string());
    println!("Attribue proc macro annotated item: {}", annotated_item.to_string());
    annotated_item
}

#[proc_macro_attribute]
pub fn replace_12_to_13(_input: TokenStream, annotated_item: TokenStream) -> TokenStream {
    let str = annotated_item.to_string();
    let updated_str = str.replace("12", "13");
    updated_str.parse().unwrap()
}

#[proc_macro_derive(HelloTrait)]
pub fn sparkbit_derive(input: TokenStream) -> TokenStream {
    let ast: syn::DeriveInput = syn::parse(input).unwrap();
    let name: syn::Ident = ast.ident;
    let gen = quote! {
        impl HelloTrait for #name {
            fn hello_macro() {
                println!("Hello, Macro! My name is {}!", stringify!(#name));
            }
        }
    };
    gen.into()
}

