pub mod common;


use std::marker::PhantomData;

use vlm_macro_derive::VLM;
pub use vlm_macro::web::{VlmHost,VlmPort,VirtualEnv,EJS,HTML,VlmContentType,XML};



#[derive(VLM)]
pub struct Vlm<V, U> {
    _phantom: PhantomData<(U, V)>,
}

impl<V, U> Vlm<V, U> {
    pub fn new() -> Self {
        Self { _phantom: PhantomData }
    }
}






