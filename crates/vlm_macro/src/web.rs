use std::error::Error;

use web_1::{VlmContentType, EJS, HTML, XML};

pub mod web_1;
pub mod span;


// --- Trait definition for a Virtual Environment for a language ---
pub trait VirtualEnv {
    /// Returns the name of the language environment.
    fn name(&self) -> &str;
    
    /// Detects if the environment is “active” (i.e. already initialized).
    fn is_active(&self) -> Result<bool, Box<dyn Error>>;
    
    /// Automatically initialize or reuse the environment.
    /// (For Python, this is inherent; for others we simulate it.)
    fn init(&self) -> Result<(), Box<dyn Error>>;
    
    /// Execute a given code snippet within the environment.
    fn run_code(&self, code: &str) -> Result<(), Box<dyn Error>>;
}


// 

impl XML {
    pub fn new(content_type: &[u8], charset: &[u8]) -> Self {
        Self {
            content_type: content_type.to_vec(),
            charset: charset.to_vec(),
        }
    }
}

impl VlmContentType for XML {
    fn content_type_header(&self) -> &[u8] {
        &self.content_type
    }

    fn charset(&self) -> &[u8] {
        &self.charset
    }
}

// 

impl HTML {
    pub fn new(content_type: &[u8], charset: &[u8]) -> Self {
        Self {
            content_type: content_type.to_vec(),
            charset: charset.to_vec(),
        }
    }
}

impl VlmContentType for HTML {
    fn content_type_header(&self) -> &[u8] {
        &self.content_type
    }

    fn charset(&self) -> &[u8] {
        &self.charset
    }
}

// 


impl EJS {
    pub fn new(content_type: &[u8], charset: &[u8]) -> Self {
        Self {
            content_type: content_type.to_vec(),
            charset: charset.to_vec(),
        }
    }
}

impl VlmContentType for EJS {
    fn content_type_header(&self) -> &[u8] {
        &self.content_type
    }

    fn charset(&self) -> &[u8] {
        &self.charset
    }
}
