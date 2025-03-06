use std::{path::PathBuf, sync::Arc};

use vlm::{Vlm, VlmContentType, VlmHost, VlmPort, HTML};



fn main(){
    let addr: (VlmHost, VlmPort) = ([127, 0, 0, 1], 3030);
    
    // Specify the path to your content (for example, an HTML file)
    let content_path = PathBuf::from("index.html");
    
    // Create a vector of content types.
    let content_types: Vec<Arc<dyn VlmContentType + Send + Sync>> = vec![
        Arc::new(HTML::new(b"text/html", b"utf-8")),
    ];
    
    // Call the generated VLM method to start the server.
    println!("Server started successfully");
    if let Err(e) = Vlm::start_server(addr, content_path, content_types) {
        eprintln!("Error starting server: {}", e);
    }
}