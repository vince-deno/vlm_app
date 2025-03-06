extern crate proc_macro;

use proc_macro::TokenStream;
use quote::quote;
use syn::{parse_macro_input, DeriveInput};

#[proc_macro_derive(VLM)]
pub fn vlm_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    impl_vlm(&ast)
}

fn impl_vlm(ast: &syn::DeriveInput) -> TokenStream {
    let name = &ast.ident;
    let genz = quote! {
        impl VLM for #name {
            fn vlm(
                &self,
                addr: (VlmHost, VlmPort),
                content: std::path::PathBuf,
                content_type: Vec<std::sync::Arc<dyn VlmContentType + Send + Sync>>
            ) -> Result<(), Box<dyn std::error::Error>> {
                Self::start_server(addr, content, content_type)
            }

            fn vlm_execute<F>(&mut self, func: F) -> Option<&mut dyn VLMSpanUtils<F>>
            where F: FnOnce(&mut Self)
            {
                func(self);
                None
            }

            fn foo<'a>(v: &'a i32) {
                println!("foo: {}", v);
            }

            fn vlmapp<'a, 'b>(l: &'a i32, m: &'b i32) {
                println!("vlmapp: {} and {}", l, m);
            }

            fn tests(&self) -> Option<&'static str> {
                Some("Test passed")
            }

            #[cfg(target_os = "windows")]
            fn windows_build_test(&self) -> &'static str {
                "Windows build test message"
            }
        }

        impl #name {
            pub fn start_server(
                addr: (VlmHost, VlmPort),
                content_path: std::path::PathBuf,
                content_types: Vec<std::sync::Arc<dyn VlmContentType + Send + Sync>>
            ) -> Result<(), Box<dyn std::error::Error>> {
                use std::fs;
                use warp::Filter;
                use std::net::{Ipv4Addr, SocketAddr};
                use warp::http::Response;
                use warp::hyper::Body;

                // Read file content from the provided path.
                let content = fs::read_to_string(&content_path)?;

                let route = warp::path::end().and(warp::header::optional("Accept")).map({
                    let content = content.clone();
                    let content_types = content_types.clone();
                    move |accept_header: Option<String>| {
                        let content_type = content_types.iter().find(|ct| {
                            accept_header.as_ref().map_or(false, |accept| {
                                let header = String::from_utf8_lossy(ct.content_type_header());
                                header.contains(&accept[..])
                            })
                        }).unwrap_or(&content_types[0]);

                        Response::builder()
                            .header("Content-Type", format!("{}; charset={}",
                                String::from_utf8_lossy(content_type.content_type_header()),
                                String::from_utf8_lossy(content_type.charset())))
                            .body(Body::from(content.clone()))
                            .unwrap()
                    }
                });

                // Build a Tokio runtime using the current-thread builder.
                let rt = tokio::runtime::Builder::new_current_thread()
                    .enable_all()
                    .build()
                    .map_err(|e| format!("Failed to create Tokio runtime: {}", e))?;
                let (host, port) = addr;
                let socket = SocketAddr::from((Ipv4Addr::from(host), port));
                rt.block_on(async move {
                    warp::serve(route).run(socket).await;
                });
                Ok(())
            }
        }
    };
    genz.into()
}
