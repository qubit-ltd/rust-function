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
macro_rules! impl_function_common_methods {
    // Internal rule: generates new and new_with_name methods
    // Parameters:
    //   $fn_trait_with_bounds - Function trait bounds
    //   $f - Closure parameter name
    //   $wrapper_expr - Wrapper expression
    //   $type_desc - Type description for docs (e.g., "function" or "bi-function")
    (@new_methods
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr,
        $type_desc:literal
    ) => {
        #[doc = concat!("Creates a new ", $type_desc, ".")]
        ///
        #[doc = concat!("Wraps the provided closure in the appropriate smart pointer type for this ", $type_desc, " implementation.")]
        ///
        /// # Type Parameters
        ///
        /// * `F` - The closure type
        ///
        /// # Parameters
        ///
        /// * `f` - The closure to wrap
        ///
        /// # Returns
        ///
        #[doc = concat!("Returns a new ", $type_desc, " instance wrapping the closure.")]
        pub fn new<F>($f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name: None,
            }
        }

        #[doc = concat!("Creates a new named ", $type_desc, ".")]
        ///
        /// Wraps the provided closure and assigns it a name, which is
        /// useful for debugging and logging purposes.
        ///
        /// # Type Parameters
        ///
        /// * `F` - The closure type
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `name` - The name for this ", $type_desc)]
        /// * `f` - The closure to wrap
        ///
        /// # Returns
        ///
        #[doc = concat!("Returns a new named ", $type_desc, " instance wrapping the closure.")]
        pub fn new_with_name<F>(name: &str, $f: F) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name: Some(name.to_string()),
            }
        }

        #[doc = concat!("Creates a new named ", $type_desc, ".")]
        ///
        /// Wraps the provided closure and assigns it a name, which is
        /// useful for debugging and logging purposes.
        ///
        /// # Type Parameters
        ///
        /// * `F` - The closure type
        ///
        /// # Parameters
        ///
        /// * `f` - The closure to wrap
        #[doc = concat!("* `name` - The optional name for this ", $type_desc)]
        ///
        /// # Returns
        ///
        #[doc = concat!("Returns a new named ", $type_desc, " instance wrapping the closure.")]
        pub fn new_with_optional_name<F>($f: F, name: Option<String>) -> Self
        where
            F: $($fn_trait_with_bounds)+,
        {
            Self {
                function: $wrapper_expr,
                name: name,
            }
        }
    };

    // Internal rule: generates name and set_name methods
    (@name_methods $type_desc:literal) => {
        #[doc = concat!("Gets the name of this ", $type_desc, ".")]
        ///
        /// # Returns
        ///
        /// Returns `Some(&str)` if a name was set, `None` otherwise.
        pub fn name(&self) -> Option<&str> {
            self.name.as_deref()
        }

        #[doc = concat!("Sets the name of this ", $type_desc, ".")]
        ///
        /// # Parameters
        ///
        #[doc = concat!("* `name` - The name to set for this ", $type_desc)]
        pub fn set_name(&mut self, name: &str) {
            self.name = Some(name.to_string());
        }
    };

    // Two generic parameters - Function types
    (
        $struct_name:ident < $t:ident, $r:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_function_common_methods!(@new_methods
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "function"
        );

        impl_function_common_methods!(@name_methods "function");
    };

    // Three generic parameters - BiFunction types (if applicable)
    (
        $struct_name:ident < $t:ident, $u:ident, $r:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_function_common_methods!(@new_methods
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "bi-function"
        );

        impl_function_common_methods!(@name_methods "bi-function");
    };
}

pub(crate) use impl_function_common_methods;
