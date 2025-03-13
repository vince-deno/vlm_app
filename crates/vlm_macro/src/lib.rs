use std::{path::PathBuf, sync::Arc};

use web::{VlmContentType, VlmHost, VlmPort};

pub mod cli;
pub mod common;
pub mod web;
pub mod errors;



#[derive(Debug)]
pub enum V {
    Go(PathBuf),
}


pub trait VLM<T> {
    fn vlm(
        &self,
        addr: (VlmHost, VlmPort),
        content: std::path::PathBuf,
        content_type: std::sync::Arc<Option<Vec<Arc<dyn VlmContentType>>>>,
        mode: Arc<Option<V>>,
    ) -> Result<Option<T>, Box<dyn ::std::error::Error>>;
    fn type_id(&self) -> ::std::any::TypeId;
}
