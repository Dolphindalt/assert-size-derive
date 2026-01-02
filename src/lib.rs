//! Compile-time type size assertions.
//!
//! This crate provides the [`assert_size`] attribute macro for verifying that types
//! have the expected size in bytes at compile time.
//!
//! # Quick Start
//!
//! ```
//! use assert_size_derive::assert_size;
//!
//! #[assert_size(2)]
//! struct MyData {
//!     foo: u8,
//!     bar: u8,
//! }
//! ```
//!
//! If the size doesn't match, compilation will fail with a clear error message.
//!
//! # Use Cases
//!
//! - Catching unintended size changes from code refactoring
//! - Ensuring types meet specific memory layout requirements for FFI or serialization
//! - Documenting expected type sizes for performance-critical code
//! - Detecting platform-specific size variations

use proc_macro::TokenStream;
use quote::quote;
use syn::{
    DeriveInput, LitInt, Result, parse::{Parse, ParseStream}, parse_macro_input
};

struct AssertSizeAttributeArgs {
    desired_size_in_bytes: usize,
}

impl Parse for AssertSizeAttributeArgs {
    fn parse(input: ParseStream) -> Result<Self> {
        // Parse the input as a single integer literal
        let lit: LitInt = input.parse()?;

        // Use a base10 conversion to get the integer value
        let value: usize = lit.base10_parse()?;
        Ok(AssertSizeAttributeArgs { desired_size_in_bytes: value })
    }
}

/// A compile-time assertion that verifies a type has the expected size in bytes.
///
/// This attribute macro generates a compile-time check using `std::mem::size_of` to ensure
/// the annotated type has exactly the specified size. If the size doesn't match, compilation
/// will fail with a clear error message.
///
/// # Parameters
///
/// * A single integer literal representing the expected size in bytes
///
/// # Use Cases
///
/// - Catching unintended size changes from code refactoring
/// - Ensuring types meet specific memory layout requirements (e.g., for FFI or serialization)
/// - Documenting expected type sizes for performance-critical code
/// - Detecting platform-specific size variations
///
/// # How It Works
///
/// The macro generates a const assertion that compares the actual size (via `core::mem::size_of`)
/// with the expected size. The type definition itself is preserved unchanged. The assertion is
/// evaluated at compile time, so there is zero runtime overhead. Works in both `std` and `no_std`
/// environments.
///
/// # Examples
///
/// ```
/// use assert_size_derive::assert_size;
///
/// #[assert_size(2)]
/// struct MyData {
///     foo: u8,
///     bar: u8,
/// }
///
/// #[assert_size(16)]
/// enum MyEnum {
///     Variant1(u64),
///     Variant2(u32),
/// }
/// ```
///
/// ## Compile-Time Failure Example
///
/// ```compile_fail
/// use assert_size_derive::assert_size;
///
/// // This will fail to compile because the actual size is 2 bytes, not 1
/// #[assert_size(1)]
/// struct TooSmall {
///     a: u8,
///     b: u8,
/// }
/// ```
///
/// # Compatibility
///
/// Works with any type definition: structs, enums, and unions.
#[proc_macro_attribute]
pub fn assert_size(attr: TokenStream, item: TokenStream) -> TokenStream {
    let args = parse_macro_input!(attr as AssertSizeAttributeArgs);

    let input = parse_macro_input!(item as DeriveInput);

    let desired_size_in_bytes = args.desired_size_in_bytes;
    let type_name = &input.ident;

    let generated_test_code = quote! {
        #[allow(unknown_lints, clippy::eq_op)]
        const _: () = assert!(#desired_size_in_bytes == ::core::mem::size_of::<#type_name>());

        #input
    };

    generated_test_code.into()
}
