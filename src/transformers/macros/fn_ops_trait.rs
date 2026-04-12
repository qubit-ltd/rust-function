/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Fn Ops Trait Macro
//!
//! Generate extension traits and implementations for closure types
//!
//! This macro generates extension traits for closure types that implement
//! `Fn` or `FnMut`, providing `and_then` and `when` methods without requiring
//! explicit wrapping as `BoxTransformer`, `RcTransformer`, or `ArcTransformer`.
//!
//! # Parameters
//!
//! * `$fn_signature` - Closure signature (in parentheses, without constraints)
//!   Examples: `(Fn(&T) -> R)`, `(FnMut(&T) -> R)`, `(FnMut(&mut T) -> R)`
//! * `$trait_name` - Name of the extension trait (e.g., `FnTransformerOps`,
//!   `FnStatefulTransformerOps`)
//! * `$box_type` - Box wrapper type (e.g., `BoxTransformer`, `BoxStatefulTransformer`)
//! * `$chained_transformer_trait` - The name of the transformer trait that chained
//!   after the execution of this transformer (e.g., Transformer, StatefulBiTransformer)
//! * `$conditional_type` - Conditional transformer type (e.g., BoxConditionalTransformer)
//!
//! # Implementation Notes
//!
//! The macro uses mutable references (`&mut`) uniformly because in Rust,
//! `&mut T` can be automatically dereferenced to `&T`. This allows both `Fn`
//! and `FnMut` closures to use the same implementation logic, simplifying
//! the code and improving performance (avoiding additional boxing operations).
//!
//! # Usage Examples
//!
//! ```ignore
//! // Generate extension trait for Fn(&T) -> R
//! impl_fn_ops_trait!(
//!     (Fn(&T) -> R),
//!     FnTransformerOps,
//!     BoxFunction,
//!     Transformer,
//!     BoxConditionalTransformer
//! );
//!
//! // Generate extension trait for FnMut(&T) -> R
//! impl_fn_ops_trait!(
//!     (FnMut(&T) -> R),
//!     FnStatefulTransformerOps,
//!     BoxStatefulTransformer,
//!     StatefulTransformer,
//!     BoxConditionalStatefulTransformer
//! );
//!
//! // Generate extension trait for FnMut(&mut T) -> R (consuming functions)
//! impl_fn_ops_trait!(
//!     (FnMut(&mut T) -> R),
//!     FnMutatingTransformerOps,
//!     BoxMutatingTransformer,
//!     MutatingTransformer,
//!     BoxConditionalMutatingTransformer
//! );
//! ```
//!
//! # Author
//!
//! Haixing Hu

