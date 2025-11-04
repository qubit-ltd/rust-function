/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Transformer Clone Macro
//!
//! Generates Clone trait implementation for basic Transformer types
//!
//! Generates Clone implementation for Transformer structs that have `function`
//! and `name` fields. The function field is cloned using its inherent `clone`
//! method, which performs a shallow clone for smart pointers like `Arc` or `Rc`.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (two or three type parameters)
//!
//! # Examples
//!
//! ```ignore
//! // For two type parameters
//! impl_transformer_clone!(ArcTransformer<T, U>);
//!
//! // For two type parameters with Rc
//! impl_transformer_clone!(RcTransformer<T, U>);
//!
//! // For three type parameters
//! impl_transformer_clone!(ArcBiTransformer<T, U, V>);
//!
//! // For three type parameters with Rc
//! impl_transformer_clone!(RcBiTransformer<T, U, V>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for basic Transformer types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. It generates
/// Clone implementation for Transformer structs that have `function` and `name`
/// fields. The function field is cloned using its inherent `clone` method,
/// which performs a shallow clone for smart pointers like `Arc` or `Rc`.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (two or three type parameters)
///
/// # Examples
///
/// ```ignore
/// // For two type parameters with Arc
/// impl_transformer_clone!(ArcTransformer<T, U>);
///
/// // For two type parameters with Rc
/// impl_transformer_clone!(RcTransformer<T, U>);
///
/// // For three type parameters with Arc
/// impl_transformer_clone!(ArcBiTransformer<T, U, V>);
///
/// // For three type parameters with Rc
/// impl_transformer_clone!(RcBiTransformer<T, U, V>);
// ```
macro_rules! impl_transformer_clone {
    // Two generic parameters
    ($struct_name:ident < $generic1:ident, $generic2:ident >) => {
        impl<$generic1, $generic2> Clone for $struct_name<$generic1, $generic2> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    name: self.name.clone(),
                }
            }
        }
    };
    // Three generic parameters
    ($struct_name:ident < $generic1:ident, $generic2:ident, $generic3:ident >) => {
        impl<$generic1, $generic2, $generic3> Clone for $struct_name<$generic1, $generic2, $generic3> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    name: self.name.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_transformer_clone;
