//! Concatenates byte literals
//! # Examples
//! ```
//! use proc_concat_bytes::concat_bytes;
//! let c_str = std::ffi::CStr::from_bytes_with_nul(&concat_bytes!(b"Hello World!", b'\0')[..]).unwrap();
//! ```
use proc_macro_hack::proc_macro_hack;

#[proc_macro_hack]
pub use proc_concat_bytes_impl::concat_bytes;
