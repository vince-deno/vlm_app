use proc_macro::TokenStream;
use quote::quote;
use syn::{
    parse_macro_input, DeriveInput, GenericParam, Lit, Meta,
    punctuated::Punctuated,
    Token,
};
use std::collections::HashMap;

/// Extension trait for syn::Meta to extract a name–value pair.
trait MetaExt {
    fn as_name_value_pair(&self) -> Option<(String, Lit)>;
}

impl MetaExt for Meta {
    fn as_name_value_pair(&self) -> Option<(String, Lit)> {
        if let Meta::NameValue(nv) = self {
            nv.path.get_ident().and_then(|id| {
                if let syn::Expr::Lit(expr_lit) = &nv.value {
                    Some((id.to_string(), expr_lit.lit.clone()))
                } else {
                    None
                }
            })
        } else {
            None
        }
    }
}

/// Derive macro for implementing the generic `VLM<X>` trait and the `VLMDefined` trait.
#[proc_macro_derive(VLM, attributes(VLMDefined))]
pub fn vlm_macro_derive(input: TokenStream) -> TokenStream {
    let ast = parse_macro_input!(input as DeriveInput);
    let name = &ast.ident;
    let generics = ast.generics;
    // Split generics into (impl_generics, ty_generics, where_clause)
    let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

    // Extract the second type parameter (assumed to be the one for the trait)
    let trait_param = match generics.params.iter().nth(1) {
        Some(GenericParam::Type(ty)) => ty.ident.clone(),
        _ => panic!("Expected at least two type parameters, e.g. Vlm<V, T>"),
    };

    // --- Begin attribute extraction ---
    let mut opts: HashMap<String, Lit> = HashMap::new();
    // Look for attributes like: #[vlm(A = 86, L = 76, M = 77, vlm = 12345, as = "MyAlias")]
    for attr in ast.attrs.iter().filter(|attr| attr.path().is_ident("vlm")) {
        let metas: Punctuated<Meta, Token![,]> =
            match attr.parse_args_with(Punctuated::<Meta, Token![,]>::parse_terminated) {
                Ok(m) => m,
                Err(_) => continue,
            };
        metas.iter()
            .filter_map(|meta| meta.as_name_value_pair())
            .for_each(|(k, v)| { opts.insert(k, v); });
    }
    // Helper closure for i32 extraction.
    let extract_i32 = |key: &str, default: i32| -> i32 {
        opts.get(key)
            .map(|lit| {
                if let Lit::Int(lit_int) = lit {
                    lit_int.base10_parse::<i32>().unwrap()
                } else {
                    default
                }
            })
            .unwrap_or(default)
    };

    let a_val: i32 = extract_i32("A", 86); // Default: ASCII 'V'
    let l_val: i32 = extract_i32("L", 76); // Default: ASCII 'L'
    let m_val: i32 = extract_i32("M", 77); // Default: ASCII 'M'

    let vlm_val: i64 = opts.get("vlm")
        .map(|lit| {
            if let Lit::Int(lit_int) = lit {
                lit_int.base10_parse::<i64>().unwrap()
            } else {
                0
            }
        })
        .unwrap_or(((a_val as i64) << 16) | ((l_val as i64) << 8) | (m_val as i64));

    let alias = opts.get("as")
        .and_then(|lit| {
            if let Lit::Str(s) = lit { Some(s.value()) } else { None }
        })
        .unwrap_or_else(|| name.to_string());
    // --- End attribute extraction ---

    let expanded = quote! {
        // IMPLEMENTATION FOR THE VLM TRAIT.
        // This generates: impl<V, T> ::vlm_macro::VLM<T> for Vlm<V, T> { ... }
        impl #impl_generics ::vlm_macro::VLM<#trait_param> for #name #ty_generics #where_clause
        where
            Self: 'static,
        {
            fn vlm(
                &self,
                addr: (::vlm_macro::web::VlmHost, ::vlm_macro::web::VlmPort),
                content: ::std::path::PathBuf,
                content_type: ::std::sync::Arc<Option<Vec<::std::sync::Arc<dyn ::vlm_macro::web::VlmContentType>>>>,
                mode: ::std::sync::Arc<Option<::vlm_macro::V>>,
            ) -> Result<Option<#trait_param>, Box<dyn ::std::error::Error>> {
                Self::start_server(addr, content, content_type, mode)?;
                Ok(None)
            }

            fn type_id(&self) -> ::std::any::TypeId {
                ::std::any::TypeId::of::<Self>()
            }
        }

        // Inherent implementations for the type.
        impl #impl_generics #name #ty_generics #where_clause
        where
            Self: 'static,
        {
            pub fn start_server(
                addr: (::vlm_macro::web::VlmHost, ::vlm_macro::web::VlmPort),
                content_path: ::std::path::PathBuf,
                content_types: ::std::sync::Arc<Option<Vec<::std::sync::Arc<dyn ::vlm_macro::web::VlmContentType>>>>,
                mode: ::std::sync::Arc<Option<::vlm_macro::V>>,
            ) -> Result<(), Box<dyn ::std::error::Error>> {
                use ::std::fs;
                use warp::Filter;
                use ::std::net::{Ipv4Addr, SocketAddr};
                use warp::http::Response;
                use warp::hyper::Body;
                use tokio::sync::Mutex;
                use ::std::sync::Arc;

                let final_addr = Self::transform_address(addr, &mode);
                let content = Arc::new(Mutex::new(fs::read_to_string(&content_path)?));

                let route = warp::path::end()
                    .and(warp::header::optional("Accept"))
                    .and_then({
                        let content = Arc::clone(&content);
                        let content_types = Arc::clone(&content_types);
                        move |accept_header: Option<String>| {
                            let content = Arc::clone(&content);
                            let content_types = Arc::clone(&content_types);
                            async move {
                                let content_val = content.lock().await.clone();
                                let chosen_type = if let Some(ref types) = *content_types {
                                    types.iter().find(|ct| {
                                        accept_header.as_ref().map_or(false, |accept| {
                                            let header = String::from_utf8_lossy(ct.content_type_header());
                                            header.contains(accept)
                                        })
                                    }).cloned().unwrap_or_else(|| types.first().cloned().expect("No content types provided"))
                                } else {
                                    panic!("No content types provided");
                                };

                                let response = Response::builder()
                                    .header(
                                        "Content-Type",
                                        format!("{}; charset={}",
                                            String::from_utf8_lossy(chosen_type.content_type_header()),
                                            String::from_utf8_lossy(chosen_type.charset())
                                        )
                                    )
                                    .body(Body::from(content_val))
                                    .unwrap();
                                Ok::<_, warp::Rejection>(response)
                            }
                        }
                    });

                let socket = SocketAddr::from((Ipv4Addr::from(final_addr.0), final_addr.1));

                if tokio::runtime::Handle::try_current().is_ok() {
                    ::std::thread::spawn(move || {
                        let rt = tokio::runtime::Builder::new_multi_thread()
                            .worker_threads(4)
                            .enable_all()
                            .build()
                            .expect("Failed to create Tokio runtime");
                        rt.block_on(async move {
                            warp::serve(route).run(socket).await;
                        });
                    });
                    Ok(())
                } else {
                    let rt = tokio::runtime::Builder::new_multi_thread()
                        .worker_threads(4)
                        .enable_all()
                        .build()
                        .map_err(|e| format!("Failed to create Tokio runtime: {}", e))?;
                    rt.block_on(async move {
                        warp::serve(route).run(socket).await;
                    });
                    Ok(())
                }
            }

            fn transform_address(
                addr: (::vlm_macro::web::VlmHost, ::vlm_macro::web::VlmPort),
                mode: &::std::sync::Arc<Option<::vlm_macro::V>>
            ) -> (::vlm_macro::web::VlmHost, ::vlm_macro::web::VlmPort) {
                if let Some(_) = &**mode {
                    (addr.0, 9090)
                } else {
                    addr
                }
            }

            // Private inherent method providing a safe, memoized transformation.
            fn __safe_vlm_h_hex_asccii_simd<X>(vlm: X) -> Option<X>
            where
                X: 'static + std::any::Any + Clone + From<[i32; 4]>,
            {
                use once_cell::sync::Lazy;
                use std::sync::Mutex;
                use std::collections::HashMap;

                fn transform_value(x: i32) -> i32 {
                    static MEMO: Lazy<Mutex<HashMap<i32, i32>>> =
                        Lazy::new(|| Mutex::new(HashMap::new()));
                    let mut cache = MEMO.lock().unwrap();
                    if let Some(&cached) = cache.get(&x) {
                        cached
                    } else {
                        let result = x.pow(3) - 2 * x.pow(2) + x + 13;
                        cache.insert(x, result);
                        result
                    }
                }

                let any_val = &vlm as &dyn std::any::Any;
                if let Some(arr) = any_val.downcast_ref::<[i32; 4]>() {
                    let result_array = [
                        transform_value(arr[0]),
                        transform_value(arr[1]),
                        transform_value(arr[2]),
                        transform_value(arr[3]),
                    ];
                    Some(X::from(result_array))
                } else {
                    None
                }
            }
        }

        // IMPLEMENTATION FOR THE VLMDefined TRAIT.
        impl #impl_generics ::vlm_macro::common::vlm::VLMDefined for #name #ty_generics #where_clause
        where
            Self: 'static,
        {
            const V: i32 = #a_val;
            const L: i32 = #l_val;
            const M: i32 = #m_val;
            const VLM: i64 = #vlm_val;

            fn return_ascii(v: i32, l: i32, m: i32) -> [u32; 3] {
                [v as u32, l as u32, m as u32]
            }

            fn ascii_to_string(v: i32, l: i32, m: i32) -> Vec<String> {
                vec![
                    char::from_u32(v as u32).unwrap_or('?').to_string(),
                    char::from_u32(l as u32).unwrap_or('?').to_string(),
                    char::from_u32(m as u32).unwrap_or('?').to_string(),
                ]
            }

            fn ascii_to_hex(vlm: i64) -> isize {
                vlm as isize
            }

            fn hex_to_ascii(vlm: i64) -> isize {
                vlm as isize
            }

            fn hybrid_ascii_hex(m: i64) {
                let ascii = {
                    let bytes = [Self::V as u8, Self::L as u8, Self::M as u8];
                    String::from_utf8_lossy(&bytes).into_owned()
                };
                let result = [
                    (m as i32).wrapping_add(Self::V),
                    (m as i32).wrapping_sub(Self::L),
                    (m as i32).wrapping_mul(Self::M),
                    (m as i32) ^ Self::V,
                ];
                println!("Hybrid ({}): Advanced Result = {:#X?}", ascii, result);
            }

            fn vlm_h_hex_asccii_simd<X>(vlm: X) -> Option<X>
            where
                X: 'static + std::any::Any + Clone + From<[i32; 4]>,
            {
                Self::__safe_vlm_h_hex_asccii_simd(vlm)
            }

            fn get_alias() -> String {
                #alias.to_string()
            }
        }
    };

    expanded.into()
}


