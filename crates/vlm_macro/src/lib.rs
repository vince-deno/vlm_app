use core::error::Error;
use std::{env, path::PathBuf, sync::Arc};

use web::{span::VLMSpanUtils, web_1::VlmContentType};

pub mod web;
pub mod errors;

// Define architecture-specific constants
#[cfg(target_arch = "x86_64")]
const VLM_TAR: &str = "x86_64";
#[cfg(target_arch = "x86")]
const VLM_TAR: &str = "x86";
#[cfg(target_arch = "arm")]
const VLM_TAR: &str = "arm";
#[cfg(target_arch = "aarch64")]
const VLM_TAR: &str = "aarch64";
#[cfg(target_arch = "riscv64")]
const VLM_TAR: &str = "riscv64";
#[cfg(target_arch = "powerpc")]
const VLM_TAR: &str = "powerpc";
#[cfg(target_arch = "powerpc64")]
const VLM_TAR: &str = "powerpc64";
#[cfg(target_arch = "mips")]
const VLM_TAR: &str = "mips";
#[cfg(target_arch = "mips64")]
const VLM_TAR: &str = "mips64";
#[cfg(target_arch = "sparc")]
const VLM_TAR: &str = "sparc";
#[cfg(target_arch = "wasm32")]
const VLM_TAR: &str = "wasm32";

// Fallback for unsupported architectures.
#[cfg(not(any(
    target_arch = "x86_64",
    target_arch = "x86",
    target_arch = "arm",
    target_arch = "aarch64",
    target_arch = "riscv64",
    target_arch = "powerpc",
    target_arch = "powerpc64",
    target_arch = "mips",
    target_arch = "mips64",
    target_arch = "sparc",
    target_arch = "wasm32"
)))]
const VLM_TAR: &str = "unknown";

pub type VlmHost=[u8; 4];
pub type VlmPort=u16;
pub trait VLM {
    /// The main VLM function.
    fn vlm(&self, addr: (VlmHost, VlmPort),content:PathBuf,content_type:Vec<Arc<dyn VlmContentType + Send + Sync>>) -> Result<(), Box<dyn Error>>;

    /// Executes a function within the VLM context, returning a mutable reference
    /// to an object implementing VLMSpanUtils for the given function type.
    fn vlm_execute<F>(&mut self, func: F) -> Option<&mut dyn VLMSpanUtils<F>>
    where
        F: FnOnce(&mut Self);

    /// Returns the target architecture as a static string.
    fn os_type(&self) -> &'static str {
        VLM_TAR
    }

    /// Retrieves an environment variable value.
    fn env_var(key: &str) -> Option<String> {
        env::var(key).ok()
    }

    /// Example function taking a reference to an i32.
    fn foo<'a>(v: &'a i32);

    /// Example function with two parameters having independent lifetimes.
    fn vlmapp<'a, 'b>(l: &'a i32, m: &'b i32);

    /// Returns a compile-time environment variable if set.
    fn tests(&self) -> Option<&'static str>;

    /// Returns a Windows-specific build test message.
    #[cfg(target_os = "windows")]
    fn windows_build_test(&self) -> &'static str;
}
