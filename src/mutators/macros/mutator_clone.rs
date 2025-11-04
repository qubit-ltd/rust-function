/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Mutator Clone Macro
//!
//! Generates Clone trait implementation for basic Mutator types
//!
//! Generates Clone implementation for Mutator structs that have `function`
//! and `name` fields. The function field is cloned using its inherent `clone`
//! method, which performs a shallow clone for smart pointers like `Arc` or `Rc`.
//!
//! # Parameters
//!
//! * `$struct_name` - The struct name
//! * `$generic` - Generic parameter list (one type parameter)
//!
//! # Examples
//!
//! ```ignore
//! // For single type parameter
//! impl_mutator_clone!(ArcMutator<T>);
//!
//! // For single type parameter with Rc
//! impl_mutator_clone!(RcMutator<T>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for basic Mutator types
///
/// This macro should be used at the top level (outside of any impl block)
/// to generate Clone trait implementations for Mutator structs. It generates
/// Clone implementation for Mutator structs that have `function` and `name`
/// fields. The function field is cloned using its inherent `clone` method,
/// which performs a shallow clone for smart pointers like `Arc` or `Rc`.
///
/// # Parameters
///
/// * `$struct_name` - The struct name
/// * `$generic` - Generic parameter list (one type parameter)
///
/// # Usage Location
///
/// This macro should be used at the top level, outside of any impl block,
/// typically in the same file as the struct definition.
///
/// # Examples
///
/// ```ignore
/// // At the top level, outside of any impl block
/// impl_mutator_clone!(ArcMutator<T>);
///
/// // For single type parameter with Rc
/// impl_mutator_clone!(RcMutator<T>);
/// ```
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_mutator_clone {
    // Single generic parameter
    ($struct_name:ident < $generic:ident >) => {
        impl<$generic> Clone for $struct_name<$generic> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    name: self.name.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_mutator_clone;
