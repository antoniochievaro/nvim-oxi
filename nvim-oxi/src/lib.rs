//! # First-class bindings to Neovim internals
//!
//! The `nvim-oxi` crate ...

pub mod api;
mod error;
#[doc(hidden)]
pub mod lua;
mod macros;
pub mod object;
mod toplevel;

pub use error::{Error, Result};
pub use lua::{LuaFn, LuaFnMut, LuaFnOnce};
pub use nvim_types::{Object, String};
pub use oxi_module::oxi_module as module;
pub use toplevel::{nprint as print, *};
