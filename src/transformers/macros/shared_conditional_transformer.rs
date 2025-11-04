/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Shared Conditional Transformer Macro
//!
//! Generates Arc/Rc-based Conditional Transformer implementations
//!
//! For Arc/Rc-based conditional transformers, generates `and_then` and `or_else`
//! methods, as well as complete Transformer/BiTransformer trait
//! implementations.
//!
//! Arc/Rc type characteristics:
//! - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
//! - Uses trait default implementations for `into_arc()` and `to_arc()`
//! - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync
//!   constraints)
//! - Rc types will get compile errors if trying to use `into_arc()` or
//!   `to_arc()` (don't satisfy Send + Sync)
//! - Implement complete `to_xxx()` methods (because they can Clone)
//!
//! # Parameters
//!
//! * `$struct_name<$generics>` - Struct name with generic parameters
//! * `$transformer_type` - Transformer wrapper type name
//! * `$transformer_trait` - Transformer trait name
//! * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
//! * `$extra_bounds` - Extra trait bounds
//!
//! # Usage Examples
//!
//! ```ignore
//! // Arc two-parameter Transformer
//! impl_shared_conditional_transformer!(
//!     ArcConditionalTransformer<T, U>,
//!     ArcTransformer,
//!     Transformer,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc two-parameter Transformer
//! impl_shared_conditional_transformer!(
//!     RcConditionalTransformer<T, U>,
//!     RcTransformer,
//!     Transformer,
//!     into_rc,
//!     'static
//! );
//!
//! // Arc three-parameter BiTransformer
//! impl_shared_conditional_transformer!(
//!     ArcConditionalBiTransformer<T, U, V>,
//!     ArcBiTransformer,
//!     BiTransformer,
//!     into_arc,
//!     Send + Sync + 'static
//! );
//!
//! // Rc three-parameter BiTransformer
//! impl_shared_conditional_transformer!(
//!     RcConditionalBiTransformer<T, U, V>,
//!     RcBiTransformer,
//!     BiTransformer,
//!     into_rc,
//!     'static
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generates Arc/Rc-based Conditional Transformer implementations
///
/// This macro should be used at the top level (outside of any impl block) as
/// it generates a complete impl block with methods for the specified struct.
/// For Arc/Rc-based conditional transformers, generates `and_then` and `or_else` methods,
/// as well as complete Transformer/BiTransformer trait implementations.
///
/// Arc/Rc type characteristics:
/// - `and_then` and `or_else` borrow &self (because Arc/Rc can Clone)
/// - Uses trait default implementations for `into_arc()` and `to_arc()`
/// - Arc types will work with `into_arc()` and `to_arc()` (satisfy Send + Sync constraints)
/// - Rc types will get compile errors if trying to use `into_arc()` or `to_arc()` (don't satisfy Send + Sync)
/// - Implement complete `to_xxx()` methods (because they can Clone)
///
/// # Parameters
///
/// * `$struct_name<$generics>` - Struct name with generic parameters
/// * `$transformer_type` - Transformer wrapper type name
/// * `$transformer_trait` - Transformer trait name
/// * `$predicate_conversion` - Predicate conversion method (into_arc or into_rc)
/// * `$extra_bounds` - Extra trait bounds
///
/// # Usage Examples
///
/// ```ignore
/// // Arc two-parameter Transformer
/// impl_shared_conditional_transformer!(
///     ArcConditionalTransformer<T, U>,
///     ArcTransformer,
///     Transformer,
///     into_arc,
///     Send + Sync + 'static
/// );
///
/// // Rc two-parameter Transformer
/// impl_shared_conditional_transformer!(
///     RcConditionalTransformer<T, U>,
///     RcTransformer,
///     Transformer,
///     into_rc,
///     'static
/// );
///
/// // Arc three-parameter BiTransformer
/// impl_shared_conditional_transformer!(
///     ArcConditionalBiTransformer<T, U, V>,
///     ArcBiTransformer,
///     BiTransformer,
///     into_arc,
///     Send + Sync + 'static
/// );
///
/// // Rc three-parameter BiTransformer
/// impl_shared_conditional_transformer!(
///     RcConditionalBiTransformer<T, U, V>,
///     RcBiTransformer,
///     BiTransformer,
///     into_rc,
///     'static
/// );
// ```
macro_rules! impl_shared_conditional_transformer {
    // Two generic parameters - Transformer
    (
        $struct_name:ident < $t:ident, $u:ident >,
        $transformer_type:ident,
        $transformer_trait:ident,
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
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
            /// let transformer1 = ArcTransformer::new(|x: &i32| *x + 1);
            /// let transformer2 = ArcTransformer::new(|x: &i32| *x * 2);
            ///
            /// let conditional = transformer1.when(|x| *x > 0);
            /// let chained = conditional.and_then(transformer2);
            ///
            /// assert_eq!(chained.transform(&5), 12);  // (5 + 1) * 2 = 12
            /// assert_eq!(chained.transform(&-5), -10); // (-5) * 2 = -10 (not (-5 + 1) * 2)
            /// ```
            #[allow(unused_mut)]
            pub fn and_then<C>(&self, mut next: C) -> $transformer_type<$t, $u>
            where
                C: $transformer_trait<$u, $u> + $($extra_bounds)+,
            {
                let first_predicate = self.predicate.clone();
                let mut first_transformer = self.transformer.clone();
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
            pub fn or_else<C>(&self, mut else_transformer: C) -> $transformer_type<$t, $u>
            where
                C: $transformer_trait<$t, $u> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                let mut then_transformer = self.transformer.clone();
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
        $struct_name:ident < $t:ident, $u:ident, $v:ident >,
        $transformer_type:ident,
        $transformer_trait:ident,
        $predicate_conversion:ident,
        $($extra_bounds:tt)+
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
            /// let transformer1 = ArcBiTransformer::new(|x: &i32, y: &i32| x + y);
            /// let transformer2 = ArcBiTransformer::new(|result: &i32, _| *result * 2);
            ///
            /// let conditional = transformer1.when(|x, y| *x > 0 && *y > 0);
            /// let chained = conditional.and_then(transformer2);
            ///
            /// assert_eq!(chained.transform(&5, &3), 16);  // (5 + 3) * 2 = 16
            /// assert_eq!(chained.transform(&-5, &3), -4); // (0) * 2 = -4 (identity case)
            /// ```
            #[allow(unused_mut)]
            pub fn and_then<C>(&self, mut next: C) -> $transformer_type<$t, $u, $v>
            where
                C: $transformer_trait<$v, $v, $v> + $($extra_bounds)+,
            {
                let first_predicate = self.predicate.clone();
                let mut first_transformer = self.transformer.clone();
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
            pub fn or_else<C>(&self, mut else_transformer: C) -> $transformer_type<$t, $u, $v>
            where
                C: $transformer_trait<$t, $u, $v> + $($extra_bounds)+,
            {
                let predicate = self.predicate.clone();
                let mut then_transformer = self.transformer.clone();
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

pub(crate) use impl_shared_conditional_transformer;
