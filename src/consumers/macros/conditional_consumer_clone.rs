/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Conditional Consumer Clone Macro
//!
//! Generates Clone trait implementation for Conditional Consumer types
//!
//! Generates Clone implementation for Conditional Consumer structs that have
//! `consumer` and `predicate` fields. Both fields are cloned using their
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
//! impl_conditional_consumer_clone!(ArcConditionalConsumer<T>);
//! impl_conditional_consumer_clone!(RcConditionalConsumer<T>);
//!
//! // For two type parameters
//! impl_conditional_consumer_clone!(ArcConditionalBiConsumer<T, U>);
//! impl_conditional_consumer_clone!(RcConditionalBiConsumer<T, U>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for Conditional Consumer types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. Generates
/// Clone implementation for Conditional Consumer structs that have `consumer`
/// and `predicate` fields. Both fields are cloned using their respective
/// Clone implementations.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$t` - Generic parameter list (one or two type parameters)
/// * `$u` - Generic parameter list (one or two type parameters)
///
/// # Examples
///
/// ```ignore
/// // For single type parameter
/// impl_conditional_consumer_clone!(ArcConditionalConsumer<T>);
/// impl_conditional_consumer_clone!(RcConditionalConsumer<T>);
///
/// // For two type parameters
/// impl_conditional_consumer_clone!(ArcConditionalBiConsumer<T, U>);
/// impl_conditional_consumer_clone!(RcConditionalBiConsumer<T, U>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_conditional_consumer_clone {
    // Single generic parameter - Consumer types
    ($struct_name:ident < $t:ident >) => {
        impl<$t> Clone for $struct_name<$t> {
            fn clone(&self) -> Self {
                Self {
                    consumer: self.consumer.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
    // Two generic parameters - BiConsumer types
    ($struct_name:ident < $t:ident, $u:ident >) => {
        impl<$t, $u> Clone for $struct_name<$t, $u> {
            fn clone(&self) -> Self {
                Self {
                    consumer: self.consumer.clone(),
                    predicate: self.predicate.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_conditional_consumer_clone;
