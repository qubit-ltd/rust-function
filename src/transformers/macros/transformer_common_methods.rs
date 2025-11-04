/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Transformer Common Methods Macro
//!
//! Generates common Transformer methods using shared implementations
//! (new, new_with_name, new_with_optional_name, name, set_name, identity)
//!
//! This macro uses `impl_common_new_methods` and `impl_common_name_methods`
//! to generate constructor and name management methods, plus a specialized
//! identity constructor for Transformer structs. This macro should be called
//! inside an impl block.
//!
//! The macro automatically detects the number of generic parameters and
//! generates the appropriate implementations for single-parameter or
//! two-parameter transformers.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$fn_trait_with_bounds` - Closure trait with complete bounds
//!   (e.g., `Fn(&T) -> U + 'static`)
//! * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
//!
//! # Usage
//!
//! ```ignore
//! // Single generic parameter - Transformer
//! impl_transformer_common_methods!(
//!     BoxTransformer<T, U>,
//!     (Fn(&T) -> U + 'static),
//!     |f| Box::new(f)
//! );
//!
//! // Single generic parameter - StatefulTransformer
//! impl_transformer_common_methods!(
//!     ArcStatefulTransformer<T, U>,
//!     (FnMut(&T) -> U + Send + 'static),
//!     |f| Arc::new(Mutex::new(f))
//! );
//!
//! // Two generic parameters - BiTransformer
//! impl_transformer_common_methods!(
//!     BoxBiTransformer<T, U, V>,
//!     (Fn(&T, &U) -> V + 'static),
//!     |f| Box::new(f)
//! );
//! ```
//!
//! # Generated Methods
//!
//! * `new()` - Creates a new transformer
//! * `new_with_name()` - Creates a named transformer
//! * `new_with_optional_name()` - Creates a transformer with optional name
//! * `name()` - Gets the name of the transformer
//! * `set_name()` - Sets the name of the transformer
//! * `identity()` - Creates a transformer that returns the input unchanged
//!
//! # Author
//!
//! Haixing Hu

/// Generates common Transformer methods using shared implementations
/// (new, new_with_name, new_with_optional_name, name, set_name, identity)
///
/// This macro uses `impl_common_new_methods` and `impl_common_name_methods`
/// to generate constructor and name management methods, plus a specialized
/// identity constructor for Transformer structs. This macro should be used
/// inside an existing impl block for the target struct.
///
/// The macro automatically detects the number of generic parameters and
/// generates the appropriate implementations for single-parameter or
/// two-parameter transformers.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$fn_trait_with_bounds` - Closure trait with complete bounds
///   (e.g., `Fn(&T) -> U + 'static`)
/// * `$wrapper_expr` - Wrapper expression (uses `f` for the closure)
///
/// # Usage
///
/// ```ignore
/// // Single generic parameter - Transformer
/// impl_transformer_common_methods!(
///     BoxTransformer<T, U>,
///     (Fn(&T) -> U + 'static),
///     |f| Box::new(f)
/// );
///
/// // Single generic parameter - StatefulTransformer
/// impl_transformer_common_methods!(
///     ArcStatefulTransformer<T, U>,
///     (FnMut(&T) -> U + Send + 'static),
///     |f| Arc::new(Mutex::new(f))
/// );
///
/// // Two generic parameters - BiTransformer
/// impl_transformer_common_methods!(
///     BoxBiTransformer<T, U, V>,
///     (Fn(&T, &U) -> V + 'static),
///     |f| Box::new(f)
/// );
/// ```
///
/// # Generated Methods
///
/// * `new()` - Creates a new transformer
/// * `new_with_name()` - Creates a named transformer
/// * `new_with_optional_name()` - Creates a transformer with optional name
/// * `name()` - Gets the name of the transformer
/// * `set_name()` - Sets the name of the transformer
/// * `identity()` - Creates a transformer that returns the input unchanged
///
/// # Author
///
/// Haixing Hu
macro_rules! impl_transformer_common_methods {
    // Single generic parameter - Transformer types
    (
        $struct_name:ident < $t:ident, $u:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_common_new_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "transformer"
        );

        impl_common_name_methods!("transformer");

        /// Creates an identity transformer.
        ///
        /// Creates a transformer that returns the input value unchanged. Useful for
        /// default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new transformer instance that returns the input unchanged.
        pub fn identity() -> $struct_name<$t, $t> {
            $struct_name::<$t, $t>::new(|t| t)
        }
    };

    // Two generic parameters - BiTransformer types
    (
        $struct_name:ident < $t:ident, $u:ident, $v:ident >,
        ($($fn_trait_with_bounds:tt)+),
        |$f:ident| $wrapper_expr:expr
    ) => {
        impl_common_new_methods!(
            ($($fn_trait_with_bounds)+),
            |$f| $wrapper_expr,
            "bi-transformer"
        );

        impl_common_name_methods!("bi-transformer");

        /// Creates an identity bi-transformer.
        ///
        /// Creates a bi-transformer that returns the first input value unchanged. Useful for
        /// default values or placeholder implementations.
        ///
        /// # Returns
        ///
        /// Returns a new bi-transformer instance that returns the first input unchanged.
        pub fn identity() -> $struct_name<$t, $u, $t> {
            $struct_name::<$t, $u, $t>::new(|t, _| t)
        }
    };
}

pub(crate) use impl_transformer_common_methods;
