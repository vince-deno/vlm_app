use vlm_macro::common::span::VLMSpanUtils;
use vlm_macro::common::span::VLMSpanCore;
use vlm_macro_derive::VLMSpanUtils;
use vlm_macro_derive::VLMSpanCore;

/// Span struct representing a range
#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, VLMSpanCore, VLMSpanUtils)]
pub struct Vlmspan<T> {
	pub lo: T,
	pub hi: T,
}