/// Generate extension traits and implementations for closure types
///
/// This macro generates an extension trait that provides composition methods
/// (`and_then`, `when`) for closures implementing the specified
/// closure trait, without requiring explicit wrapping.
///
/// # Unified Implementation Strategy
///
/// The macro uses a unified implementation approach, passing intermediate
/// results using mutable references (`&mut`). This is because:
/// 1. In Rust, `&mut T` can be automatically dereferenced to `&T`
/// 2. Avoids code duplication and simplifies the macro implementation
/// 3. Better performance by avoiding additional boxing operations
/// 4. Uses `#[allow(unused_mut)]` to suppress unnecessary mutability warnings
///
/// # Parameters
///
/// * `$fn_signature` - Closure signature (in parentheses, without constraints)
/// * `$trait_name` - Name of the extension trait
/// * `$box_type` - Box wrapper type
/// * `$chained_function_trait` - The name of the function trait that chained
///   after the execution of this function (e.g., Function, BiFunction)
/// * `$conditional_type` - Conditional function type
///
/// # Generated Code
///
/// Generates a trait definition and a blanket implementation, containing:
/// - `and_then<S, F>` - Chain composition method
/// - `when<P>` - Conditional execution method
///
/// # Examples
///
/// ```ignore
/// // Fn(&T) -> R version
/// impl_fn_ops_trait!(
///     (Fn(&T) -> R),
///     FnFunctionOps,
///     BoxFunction,
///     Function,
///     BoxConditionalFunction
/// );
///
/// // FnMut(&T) -> R version
/// impl_fn_ops_trait!(
///     (FnMut(&T) -> R),
///     FnStatefulFunctionOps,
///     BoxStatefulFunction,
///     StatefulFunction,
///     BoxConditionalStatefulFunction
/// );
///
/// // FnMut(&mut T) -> R version (consuming functions)
/// impl_fn_ops_trait!(
///     (FnMut(&mut T) -> R),
///     FnMutatingFunctionOps,
///     BoxMutatingFunction,
///     MutatingFunction,
///     BoxConditionalMutatingFunction
/// );
/// ```
///
/// # Author
///
/// Haixing Hu
#[macro_export]
macro_rules! impl_fn_ops_trait {
    // Unified implementation - accepts closure signature (without constraints)
    (
        ($($fn_signature:tt)+),
        $trait_name:ident,
        $box_type:ident,
        $chained_function_trait:ident,
        $conditional_type:ident
    ) => {
        /// Extension trait for closures implementing the base function trait
        ///
        /// Provides composition methods (`and_then`, `when`) for closures
        /// and function pointers without requiring explicit wrapping.
        ///
        /// This trait is automatically implemented for all closures and function
        /// pointers that implement the base function trait.
        ///
        /// # Design Rationale
        ///
        /// While closures automatically implement the base function trait through blanket
        /// implementation, they don't have access to instance methods like `and_then`,
        /// and `when`. This extension trait provides those methods,
        /// returning the appropriate Box-based function type for maximum flexibility.
        ///
        /// # Examples
        ///
        /// ## Chain composition with and_then
        ///
        /// ```rust
        /// use qubit_function::{Function, FnFunctionOps};
        ///
        /// let double = |x: i32| x * 2;
        /// let to_string = |x: i32| x.to_string();
        ///
        /// let composed = double.and_then(to_string);
        /// assert_eq!(composed.apply(21), "42");
        /// ```
        ///
        /// ## Conditional transformation with when
        ///
        /// ```rust
        /// use qubit_function::{Function, FnFunctionOps};
        ///
        /// let double = |x: i32| x * 2;
        /// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
        ///
        /// assert_eq!(conditional.apply(5), 10);
        /// assert_eq!(conditional.apply(-5), 5);
        /// ```
        ///
        /// # Author
        ///
        /// Haixing Hu
        pub trait $trait_name<T, R>: $($fn_signature)+ + Sized {
            /// Chain composition - applies self first, then after
            ///
            /// Creates a new function that applies this function first, then
            /// applies the after function to the result. Consumes self and returns
            /// a Box-based function.
            ///
            /// # Type Parameters
            ///
            /// * `S` - The output type of the after function
            /// * `F` - The type of the after function (must implement the function trait)
            ///
            /// # Parameters
            ///
            /// * `after` - The function to apply after self. **Note: This parameter
            ///   is passed by value and will transfer ownership.** If you need to
            ///   preserve the original function, clone it first (if it implements
            ///   `Clone`). Can be:
            ///   - A closure
            ///   - A function pointer
            ///   - A Box-based function
            ///   - An Rc-based function
            ///   - An Arc-based function
            ///   - Any type implementing the function trait
            ///
            /// # Returns
            ///
            /// A new Box-based function representing the composition
            ///
            /// # Examples
            ///
            /// ## Direct value passing (ownership transfer)
            ///
            /// ```rust
            /// use qubit_function::{Function, FnFunctionOps, BoxFunction};
            ///
            /// let double = |x: i32| x * 2;
            /// let to_string = BoxFunction::new(|x: i32| x.to_string());
            ///
            /// // to_string is moved here
            /// let composed = double.and_then(to_string);
            /// assert_eq!(composed.apply(21), "42");
            /// // to_string.apply(5); // Would not compile - moved
            /// ```
            ///
            /// ## Preserving original with clone
            ///
            /// ```rust
            /// use qubit_function::{Function, FnFunctionOps, BoxFunction};
            ///
            /// let double = |x: i32| x * 2;
            /// let to_string = BoxFunction::new(|x: i32| x.to_string());
            ///
            /// // Clone to preserve original
            /// let composed = double.and_then(to_string.clone());
            /// assert_eq!(composed.apply(21), "42");
            ///
            /// // Original still usable
            /// assert_eq!(to_string.apply(5), "5");
            /// ```
            #[allow(unused_mut)]
            #[inline]
            fn and_then<S, F>(mut self, mut after: F) -> $box_type<T, S>
            where
                Self: 'static,
                S: 'static,
                F: $chained_function_trait<R, S> + 'static,
                T: 'static,
                R: 'static,
            {
                $box_type::new(move |x| {
                  let mut r = self(x);
                  after.apply(&mut r)
                })
            }

            /// Creates a conditional function
            ///
            /// Returns a function that only executes when a predicate is satisfied.
            /// You must call `or_else()` to provide an alternative function for when
            /// the condition is not satisfied.
            ///
            /// # Parameters
            ///
            /// * `predicate` - The condition to check. **Note: This parameter is passed
            ///   by value and will transfer ownership.** If you need to preserve the
            ///   original predicate, clone it first (if it implements `Clone`). Can be:
            ///   - A closure: `|x: &T| -> bool`
            ///   - A function pointer: `fn(&T) -> bool`
            ///   - A `BoxPredicate<T>`
            ///   - An `RcPredicate<T>`
            ///   - An `ArcPredicate<T>`
            ///   - Any type implementing `Predicate<T>`
            ///
            /// # Returns
            ///
            /// Returns the appropriate conditional function type
            ///
            /// # Examples
            ///
            /// ## Basic usage with or_else
            ///
            /// ```rust
            /// use qubit_function::{Function, FnFunctionOps};
            ///
            /// let double = |x: i32| x * 2;
            /// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
            ///
            /// assert_eq!(conditional.apply(5), 10);
            /// assert_eq!(conditional.apply(-5), 5);
            /// ```
            ///
            /// ## Preserving predicate with clone
            ///
            /// ```rust
            /// use qubit_function::{Function, FnFunctionOps, BoxPredicate};
            ///
            /// let double = |x: i32| x * 2;
            /// let is_positive = BoxPredicate::new(|x: &i32| *x > 0);
            ///
            /// // Clone to preserve original predicate
            /// let conditional = double.when(is_positive.clone())
            ///     .or_else(|x: i32| -x);
            ///
            /// assert_eq!(conditional.apply(5), 10);
            ///
            /// // Original predicate still usable
            /// assert!(is_positive.test(&3));
            /// ```
            #[inline]
            fn when<P>(self, predicate: P) -> $conditional_type<T, R>
            where
                Self: 'static,
                P: Predicate<T> + 'static,
                T: 'static,
                R: 'static,
            {
                $box_type::new(self).when(predicate)
            }
        }

        /// Blanket implementation for all closures
        ///
        /// Automatically implements the extension trait for any type that
        /// implements the base function trait.
        ///
        /// # Author
        ///
        /// Haixing Hu
        impl<T, R, F> $trait_name<T, R> for F where F: $($fn_signature)+ {}
    };
}

pub(crate) use impl_fn_ops_trait;
