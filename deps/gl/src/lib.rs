//!
//! # Usage
//!
//! You can import the pointer style loader and type aliases like so:
//!
//! ~~~no_run
//! # #![allow(unused_imports)]
//! extern crate gl;
//! # fn main() {
//! // include the OpenGL type aliases
//! use gl::types::*;
//! # }
//! ~~~
//!
//! You can load the function pointers into their respective function pointers
//! using the `load_with` function. You must supply a loader function from your
//! context library, This is how it would look using [glfw-rs]
//! (https://github.com/PistonDevelopers/glfw-rs):
//!
//! ~~~ignore
//! // the supplied function must be of the type:
//! // `&fn(symbol: &'static str) -> *const std::os::raw::c_void`
//! gl::load_with(|s| glfw.get_proc_address(s));
//!
//! // loading a specific function pointer
//! gl::Viewport::load_with(|s| glfw.get_proc_address(s));
//! ~~~
//!
//! Calling a function that has not been loaded will result in a failure like:
//! `panic!("gl::Viewport was not loaded")`, which aviods a segfault. This feature
//! does not cause any run time overhead because the failing functions are
//! assigned only when `load_with` is called.
//!
//! ~~~no_run
//! # #![allow(path_statement)]
//! # extern crate gl;
//! # fn main() {
//! // accessing an enum
//! gl::TEXTURE_2D;
//!
//! // calling a function
//! unsafe { gl::DrawArrays(gl::TRIANGLES, 0, 3) };
//!
//! // functions that take pointers are unsafe
//! # let shader = 0;
//! # let c_str: *const i8 = std::ptr::null();
//! unsafe { gl::ShaderSource(shader, 1, &c_str, std::ptr::null()) };
//! # }
//! ~~~
//!
//! Each function pointer has an associated boolean value allowing you to
//! check if a function has been loaded at run time. The function accesses a
//! corresponding global boolean that is set when `load_with` is called, so there
//! shouldn't be much overhead.
//!
//! ~~~no_run
//! if gl::Viewport::is_loaded() {
//!     // do something...
//! }
//! ~~~
//!

#![crate_name = "gl"]
#![crate_type = "lib"]

include!(concat!(env!("OUT_DIR"), "/bindings.rs"));