#[proc_macro_derive(VLMSpanCore)]
pub fn derive_vlm_span_core(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl<T> VLMSpanCore<T> for #name<T>
        where
            T: Ord + Copy + std::ops::Sub<Output = T>,  // ✅ Add `Sub<Output = T>`
        {
            fn new(lo: T, hi: T) -> Self {
                Self { lo: lo.min(hi), hi: lo.max(hi) }
            }

            fn range(&self) -> std::ops::Range<T> {
                self.lo..self.hi
            }

            fn len(&self) -> T {
                self.hi - self.lo  // ✅ Now `T` supports subtraction
            }

            fn is_empty(&self) -> bool {
                self.lo >= self.hi
            }

            fn overlaps(&self, other: &Self) -> bool {
                self.lo < other.hi && self.hi > other.lo
            }

            fn intersection(&self, other: &Self) -> Option<Self> {
                let lo = self.lo.max(other.lo);
                let hi = self.hi.min(other.hi);
                if lo < hi {
                    Some(Self::new(lo, hi))
                } else {
                    None
                }
            }

            fn union(&self, other: &Self) -> Self {
                Self::new(self.lo.min(other.lo), self.hi.max(other.hi))
            }

            fn split_at(&self, position: T) -> (Self, Self) {
                let pos = position.max(self.lo).min(self.hi);
                (
                    Self::new(self.lo, pos),
                    Self::new(pos, self.hi),
                )
            }
        }
    };

    TokenStream::from(expanded)
}

