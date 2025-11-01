/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Function Clone Macro
//!
//! Generates Clone trait implementation for basic Function types
//!
//! Generates Clone implementation for Function structs that have `function`
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
//! impl_function_clone!(ArcFunction<T, R>);
//!
//! // For two type parameters with Rc
//! impl_function_clone!(RcFunction<T, R>);
//!
//! // For three type parameters - BiFunction
//! impl_function_clone!(ArcBiFunction<T, U, R>);
//!
//! // For three type parameters with Rc
//! impl_function_clone!(RcBiFunction<T, U, R>);
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Clone trait implementation for basic Function types
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete `impl Clone for $struct_name` block. It generates
/// Clone implementation for Function structs that have `function` and `name`
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
/// impl_function_clone!(ArcFunction<T, R>);
///
/// // For two type parameters with Rc
/// impl_function_clone!(RcFunction<T, R>);
///
/// // For three type parameters with Arc
/// impl_function_clone!(ArcBiFunction<T, U, R>);
///
/// // For three type parameters with Rc
/// impl_function_clone!(RcBiFunction<T, U, R>);
/// ```
macro_rules! impl_function_clone {
    // Two generic parameters - Function types
    ($struct_name:ident < $t:ident, $r:ident >) => {
        impl<$t, $r> Clone for $struct_name<$t, $r> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    name: self.name.clone(),
                }
            }
        }
    };
    // Three generic parameters - BiFunction types
    ($struct_name:ident < $t:ident, $u:ident, $r:ident >) => {
        impl<$t, $u, $r> Clone for $struct_name<$t, $u, $r> {
            fn clone(&self) -> Self {
                Self {
                    function: self.function.clone(),
                    name: self.name.clone(),
                }
            }
        }
    };
}

pub(crate) use impl_function_clone;
