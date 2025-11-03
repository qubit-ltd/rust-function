/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # Predicate Common Methods Macro
//!
//! Generates common Predicate methods (new, new_with_name, name,
//! set_name, always_true)
//!
//! Generates constructor methods, name management methods and always_true
//! constructor for Predicate structs. This macro should be called inside
//! an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter predicates.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds
//!   (e.g., `Fn(&T) -> bool + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```ignore
//! // Single generic parameter - Predicate
//! impl_predicate_common_methods!(
//!     BoxPredicate<T>,
//!     (Fn(&T) -> bool + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Two generic parameters - BiPredicate
//! impl_predicate_common_methods!(
//!     BoxBiPredicate<T, U>,
//!     (Fn(&T, &U) -> bool + 'static),
//!     |f| Box::new(f)
//! );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new predicate
//! * `new_with_name()` - Creates a named predicate
//! * `name()` - Gets the name of the predicate
//! * `set_name()` - Sets the name of the predicate
//! * `always_true()` - Creates a predicate that always returns true
//!
//! # Author
//!
//! Haixing Hu

/// Generates common Predicate methods (new, new_with_name, name,
/// set_name, always_true, always_false)
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates constructor methods, name management methods
/// and always_true/always_false constructors for Predicate structs.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter predicates.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds
///   (e.g., `Fn(&T) -> bool + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage
///
/// ```ignore
/// // Single generic parameter - Predicate
/// impl_predicate_common_methods!(
///     BoxPredicate<T>,
///     (Fn(&T) -> bool + 'static),
///     |f| Box::new(f)
/// );
///
/// // Two generic parameters - BiPredicate
/// impl_predicate_common_methods!(
///     BoxBiPredicate<T, U>,
///     (Fn(&T, &U) -> bool + 'static),
///     |f| Box::new(f)
/// );
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new predicate
/// * `new_with_name()` - Creates a named predicate
/// * `name()` - Gets the name of the predicate
/// * `set_name()` - Sets the name of the predicate
/// * `always_true()` - Creates a predicate that always returns true
/// * `always_false()` - Creates a predicate that always returns false
macro_rules! impl_predicate_common_methods {
    // Internal rule: generates new and new_with_name methods
    // Parameters:
    //   $fn_trait_with_bounds - Function trait bounds
    //   $f - Closure parameter name
    //   $wrapper_expr - Wrapper expression
    //   $type_desc - Type description for docs (e.g., "predicate" or "bi-predicate")
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

    // Single generic parameter - Predicate types
    (
        $struct_name:ident < $t:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {

        impl_predicate_common_methods!(@new_methods
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "predicate"
        );

        impl_predicate_common_methods!(@name_methods "predicate");

        /// Creates a predicate that always returns `true`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `true`.")]
        pub fn always_true() -> Self {
            Self::new_with_name(ALWAYS_TRUE_NAME, |_| true)
        }

        /// Creates a predicate that always returns `false`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `false`.")]
        pub fn always_false() -> Self {
            Self::new_with_name(ALWAYS_FALSE_NAME, |_| false)
        }
    };

    // Two generic parameters - BiPredicate types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {

        impl_predicate_common_methods!(@new_methods
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "bi-predicate"
        );

        impl_predicate_common_methods!(@name_methods "bi-predicate");

        /// Creates a bi-predicate that always returns `true`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `true`.")]
        pub fn always_true() -> Self {
            Self::new_with_name(ALWAYS_TRUE_NAME, |_, _| true)
        }

        /// Creates a bi-predicate that always returns `false`.
        ///
        /// # Returns
        ///
        #[doc = concat!("A new `", stringify!($struct_name), "` that always returns `false`.")]
        pub fn always_false() -> Self {
            Self::new_with_name(ALWAYS_FALSE_NAME, |_, _| false)
        }
    };
}

pub(crate) use impl_predicate_common_methods;