#[proc_macro_derive(VLMSpanUtils)]
pub fn derive_vlm_span_utils(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = input.ident;

    let expanded = quote! {
        impl<T> VLMSpanUtils<T> for #name<T>
        where
            T: Ord + Copy + std::fmt::Debug,
        {
            fn print(&self) {
                println!("{:?}", self);
            }
        }
    };

    TokenStream::from(expanded)
}



#[proc_macro_derive(VLMCli, attributes(DefaultVLMTaskExecutor, DefaultVLMGenericTaskExecutor, DefaultVLMCli))]
pub fn vlmcli_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        // Automatically implement the helper trait DefaultVLMCli
        impl DefaultVLMCli for #name {}

        impl VLMCli for #name
        where
            #name: Default + DefaultVLMCli,
        {
            fn run(&self) -> Result<(), ::std::fmt::Error> {
                self.run_default()
            }
            fn parse_args(&self) -> Result<(), ::std::fmt::Error> {
                self.parse_args_default()
            }
            fn help(&self) {
                self.help_default()
            }
            fn pattern(pattern: String) -> Self {
                Self::pattern_default(pattern)
            }
            fn file(file: ::std::path::PathBuf) -> Self {
                Self::file_default(file)
            }
            fn count(&self, count: usize) {
                let _ = Self::default().count_default(count);
            }
            fn path(&self, path: ::std::path::PathBuf) {
                let _ = Self::default().path_default(path);
            }
            fn json(&self, json: bool) {
                let _ = Self::default().json_default(json);
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(VLMTaskExecutor, attributes(DefaultVLMTaskExecutor))]
pub fn vlm_task_executor_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        // Automatically implement the helper trait DefaultVLMTaskExecutor
        impl DefaultVLMTaskExecutor for #name {}

        impl VLMTaskExecutor for #name
        where
            #name: DefaultVLMTaskExecutor,
        {
            fn execute_simple_task(&self) -> Result<(), String> {
                self.execute_simple_task_default()
            }
        }
    };
    TokenStream::from(expanded)
}

#[proc_macro_derive(VLMGenericTaskExecutor, attributes(DefaultVLMGenericTaskExecutor))]
pub fn vlm_generic_task_executor_derive(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as DeriveInput);
    let name = &input.ident;
    
    let expanded = quote! {
        // Automatically implement the helper trait DefaultVLMGenericTaskExecutor
        impl DefaultVLMGenericTaskExecutor for #name {}

        impl VLMGenericTaskExecutor for #name
        where
            #name: DefaultVLMGenericTaskExecutor + VLMTaskExecutor,
        {
            fn execute_task<F, U>(&self, task: F) -> Result<U, String>
            where F: FnOnce() -> U {
                self.execute_task_default(task)
            }
        }
    };
    TokenStream::from(expanded)
}
