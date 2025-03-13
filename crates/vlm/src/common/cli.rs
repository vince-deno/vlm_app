use vlm_macro::cli::cl::DefaultVLMCli;
pub use vlm_macro::common::tasks::VLMTaskExecutor;
pub use vlm_macro::common::tasks::VLMGenericTaskExecutor;
use vlm_macro::common::tasks::{DefaultVLMGenericTaskExecutor, DefaultVLMTaskExecutor};
pub use vlm_macro_derive::{VLMCli, VLMGenericTaskExecutor, VLMTaskExecutor};




#[derive(Default)]
pub struct Vlmcli;

// Implement the helper traits. This is all that’s needed.
impl DefaultVLMCli for Vlmcli {}
impl DefaultVLMTaskExecutor for Vlmcli {}
impl DefaultVLMGenericTaskExecutor for Vlmcli {}

