/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Function Identity Macro
//!
//! Generates identity method implementation for function types.
//!
//! This macro generates an `impl` block that implements the `identity()` method
//! for function types that have identical input and output types (T -> T).
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name (e.g., `BoxFunction`, `RcFunction`, `ArcFunction`)
//! * `$t:ident` - The generic type parameter name (usually `T`)
//!
//! # Usage
//!
//! ```ignore
//! impl_function_identity!(BoxFunction<T>);
//! impl_function_identity!(RcFunction<T>);
//! impl_function_identity!(ArcFunction<T>);
//! impl_function_identity!(BoxBiFunction<T, U>);
//! ```
//!
//! # Generated Implementation
//!
//! For single-parameter functions, the macro generates:
//!
//! ```ignore
//! impl<T> $struct_name<T, T>
//! where
//!     T: Clone,
//! {
//!     /// Creates an identity function
//!     ///
//!     /// # Examples
//!     ///
//!     /// ```rust
//!     /// use prism3_function::{$struct_name, Function};
//!     ///
//!     /// let identity = $struct_name::<i32, i32>::identity();
//!     /// assert_eq!(identity.apply(&42), 42);
//!     /// ```
//!     pub fn identity() -> $struct_name<T, T> {
//!         $struct_name::new(|x: &T| x.clone())
//!     }
//! }
//! ```
//!
//! For two-parameter functions, the macro generates:
//!
//! ```ignore
//! impl<T, U> $struct_name<T, U, T>
//! where
//!     T: Clone,
//! {
//!     /// Creates an identity function
//!     ///
//!     /// # Examples
//!     ///
//!     /// ```rust
//!     /// use prism3_function::{$struct_name, BiFunction};
//!     ///
//!     /// let identity = $struct_name::<i32, String, i32>::identity();
//!     /// assert_eq!(identity.apply(&42, &"test".to_string()), 42);
//!     /// ```
//!     pub fn identity() -> $struct_name<T, U, T> {
//!         $struct_name::new(|x: &T, _: &U| x.clone())
//!     }
//! }
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates identity method implementation for function types.
///
/// This macro generates an `impl` block that implements the `identity()` method
/// for function types that have identical input and output types (T -> T).
///
/// # Parameters
///
/// * `$struct_name<$input_type, $output_type>` - The struct name with two generic type parameters
///   - Both generic parameters must be the same type identifier (e.g., `BoxFunction<T, T>`)
///   - Note: The macro caller must ensure $input_type and $output_type are identical
///
/// # Usage
///
/// ```rust,ignore
/// impl_function_identity_method!(BoxFunction<T, T>);
/// impl_function_identity_method!(RcFunction<T, T>);
/// impl_function_identity_method!(ArcFunction<T, T>);
/// impl_function_identity_method!(BoxFunctionOnce<T, T>);
/// impl_function_identity_method!(BoxMutatingFunction<T, T>);
/// impl_function_identity_method!(BoxStatefulFunction<T, T>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_function_identity_method {
    ($struct_name:ident < $t:ident , $r:ident >) => {
        // Note: The caller must ensure $t and $r are the same identifier
        impl<$t> $struct_name<$t, $t>
        where
            $t: Clone + 'static,
        {
            /// Creates an identity function
            ///
            /// # Examples
            #[doc = concat!("/// ```rust\n/// use prism3_function::", stringify!($struct_name), ";\n///\n/// let identity = ", stringify!($struct_name), "::<i32, i32>::identity();\n/// assert_eq!(identity.apply(&42), 42);\n/// ```")]
            pub fn identity() -> $struct_name<$t, $t> {
                $struct_name::new(|x: &$t| x.clone())
            }
        }
    };

    // Special case for mutating functions that take &mut T
    ($struct_name:ident < $t:ident , $r:ident >, mutating) => {
        // Note: The caller must ensure $t and $r are the same identifier
        impl<$t> $struct_name<$t, $t>
        where
            $t: Clone + 'static,
        {
            /// Creates an identity function
            ///
            /// # Examples
            #[doc = concat!("/// ```rust\n/// use prism3_function::", stringify!($struct_name), ";\n///\n/// let mut identity = ", stringify!($struct_name), "::<i32, i32>::identity();\n/// let mut value = 42;\n/// assert_eq!(identity.apply(&mut value), 42);\n/// ```")]
            pub fn identity() -> $struct_name<$t, $t> {
                $struct_name::new(|x: &mut $t| x.clone())
            }
        }
    };
}

pub(crate) use impl_function_identity_method;
