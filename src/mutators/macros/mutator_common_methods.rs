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
    // Single generic parameter - Mutator types
    (
        $struct_name:ident < $t:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_common_new_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "mutator"
        );
        impl_common_name_methods!("mutator");

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
