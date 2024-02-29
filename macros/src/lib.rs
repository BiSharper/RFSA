use proc_macro::TokenStream;
use syn::DeriveInput;

#[proc_macro_derive(VMeta)]
pub fn proc_gfs_meta_derive(input: TokenStream) -> TokenStream {
    let ast: DeriveInput = syn::parse(input).unwrap();
    let meta_type = ast.ident;

    return quote::quote! {
        impl super::rvfs::VMetadata for #meta_type {

        }
    }.into()
}