/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Conditional Predicate Clone Macro
//!
//! Generates Clone trait implementation for Conditional Predicate types
//!
//! Generates Clone implementation for Conditional Predicate structs that have
//! `predicate` and `condition` fields. Both fields are cloned using their
//! respective Clone implementations.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one or two type parameters)
//!
//! # Examples
//!
//! ```ignore
//! // For single type parameter
//! impl_conditional_predicate_clone!(ArcConditionalPredicate<T>);
//! impl_conditional_predicate_clone!(RcConditionalPredicate<T>);
//!
//! // For two type parameters
//! impl_conditional_predicate_clone!(ArcConditionalBiPredicate<T, U>);
//! impl_conditional_predicate_clone!(RcConditionalBiPredicate<T, U>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for Conditional Predicate types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. Generates
/// Clone implementation for Conditional Predicate structs that have `predicate`
/// and `condition` fields. Both fields are cloned using their respective
/// Clone implementations.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (one or two type parameters)
///
/// # Examples
///
/// ```ignore
/// // For single type parameter
/// impl_conditional_predicate_clone!(ArcConditionalPredicate<T>);
/// impl_conditional_predicate_clone!(RcConditionalPredicate<T>);
///
/// // For two type parameters
/// impl_conditional_predicate_clone!(ArcConditionalBiPredicate<T, U>);
/// impl_conditional_predicate_clone!(RcConditionalBiPredicate<T, U>);
/// ```
macro_rules! impl_conditional_predicate_clone {
    // Single generic parameter
    ($struct_name:ident < $t:ident >) => {
        impl<$t> Clone for $struct_name<$t> {
            fn clone(&self) -> Self {
                Self {
                    predicate: self.predicate.clone(),
                    condition: self.condition.clone(),
                }
            }
        }
    };
    // Two generic parameters
    ($struct_name:ident < $t:ident, $u:ident >) => {
        impl<$t, $u> Clone for $struct_name<$t, $u> {
            fn clone(&self) -> Self {
                Self {
                    predicate: self.predicate.clone(),
                    condition: self.condition.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_conditional_predicate_clone;
