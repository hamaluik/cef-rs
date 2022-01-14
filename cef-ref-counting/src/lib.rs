use proc_macro::TokenStream;
use quote::quote;
use syn::parse::Parser;
use syn::{parse, parse_macro_input, ItemStruct};

#[proc_macro_attribute]
pub fn ref_count(args: TokenStream, input: TokenStream) -> TokenStream {
    let mut item_struct = parse_macro_input!(input as ItemStruct);
    let _ = parse_macro_input!(args as parse::Nothing);

    if let syn::Fields::Named(ref mut fields) = item_struct.fields {
        fields.named.push(
            syn::Field::parse_named
                .parse2(quote! { ref_count: std::sync::atomic::AtomicIsize })
                .unwrap(),
        );
    }

    return quote! {
        #item_struct
    }
    .into();
}

#[proc_macro_derive(RefCount)]
pub fn ref_count_derive(input: TokenStream) -> TokenStream {
    let ast = syn::parse(input).unwrap();
    impl_ref_count_derive(&ast)
}

fn impl_ref_count_derive(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let gen = quote! {
        impl #name {
            pub unsafe extern "C" fn add_ref(base: *mut cef_sys::cef_base_ref_counted_t) {
                let slf = base as *mut Self;
                let count = (*slf)
                    .ref_count
                    .fetch_add(1, std::sync::atomic::Ordering::SeqCst);
                log::trace!("{}::add_ref, {} -> {}", stringify!(#name), count, count + 1);
            }

            pub unsafe extern "C" fn release(base: *mut cef_sys::cef_base_ref_counted_t) -> i32 {
                let slf = base as *mut Self;
                let count = (*slf)
                    .ref_count
                    .fetch_sub(1, std::sync::atomic::Ordering::SeqCst)
                    - 1;

                if count < 1 {
                    let slf: Box<Self> = Box::from_raw(slf as *mut Self);
                    log::trace!("{}::release {} -> 0 (dropping!)", stringify!(#name), count + 1);
                    drop(slf); // not needed, but just to be extra clear
                    1
                } else {
                    log::trace!("{}::release, {} -> {}", stringify!(#name), count + 1, count);
                    0
                }
            }

            pub unsafe extern "C" fn has_one_ref(base: *mut cef_sys::cef_base_ref_counted_t) -> i32 {
                let slf = base as *mut Self;
                let count = (*slf).ref_count.load(std::sync::atomic::Ordering::SeqCst);
                log::trace!("{}::has_one_ref ({} -> {:?})", stringify!(#name), count, count == 1);
                if count == 1 {
                    1
                } else {
                    0
                }
            }

            pub unsafe extern "C" fn has_at_least_one_ref(base: *mut cef_sys::cef_base_ref_counted_t) -> i32 {
                let slf = base as *mut Self;
                let count = (*slf).ref_count.load(std::sync::atomic::Ordering::SeqCst);
                log::trace!("{}::has_at_least_one_ref ({} -> {:?})", stringify!(#name), count, count >= 1);
                if count >= 1 {
                    1
                } else {
                    0
                }
            }
        }
    };
    gen.into()
}
