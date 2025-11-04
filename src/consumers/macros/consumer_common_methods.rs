/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Consumer Common Methods Macro
//!
//! Generates common Consumer methods (new, new_with_name, name,
//! set_name, noop)
//!
//! Generates constructor methods, name management methods and noop
//! constructor for Consumer structs. This macro should be called inside
//! an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter consumers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds
//!   (e.g., `Fn(&T) + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```ignore
//! // Single generic parameter - Consumer
//! impl_consumer_common_methods!(
//!     BoxConsumer<T>,
//!     (Fn(&T) + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Single generic parameter - StatefulConsumer
//! impl_consumer_common_methods!(
//!     ArcStatefulConsumer<T>,
//!     (FnMut(&T) + Send + 'static),
//!     |f| Arc::new(Mutex::new(f))
//! );
//!
//! // Two generic parameters - BiConsumer
//! impl_consumer_common_methods!(
//!     BoxBiConsumer<T, U>,
//!     (Fn(&T, &U) + 'static),
//!     |f| Box::new(f)
//! );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new consumer
//! * `new_with_name()` - Creates a named consumer
//! * `name()` - Gets the name of the consumer
//! * `set_name()` - Sets the name of the consumer
//! * `noop()` - Creates a consumer that performs no operation
//!
//! # Author
//!
//! Haixing Hu

/// Generates common Consumer methods (new, new_with_name, name,
/// set_name, noop)
///
/// This macro should be used inside an existing impl block for the target
/// struct. It generates individual methods but does not create a complete
/// impl block itself. Generates constructor methods, name management methods
/// and noop constructor for Consumer structs.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter consumers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds
///   (e.g., `Fn(&T) + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage
///
/// ```ignore
/// // Single generic parameter - Consumer
/// impl_consumer_common_methods!(
///     BoxConsumer<T>,
///     (Fn(&T) + 'static),
///     |f| Box::new(f)
/// );
///
/// // Single generic parameter - StatefulConsumer
/// impl_consumer_common_methods!(
///     ArcStatefulConsumer<T>,
///     (FnMut(&T) + Send + 'static),
///     |f| Arc::new(Mutex::new(f))
/// );
///
/// // Two generic parameters - BiConsumer
/// impl_consumer_common_methods!(
///     BoxBiConsumer<T, U>,
///     (Fn(&T, &U) + 'static),
///     |f| Box::new(f)
/// );
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new consumer
/// * `new_with_name()` - Creates a named consumer
/// * `name()` - Gets the name of the consumer
/// * `set_name()` - Sets the name of the consumer
/// * `noop()` - Creates a consumer that performs no operation
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_consumer_common_methods {
    // Single generic parameter - Consumer types
    (
        $struct_name:ident < $t:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_common_new_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "consumer"
        );
        impl_common_name_methods!("consumer");

        /// Creates a no-operation consumer.
        ///
        /// Creates a consumer that does nothing when called. Useful for
        /// default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new consumer instance that performs no operation.
        pub fn noop() -> Self {
            Self::new(|_| {})
        }
    };

    // Two generic parameters - BiConsumer types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_common_new_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "bi-consumer"
        );
        impl_common_name_methods!("bi-consumer");

        /// Creates a no-operation bi-consumer.
        ///
        /// Creates a bi-consumer that does nothing when called. Useful
        /// for default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new bi-consumer instance that performs no operation.
        pub fn noop() -> Self {
            Self::new(|_, _| {})
        }
    };
}

pub(crate) use impl_consumer_common_methods;
