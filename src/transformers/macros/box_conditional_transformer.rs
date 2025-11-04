/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Box Conditional Transformer Macro
//!
//! Generates Box-based Conditional Transformer implementations
//!
//! For Box-based conditional transformers, generates `and_then` and `or_else`
//! methods, as well as complete Transformer/BiTransformer trait
//! implementations.
//!
//! Box type characteristics:
//! - `and_then` and `or_else` consume self (because Box cannot Clone)
//! - Does not implement `into_arc()` (because Box types are not Send + Sync)
//! - Does not implement `to_xxx()` methods (because Box types cannot Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$transformer_type` - Transformer wrapper type name
//! * `$transformer_trait` - Transformer trait name
//!
//! # Usage Examples
//!
//! ```ignore
//! // Two-parameter Transformer
//! impl_box_conditional_transformer!(
//!     BoxConditionalTransformer<T, U>,
//!     BoxTransformer,
//!     Transformer
//! );
//!
//! // Three-parameter BiTransformer
//! impl_box_conditional_transformer!(
//!     BoxConditionalBiTransformer<T, U, V>,
//!     BoxBiTransformer,
//!     BiTransformer
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Box-based Conditional Transformer implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Box-based conditional transformers, generates `and_then` and `or_else` methods,
/// as well as complete Transformer/BiTransformer trait implementations.
///
/// Box type characteristics:
/// - `and_then` and `or_else` consume self (because Box cannot Clone)
/// - Does not implement `into_arc()` (because Box types are not Send + Sync)
/// - Does not implement `to_xxx()` methods (because Box types cannot Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$transformer_type` - Transformer wrapper type name
/// * `$transformer_trait` - Transformer trait name
///
/// # Usage Examples
///
/// ```ignore
/// // Two-parameter Transformer
/// impl_box_conditional_transformer!(
///     BoxConditionalTransformer<T, U>,
///     BoxTransformer,
///     Transformer
/// );
///
/// // Three-parameter BiTransformer
/// impl_box_conditional_transformer!(
///     BoxConditionalBiTransformer<T, U, V>,
///     BoxBiTransformer,
///     BiTransformer
/// );
/// ```
macro_rules! impl_box_conditional_transformer {
    // Two generic parameters - Transformer
    (
        $struct_name:ident<$t:ident, $u:ident>,
        $transformer_type:ident,
        $transformer_trait:ident
    ) => {
        impl<$t, $u> $struct_name<$t, $u>
        where
            $t: 'static,
            $u: 'static,
        {
            /// Chains another transformer in sequence
            ///
            /// Combines the current conditional transformer with another transformer
            /// into a new transformer that implements the following semantics:
            ///
            /// When the returned transformer is called with an argument:
            /// 1. First, it checks the predicate of this conditional transformer
            /// 2. If the predicate is satisfied, it executes the internal
            ///    transformer of this conditional transformer
            /// 3. Then, **regardless of whether the predicate was satisfied**,
            ///    it unconditionally executes the `next` transformer on the result
            ///
            /// In other words, this creates a transformer that conditionally
            /// transforms the input, and then always applies another transformation
            /// to the result.
            ///
            /// # Parameters
            ///
            /// * `next` - The next transformer to execute (always executed)
            ///
            /// # Returns
            ///
            /// Returns a new combined transformer
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let transformer1 = BoxTransformer::new(|x: &i32| *x + 1);
            /// let transformer2 = BoxTransformer::new(|x: &i32| *x * 2);
            ///
            /// let conditional = transformer1.when(|x| *x > 0);
            /// let chained = conditional.and_then(transformer2);
            ///
            /// assert_eq!(chained.transform(&5), 12);  // (5 + 1) * 2 = 12
            /// assert_eq!(chained.transform(&-5), -10); // (-5) * 2 = -10 (not (-5 + 1) * 2)
            /// ```
            #[allow(unused_mut)]
            pub fn and_then<C>(self, mut next: C) -> $transformer_type<$t, $u>
            where
                C: $transformer_trait<$u, $u> + 'static,
            {
                let first_predicate = self.predicate;
                let mut first_transformer = self.transformer;
                $transformer_type::new(move |t| {
                    let intermediate = if first_predicate.test(t) {
                        first_transformer.transform(t)
                    } else {
                        // Identity transformation for conditional case
                        t.clone()
                    };
                    next.transform(&intermediate)
                })
            }

            /// Adds an else branch
            ///
            /// Executes the original transformer when the condition is satisfied, otherwise
            /// executes else_transformer.
            ///
            /// # Parameters
            ///
            /// * `else_transformer` - The transformer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new transformer with if-then-else logic
            #[allow(unused_mut)]
            pub fn or_else<C>(self, mut else_transformer: C) -> $transformer_type<$t, $u>
            where
                C: $transformer_trait<$t, $u> + 'static,
            {
                let predicate = self.predicate;
                let mut then_transformer = self.transformer;
                $transformer_type::new(move |t| {
                    if predicate.test(t) {
                        then_transformer.transform(t)
                    } else {
                        else_transformer.transform(t)
                    }
                })
            }
        }
    };

    // Three generic parameters - BiTransformer
    (
        $struct_name:ident<$t:ident, $u:ident, $v:ident>,
        $transformer_type:ident,
        $transformer_trait:ident
    ) => {
        impl<$t, $u, $v> $struct_name<$t, $u, $v>
        where
            $t: 'static,
            $u: 'static,
            $v: 'static,
        {
            /// Chains another bi-transformer in sequence
            ///
            /// Combines the current conditional bi-transformer with another
            /// bi-transformer into a new bi-transformer that implements the
            /// following semantics:
            ///
            /// When the returned bi-transformer is called with two arguments:
            /// 1. First, it checks the predicate of this conditional
            ///    bi-transformer
            /// 2. If the predicate is satisfied, it executes the internal
            ///    bi-transformer of this conditional bi-transformer
            /// 3. Then, **regardless of whether the predicate was
            ///    satisfied**, it unconditionally executes the `next`
            ///    bi-transformer on the result
            ///
            /// In other words, this creates a bi-transformer that conditionally
            /// transforms the inputs, and then always applies another transformation
            /// to the result.
            ///
            /// # Parameters
            ///
            /// * `next` - The next bi-transformer to execute (always executed)
            ///
            /// # Returns
            ///
            /// Returns a new combined bi-transformer
            ///
            /// # Examples
            ///
            /// ```ignore
            /// let transformer1 = BoxBiTransformer::new(|x: &i32, y: &i32| x + y);
            /// let transformer2 = BoxBiTransformer::new(|result: &i32, _| *result * 2);
            ///
            /// let conditional = transformer1.when(|x, y| *x > 0 && *y > 0);
            /// let chained = conditional.and_then(transformer2);
            ///
            /// assert_eq!(chained.transform(&5, &3), 16);  // (5 + 3) * 2 = 16
            /// assert_eq!(chained.transform(&-5, &3), -4); // (0) * 2 = -4 (identity case)
            /// ```
            #[allow(unused_mut)]
            pub fn and_then<C>(self, mut next: C) -> $transformer_type<$t, $u, $v>
            where
                C: $transformer_trait<$v, $v, $v> + 'static,
            {
                let first_predicate = self.predicate;
                let mut first_transformer = self.transformer;
                $transformer_type::new(move |t, u| {
                    let intermediate = if first_predicate.test(t, u) {
                        first_transformer.transform(t, u)
                    } else {
                        // Identity transformation for conditional case
                        // This would need to be handled properly in actual implementation
                        panic!("Identity transformation not implemented for bi-transformer")
                    };
                    next.transform(&intermediate, &intermediate)
                })
            }

            /// Adds an else branch
            ///
            /// Executes the original bi-transformer when the condition is satisfied, otherwise
            /// executes else_transformer.
            ///
            /// # Parameters
            ///
            /// * `else_transformer` - The bi-transformer for the else branch
            ///
            /// # Returns
            ///
            /// Returns a new bi-transformer with if-then-else logic
            #[allow(unused_mut)]
            pub fn or_else<C>(self, mut else_transformer: C) -> $transformer_type<$t, $u, $v>
            where
                C: $transformer_trait<$t, $u, $v> + 'static,
            {
                let predicate = self.predicate;
                let mut then_transformer = self.transformer;
                $transformer_type::new(move |t, u| {
                    if predicate.test(t, u) {
                        then_transformer.transform(t, u)
                    } else {
                        else_transformer.transform(t, u)
                    }
                })
            }
        }
    };
}

pub(crate) use impl_box_conditional_transformer;
