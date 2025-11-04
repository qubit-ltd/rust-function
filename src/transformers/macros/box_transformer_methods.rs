/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Box Transformer Methods Macro
//!
//! Generates when and and_then method implementations for Box-based Transformer
//!
//! Generates conditional execution when method and chaining and_then method
//! for Box-based transformers that consume self (because Box cannot be cloned).
//!
//! This macro supports both single-parameter and two-parameter transformers through
//! pattern matching on the struct signature.
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - The struct name with its generic parameters
//!   - Single parameter: `BoxTransformer<T, U>`
//!   - Two parameters: `BoxBiTransformer<T, U, V>`
//! * `$conditional_type` - The conditional transformer type for when (e.g., BoxConditionalTransformer)
//! * `$transformer_trait` - Transformer trait name (e.g., Transformer, BiTransformer)
//!
//! # Parameter Usage Comparison
//!
//! | Transformer Type | Struct Signature | `$conditional_type` |
//! |------------------|-----------------|----------------|
//! | **Transformer** | `BoxTransformer<T, U>` | BoxConditionalTransformer |
//! | **TransformerOnce** | `BoxTransformerOnce<T, U>` | BoxConditionalTransformerOnce |
//! | **StatefulTransformer** | `BoxStatefulTransformer<T, U>` | BoxConditionalStatefulTransformer |
//! | **BiTransformer** | `BoxBiTransformer<T, U, V>` | BoxConditionalBiTransformer |
//! | **BiTransformerOnce** | `BoxBiTransformerOnce<T, U, V>` | BoxConditionalBiTransformerOnce |
//! | **StatefulBiTransformer** | `BoxStatefulBiTransformer<T, U, V>` | BoxConditionalStatefulBiTransformer |
//!
//! | `$transformer_trait` |
//! |---------------------|
//! | Transformer |
//! | TransformerOnce |
//! | StatefulTransformer |
//! | BiTransformer |
//! | BiTransformerOnce |
//! | StatefulBiTransformer |
//!
//! # Examples
//!
//! ```ignore
//! // Single-parameter transformer
//! impl_box_transformer_methods!(
//!     BoxTransformer<T, U>,
//!     BoxConditionalTransformer,
//!     Transformer
//! );
//!
//! // Two-parameter transformer
//! impl_box_transformer_methods!(
//!     BoxBiTransformer<T, U, V>,
//!     BoxConditionalBiTransformer,
//!     BiTransformer
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates when and and_then method implementations for Box-based Transformer
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// Generates conditional execution when method and chaining and_then method
/// for Box-based transformers that consume self (because Box cannot be cloned).
///
/// This macro supports both single-parameter and two-parameter transformers through
/// pattern matching on the struct signature.
///
/// # Parameters
///
/// * `$struct_name<$generics>` - The struct name with its generic parameters
///   - Single parameter: `BoxTransformer<T, U>`
///   - Two parameters: `BoxBiTransformer<T, U, V>`
/// * `$conditional_type` - The conditional transformer type for when (e.g., BoxConditionalTransformer)
/// * `$transformer_trait` - Transformer trait name (e.g., Transformer, BiTransformer)
///
/// # Parameter Usage Comparison
///
/// | Transformer Type | Struct Signature | `$conditional_type` | `$transformer_trait` |
// |------------------|-----------------|----------------|---------------------|
// | **Transformer** | `BoxTransformer<T, U>` | BoxConditionalTransformer | Transformer |
// | **TransformerOnce** | `BoxTransformerOnce<T, U>` | BoxConditionalTransformerOnce | TransformerOnce |
// | **StatefulTransformer** | `BoxStatefulTransformer<T, U>` | BoxConditionalStatefulTransformer | StatefulTransformer |
// | **BiTransformer** | `BoxBiTransformer<T, U, V>` | BoxConditionalBiTransformer | BiTransformer |
// | **BiTransformerOnce** | `BoxBiTransformerOnce<T, U, V>` | BoxConditionalBiTransformerOnce | BiTransformerOnce |
// | **StatefulBiTransformer** | `BoxStatefulBiTransformer<T, U, V>` | BoxConditionalStatefulBiTransformer | StatefulBiTransformer |
//
/// # Examples
///
/// ```ignore
/// // Single-parameter transformer
/// impl_box_transformer_methods!(
///     BoxTransformer<T, U>,
///     BoxConditionalTransformer,
///     Transformer
/// );
///
/// // Two-parameter transformer
/// impl_box_transformer_methods!(
///     BoxBiTransformer<T, U, V>,
///     BoxConditionalBiTransformer,
///     BiTransformer
/// );
// ```
macro_rules! impl_box_transformer_methods {
    // Single generic parameter - Transformer
    ($struct_name:ident < $t:ident, $u:ident >, $conditional_type:ident, $transformer_trait:ident) => {
        /// Creates a conditional transformer that executes based on predicate
        /// result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The predicate to determine whether to execute
        ///   the transformation operation
        ///
        /// # Returns
        ///
        /// Returns a conditional transformer that only executes when the
        /// predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use std::sync::Arc;
        /// use std::sync::atomic::{AtomicI32, Ordering};
        /// use prism3_rust_function::transformers::*;
        ///
        /// let transformer = BoxTransformer::new({
        ///     |value: &i32| value * 2
        /// });
        ///
        /// let conditional = transformer.when(|value: &i32| *value > 0);
        /// assert_eq!(conditional.transform(&5), 10);  // transformed
        /// assert_eq!(conditional.transform(&-1), -1); // identity (unchanged)
        /// ```
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $u>
        where
            P: Predicate<$t> + 'static,
        {
            $conditional_type {
                transformer: self,
                predicate: predicate.into_box(),
            }
        }

        /// Chains execution with another transformer, executing the current
        /// transformer first, then the subsequent transformer.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent transformer to execute after the current
        ///   transformer completes
        ///
        /// # Returns
        ///
        /// Returns a new transformer that executes the current transformer and
        /// the subsequent transformer in sequence.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_rust_function::transformers::*;
        ///
        /// let transformer1 = BoxTransformer::new({
        ///     |value: &i32| value + 1
        /// });
        ///
        /// let transformer2 = BoxTransformer::new({
        ///     |value: &i32| value * 2
        /// });
        ///
        /// let chained = transformer1.and_then(transformer2);
        /// assert_eq!(chained.transform(&5), 12); // (5 + 1) * 2 = 12
        /// ```
        #[allow(unused_mut)]
        pub fn and_then<C>(self, mut after: C) -> $struct_name<$t, $u>
        where
            Self: Sized + 'static,
            $t: 'static,
            $u: 'static,
            C: $transformer_trait<$t, $u> + 'static,
        {
            let mut first = self;
            $struct_name::new(move |t: &$t| {
                let intermediate = first.transform(t);
                after.transform(&intermediate)
            })
        }
    };
    // Two generic parameters - BiTransformer
    ($struct_name:ident < $t:ident, $u:ident, $v:ident >, $conditional_type:ident, $transformer_trait:ident) => {
        /// Creates a conditional two-parameter transformer that executes based
        /// on bi-predicate result.
        ///
        /// # Parameters
        ///
        /// * `predicate` - The bi-predicate to determine whether to execute
        ///   the transformation operation
        ///
        /// # Returns
        ///
        /// Returns a conditional two-parameter transformer that only executes
        /// when the predicate returns `true`.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_rust_function::transformers::*;
        ///
        /// let bi_transformer = BoxBiTransformer::new({
        ///     |key: &String, value: &i32| format!("{}: {}", key, value)
        /// });
        ///
        /// let conditional = bi_transformer.when(|key: &String, value: &i32| *value > 0);
        /// assert_eq!(conditional.transform(&"test".to_string(), &5), "test: 5".to_string());  // transformed
        /// assert_eq!(conditional.transform(&"test".to_string(), &-1), "test".to_string());    // identity (key unchanged)
        /// ```
        pub fn when<P>(self, predicate: P) -> $conditional_type<$t, $u, $v>
        where
            P: BiPredicate<$t, $u> + 'static,
        {
            $conditional_type {
                transformer: self,
                predicate: predicate.into_box(),
            }
        }

        /// Chains execution with another two-parameter transformer, executing
        /// the current transformer first, then the subsequent transformer.
        ///
        /// # Parameters
        ///
        /// * `after` - The subsequent two-parameter transformer to execute after
        ///   the current transformer completes
        ///
        /// # Returns
        ///
        /// Returns a new two-parameter transformer that executes the current
        /// transformer and the subsequent transformer in sequence.
        ///
        /// # Examples
        ///
        /// ```rust
        /// use prism3_rust_function::transformers::*;
        ///
        /// let bi_transformer1 = BoxBiTransformer::new({
        ///     |key: &String, value: &i32| (key.clone(), *value + 1)
        /// });
        ///
        /// let bi_transformer2 = BoxBiTransformer::new({
        ///     |key: &String, value: &i32| format!("{}: {}", key, value)
        /// });
        ///
        /// let chained = bi_transformer1.and_then(bi_transformer2);
        /// let result = chained.transform(&"test".to_string(), &5);
        /// assert_eq!(result, "test: 6"); // (value + 1) = 6
        /// ```
        #[allow(unused_mut)]
        pub fn and_then<C>(self, mut after: C) -> $struct_name<$t, $u, $v>
        where
            Self: Sized + 'static,
            $t: 'static,
            $u: 'static,
            $v: 'static,
            C: $transformer_trait<$t, $u, $v> + 'static,
        {
            let mut first = self;
            $struct_name::new(move |t: &$t, u: &$u| {
                let intermediate = first.transform(t, u);
                after.transform(&intermediate.0, &intermediate.1)
            })
        }
    };
}

pub(crate) use impl_box_transformer_methods;
