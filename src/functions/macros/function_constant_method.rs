/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Function Constant Method Macro
//!
//! Generates constant method implementation for function types.
//!
//! This macro generates an `impl` block that implements the `constant()` method
//! for function types that return a constant value regardless of input.
//!
//! This macro supports both single-parameter and two-parameter functions through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxFunction<T, R>`
//!   - Two parameters: `BoxBiFunction<T, U, R>`
//!
//! # Usage
//!
//! ```ignore
//! // Single-parameter function
//! impl_function_constant_method!(BoxFunction<T, R>);
//!
//! // Two-parameter function
//! impl_function_constant_method!(BoxBiFunction<T, U, R>);
//! ```
//!
//! # Generated Implementation
//!
//! For single-parameter functions, the macro generates:
//!
//! ```ignore
//! impl<T, R> BoxFunction<T, R>
//! where
//!     R: Clone + 'static,
//! {
//!     /// Creates a constant function
//!     ///
//!     /// # Examples
//!     ///
//!     /// ```rust
//!     /// use prism3_function::{BoxFunction, Function};
//!     ///
//!     /// let constant = BoxFunction::constant("hello");
//!     /// assert_eq!(constant.apply(123), "hello");
//!     /// ```
//!     pub fn constant(value: R) -> BoxFunction<T, R> {
//!         BoxFunction::new(move |_| value.clone())
//!     }
//! }
//! ```
//!
//! For two-parameter functions, the macro generates:
//!
//! ```ignore
//! impl<T, U, R> BoxBiFunction<T, U, R>
//! where
//!     R: Clone + 'static,
//! {
//!     /// Creates a constant function
//!     ///
//!     /// # Examples
//!     ///
//!     /// ```rust
//!     /// use prism3_function::{BoxBiFunction, BiFunction};
//!     ///
//!     /// let constant = BoxBiFunction::constant("hello");
//!     /// assert_eq!(constant.apply(123, "test"), "hello");
//!     /// ```
//!     pub fn constant(value: R) -> BoxBiFunction<T, U, R> {
//!         BoxBiFunction::new(move |_, _| value.clone())
//!     }
//! }
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates constant method implementation for function types.
///
/// This macro generates an `impl` block that implements the `constant()` method
/// for function types that return a constant value regardless of input.
///
/// This macro supports both single-parameter and two-parameter functions through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxFunction<T, R>`
///   - Two parameters: `BoxBiFunction<T, U, R>`
///
/// # Usage
///
/// ```rust,ignore
/// // Single-parameter function
/// impl_function_constant_method!(BoxFunction<T, R>);
///
/// // Two-parameter function
/// impl_function_constant_method!(BoxBiFunction<T, U, R>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_function_constant_method {
    // Two generic parameters - Function
    ($struct_name:ident < $t:ident, $r:ident >, $($extra_bounds:tt)+) => {
        impl<$t, $r> $struct_name<$t, $r>
        where
            $t: 'static,
            $r: Clone + $($extra_bounds)+,
        {
            /// Creates a constant function
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use prism3_function::{", stringify!($struct_name), ", Function};\n///\n/// let constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $r> {
                $struct_name::new(move |_| value.clone())
            }
        }
    };

    // Three generic parameters - BiFunction (no extra bounds)
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: Clone + 'static,
        {
            /// Creates a constant function
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use prism3_function::{", stringify!($struct_name), ", BiFunction};\n///\n/// let constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123, \"test\"), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $u, $r> {
                $struct_name::new(move |_, _| value.clone())
            }
        }
    };

    // Three generic parameters - BiFunction
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >, $($extra_bounds:tt)+) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r>
        where
            $t: 'static,
            $u: 'static,
            $r: Clone + $($extra_bounds)+,
        {
            /// Creates a constant function
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use prism3_function::{", stringify!($struct_name), ", BiFunction};\n///\n/// let constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123, \"test\"), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $u, $r> {
                $struct_name::new(move |_, _| value.clone())
            }
        }
    };
}

pub(crate) use impl_function_constant_method;
