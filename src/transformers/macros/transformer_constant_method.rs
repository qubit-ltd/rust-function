/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Transformer Constant Method Macro
//!
//! Generates constant method implementation for transformer types.
//!
//! This macro generates an `impl` block that implements the `constant()` method
//! for transformer types that return a constant value regardless of input.
//!
//! This macro supports various transformer types through pattern matching on
//! the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxTransformer<T, R>`
//!   - Two parameters: `BoxBiTransformer<T, U, R>`
//!   - Stateful variants: `BoxStatefulTransformer<T, R>`
//!
//! # Usage
//!
//! ```ignore
//! // Single-parameter transformer
//! impl_transformer_constant_method!(BoxTransformer<T, R>);
//!
//! // Two-parameter transformer
//! impl_transformer_constant_method!(BoxBiTransformer<T, U, R>);
//!
//! // Stateful transformer
//! impl_transformer_constant_method!(BoxStatefulTransformer<T, R>);
//! ```
//!
//! # Generated Implementation
//!
//! For single-parameter transformers, the macro generates:
//!
//! ```ignore
//! impl<T, R> BoxTransformer<T, R>
//! where
//!     R: Clone + 'static,
//! {
//!     /// Creates a constant transformer
//!     ///
//!     /// # Examples
//!     ///
//!     /// ```rust
//!     /// use qubit_function::{BoxTransformer, Transformer};
//!     ///
//!     /// let constant = BoxTransformer::constant("hello");
//!     /// assert_eq!(constant.apply(123), "hello");
//!     /// ```
//!     pub fn constant(value: R) -> BoxTransformer<T, R> {
//!         BoxTransformer::new(move |_| value.clone())
//!     }
//! }
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates constant method implementation for transformer types.
///
/// This macro generates an `impl` block that implements the `constant()` method
/// for transformer types that return a constant value regardless of input.
///
/// This macro supports various transformer types through pattern matching on
/// the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxTransformer<T, R>`
///   - Two parameters: `BoxBiTransformer<T, U, R>`
///   - Stateful variants: `BoxStatefulTransformer<T, R>`
///
/// # Usage
///
/// ```rust,ignore
/// // Single-parameter transformer
/// impl_transformer_constant_method!(BoxTransformer<T, R>);
///
/// // Two-parameter transformer
/// impl_transformer_constant_method!(BoxBiTransformer<T, U, R>);
///
/// // Stateful transformer
/// impl_transformer_constant_method!(BoxStatefulTransformer<T, R>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_transformer_constant_method {
    // Single-parameter transformer (BoxTransformer, RcTransformer, BoxTransformerOnce)
    ($struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> $struct_name<$t, $r> {
            /// Creates a constant transformer
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use qubit_function::{", stringify!($struct_name), ", Transformer};\n///\n/// let constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $r>
            where
                $t: 'static,
                $r: Clone + 'static,
            {
                $struct_name::new(move |_| value.clone())
            }
        }
    };

    // Thread-safe single-parameter transformer (ArcTransformer)
    (thread_safe $struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> $struct_name<$t, $r> {
            /// Creates a constant transformer
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use qubit_function::{", stringify!($struct_name), ", Transformer};\n///\n/// let constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $r>
            where
                $t: Send + Sync + 'static,
                $r: Clone + Send + Sync + 'static,
            {
                $struct_name::new(move |_| value.clone())
            }
        }
    };

    // Two-parameter transformer (BoxBiTransformer, RcBiTransformer, BoxBiTransformerOnce)
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r> {
            /// Creates a constant bi-transformer
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use qubit_function::{", stringify!($struct_name), ", BiTransformer};\n///\n/// let constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123, 456), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $u, $r>
            where
                $t: 'static,
                $u: 'static,
                $r: Clone + 'static,
            {
                $struct_name::new(move |_, _| value.clone())
            }
        }
    };

    // Thread-safe two-parameter transformer (ArcBiTransformer)
    (thread_safe $struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r> {
            /// Creates a constant bi-transformer
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use qubit_function::{", stringify!($struct_name), ", BiTransformer};\n///\n/// let constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123, 456), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $u, $r>
            where
                $t: Send + Sync + 'static,
                $u: Send + Sync + 'static,
                $r: Clone + Send + Sync + 'static,
            {
                $struct_name::new(move |_, _| value.clone())
            }
        }
    };

    // Stateful transformer (BoxStatefulTransformer, RcStatefulTransformer)
    (stateful $struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> $struct_name<$t, $r> {
            /// Creates a constant transformer
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use qubit_function::{", stringify!($struct_name), ", StatefulTransformer};\n///\n/// let mut constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $r>
            where
                $t: 'static,
                $r: Clone + 'static,
            {
                $struct_name::new(move |_| value.clone())
            }
        }
    };

    // Thread-safe stateful transformer (ArcStatefulTransformer)
    (stateful thread_safe $struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> $struct_name<$t, $r> {
            /// Creates a constant transformer
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use qubit_function::{", stringify!($struct_name), ", StatefulTransformer};\n///\n/// let mut constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $r>
            where
                $t: Send + Sync + 'static,
                $r: Clone + Send + 'static,
            {
                $struct_name::new(move |_| value.clone())
            }
        }
    };

    // Stateful bi-transformer (BoxStatefulBiTransformer, RcStatefulBiTransformer)
    (stateful $struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r> {
            /// Creates a constant bi-transformer
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use qubit_function::{", stringify!($struct_name), ", StatefulBiTransformer};\n///\n/// let constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123, 456), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $u, $r>
            where
                $t: 'static,
                $u: 'static,
                $r: Clone + 'static,
            {
                $struct_name::new(move |_, _| value.clone())
            }
        }
    };

    // Thread-safe stateful bi-transformer (ArcStatefulBiTransformer)
    (stateful thread_safe $struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> $struct_name<$t, $u, $r> {
            /// Creates a constant bi-transformer
            ///
            /// # Examples
            ///
            #[doc = concat!("/// ```rust\n/// use qubit_function::{", stringify!($struct_name), ", StatefulBiTransformer};\n///\n/// let constant = ", stringify!($struct_name), "::constant(\"hello\");\n/// assert_eq!(constant.apply(123, 456), \"hello\");\n/// ```")]
            pub fn constant(value: $r) -> $struct_name<$t, $u, $r>
            where
                $t: Send + Sync + 'static,
                $u: Send + Sync + 'static,
                $r: Clone + Send + 'static,
            {
                $struct_name::new(move |_, _| value.clone())
            }
        }
    };
}

pub(crate) use impl_transformer_constant_method;
