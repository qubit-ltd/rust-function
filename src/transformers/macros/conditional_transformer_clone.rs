/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Transformer Clone Macro
//!
//! Generates Clone trait implementation for Conditional Transformer types
//!
//! Generates Clone implementation for Conditional Transformer structs that have
//! `transformer` and `predicate` fields. Both fields are cloned using their
//! respective Clone implementations.
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
//! impl_conditional_transformer_clone!(ArcConditionalTransformer<T, U>);
//! impl_conditional_transformer_clone!(RcConditionalTransformer<T, U>);
//!
//! // For three type parameters
//! impl_conditional_transformer_clone!(ArcConditionalBiTransformer<T, U, V>);
//! impl_conditional_transformer_clone!(RcConditionalBiTransformer<T, U, V>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for Conditional Transformer types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. Generates
/// Clone implementation for Conditional Transformer structs that have `transformer`
/// and `predicate` fields. Both fields are cloned using their respective
/// Clone implementations.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (two or three type parameters)
///
/// # Examples
///
/// ```ignore
/// // For two type parameters
/// impl_conditional_transformer_clone!(ArcConditionalTransformer<T, U>);
/// impl_conditional_transformer_clone!(RcConditionalTransformer<T, U>);
///
/// // For three type parameters
/// impl_conditional_transformer_clone!(ArcConditionalBiTransformer<T, U, V>);
/// impl_conditional_transformer_clone!(RcConditionalBiTransformer<T, U, V>);
// ```
macro_rules! impl_conditional_transformer_clone {
    // Two generic parameters
    ($struct_name:ident < $generic1:ident, $generic2:ident >) => {
        impl<$generic1, $generic2> Clone for $struct_name<$generic1, $generic2> {
            fn clone(&self) -> Self {
                Self {
                    transformer: self.transformer.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
    // Three generic parameters
    ($struct_name:ident < $generic1:ident, $generic2:ident, $generic3:ident >) => {
        impl<$generic1, $generic2, $generic3> Clone for $struct_name<$generic1, $generic2, $generic3> {
            fn clone(&self) -> Self {
                Self {
                    transformer: self.transformer.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_conditional_transformer_clone;
