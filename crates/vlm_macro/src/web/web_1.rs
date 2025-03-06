pub trait VlmContentType: Send + Sync {
    fn content_type_header(&self) -> &[u8];
    fn charset(&self) -> &[u8];
}


// Define the EJS struct
pub struct EJS {
    pub(crate) content_type: Vec<u8>,
    pub(crate) charset: Vec<u8>,
}

// Define the HTML struct
pub struct HTML {
    pub(crate) content_type: Vec<u8>,
    pub(crate) charset: Vec<u8>,
}

// Define the XML struct
pub struct XML {
    pub(crate) content_type: Vec<u8>,
    pub(crate) charset: Vec<u8>,
}