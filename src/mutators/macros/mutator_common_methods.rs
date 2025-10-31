/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
/*! # Mutator Common Methods Macro
//!
//! Generates common Mutator methods (new, new_with_name, name,
//! set_name, noop)
//!
//! Generates constructor methods, name management methods and noop
//! constructor for Mutator structs. This macro should be called inside
//! an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter mutators.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds
//!   (e.g., `FnMut(&mut T) + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```ignore
//! // Single generic parameter - Mutator
//! impl_mutator_common_methods!(
//!     BoxMutator<T>,
//!     (FnMut(&mut T) + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Single generic parameter - StatefulMutator
//! impl_mutator_common_methods!(
//!     ArcStatefulMutator<T>,
//!     (FnMut(&mut T) + Send + 'static),
//!     |f| Arc::new(Mutex::new(f))
//! );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new mutator
//! * `new_with_name()` - Creates a named mutator
//! * `name()` - Gets the name of the mutator
//! * `set_name()` - Sets the name of the mutator
//! * `noop()` - Creates a mutator that performs no operation
//!
//! # Author
//!
//! Haixing Hu
*/

/// Generates common Mutator methods (new, new_with_name, name,
/// set_name, noop)
///
/// This macro should be used inside an impl block to generate constructor
/// methods, name management methods and noop constructor for Mutator structs.
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter mutators.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds
///   (e.g., `FnMut(&mut T) + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage Location
///
/// This macro should be used inside an impl block for the struct type.
///
/// # Usage
///
/// ```ignore
/// impl<T> BoxMutator<T> {
///     // Inside an impl block
///     impl_mutator_common_methods!(
///         BoxMutator<T>,
///         (FnMut(&mut T) + 'static),
///         |f| Box::new(f)
///     );
/// }
///
/// impl<T> ArcStatefulMutator<T> {
///     // Inside an impl block for StatefulMutator
///     impl_mutator_common_methods!(
///         ArcStatefulMutator<T>,
///         (FnMut(&mut T) + Send + 'static),
///         |f| Arc::new(Mutex::new(f))
///     );
/// }
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new mutator
/// * `new_with_name()` - Creates a named mutator
/// * `name()` - Gets the name of the mutator
/// * `set_name()` - Sets the name of the mutator
/// * `noop()` - Creates a mutator that performs no operation
macro_rules! impl_mutator_common_methods {
    // Internal rule: generates new and new_with_name methods
    // Parameters:
    //   $fn_trait_with_bounds - Function trait bounds
    //   $f - Closure parameter name
    //   $wrapper_expr - Wrapper expression
    //   $type_desc - Type description for docs (e.g., "mutator")
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

    // Single generic parameter - Mutator types
    (
        $struct_name:ident < $t:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_mutator_common_methods!(@new_methods
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "mutator"
        );

        impl_mutator_common_methods!(@name_methods "mutator");

        /// Creates a no-operation mutator.
        ///
        /// Creates a mutator that does nothing when called. Useful for
        /// default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new mutator instance that performs no operation.
        pub fn noop() -> Self {
            Self::new(|_| {})
        }
    };
}

pub(crate) use impl_mutator_common_methods;
