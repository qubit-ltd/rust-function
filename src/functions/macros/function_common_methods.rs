/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Function Common Methods Macro
//!
//! Generates common Function methods (new, new_with_name, name,
//! set_name, identity)
//!
//! Generates constructor methods, name management methods and identity
//! constructor for Function structs. This macro should be called inside
//! an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter functions.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds
//!   (e.g., `Fn(&T) -> R + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```ignore
//! // Single generic parameter - Function
//! impl_function_common_methods!(
//!     BoxFunction<T, R>,
//!     (Fn(&T) -> R + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Stateful function with state
//! impl_function_common_methods!(
//!     ArcStatefulFunction<T, R>,
//!     (FnMut(&T) -> R + Send + 'static),
//!     |f| Arc::new(Mutex::new(f))
//! );
//!
//! // Two generic parameters - BiFunction (if applicable)
//! impl_function_common_methods!(
//!     BoxBiFunction<T, U, R>,
//!     (Fn(&T, &U) -> R + 'static),
//!     |f| Box::new(f)
//! );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new function
//! * `new_with_name()` - Creates a named function
//! * `name()` - Gets the name of the function
//! * `set_name()` - Sets the name of the function
//! * `identity()` - Creates an identity function
//!
//! # Author
//!
//! Haixing Hu

/// Generates common Function methods (new, new_with_name, name,
/// set_name, identity)
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates constructor methods, name management methods
/// and identity constructor for Function structs.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter functions.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds
///   (e.g., `Fn(&T) -> R + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage
///
/// ```ignore
/// // Single generic parameter - Function
/// impl_function_common_methods!(
///     BoxFunction<T, R>,
///     (Fn(&T) -> R + 'static),
///     |f| Box::new(f)
/// );
///
/// // Stateful function with state
/// impl_function_common_methods!(
///     ArcStatefulFunction<T, R>,
///     (FnMut(&T) -> R + Send + 'static),
///     |f| Arc::new(Mutex::new(f))
/// );
///
/// // Two generic parameters - BiFunction (if applicable)
/// impl_function_common_methods!(
///     BoxBiFunction<T, U, R>,
///     (Fn(&T, &U) -> R + 'static),
///     |f| Box::new(f)
/// );
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new function
/// * `new_with_name()` - Creates a named function
/// * `name()` - Gets the name of the function
/// * `set_name()` - Sets the name of the function
/// * `identity()` - Creates an identity function
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_function_common_methods {
    // Two generic parameters - Function types
    (
        $struct_name:ident < $t:ident, $r:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_common_new_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "function"
        );
        impl_common_name_methods!("function");
    };

    // Three generic parameters - BiFunction types (if applicable)
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_common_new_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "bi-function"
        );
        impl_common_name_methods!("bi-function");
    };
}

pub(crate) use impl_function_common_methods;
