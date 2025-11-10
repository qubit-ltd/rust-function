/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # BiMutatingFunctionOnce Types
//!
//! Provides Rust implementations of consuming bi-mutating-function traits similar to
//! Rust's `FnOnce(&mut T, &mut U) -> R` trait, but with value-oriented semantics for functional
//! programming patterns with two mutable input references.
//!
//! This module provides the `BiMutatingFunctionOnce<T, U, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxBiMutatingFunctionOnce`]: Single ownership, one-time use
//!
//! # Author
//!
//! Haixing Hu

use crate::{
    functions::{
        macros::{
            impl_box_conditional_function,
            impl_box_function_methods,
            impl_conditional_function_debug_display,
            impl_function_common_methods,
            impl_function_constant_method,
            impl_function_debug_display,
        },
        mutating_function_once::MutatingFunctionOnce,
    },
    macros::box_conversions::impl_box_once_conversions,
    predicates::bi_predicate::{
        BiPredicate,
        BoxBiPredicate,
    },
};

// ============================================================================
// Core Trait
// ============================================================================

/// BiMutatingFunctionOnce trait - consuming bi-mutating-function that takes
/// mutable references
///
/// Defines the behavior of a consuming bi-mutating-function: computing a value of
/// type `R` from mutable references to types `T` and `U` by taking ownership of self.
/// This trait is analogous to `FnOnce(&mut T, &mut U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (mutable reference)
/// * `U` - The type of the second input value (mutable reference)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait BiMutatingFunctionOnce<T, U, R> {
    /// Computes output from two mutable references, consuming self
    ///
    /// # Parameters
    ///
    /// * `first` - Mutable reference to the first input value
    /// * `second` - Mutable reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(self, first: &mut T, second: &mut U) -> R;

    /// Converts to BoxBiMutatingFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiMutatingFunctionOnce<T, U, R>`
    fn into_box(self) -> BoxBiMutatingFunctionOnce<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiMutatingFunctionOnce::new(move |t: &mut T, u: &mut U| self.apply(t, u))
    }

    /// Converts bi-mutating-function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&mut T, &mut U) -> R`
    fn into_fn(self) -> impl FnOnce(&mut T, &mut U) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        move |t: &mut T, u: &mut U| self.apply(t, u)
    }

    /// Converts bi-mutating-function to a boxed function pointer
    ///
    /// **📌 Borrows `&self`**: The original bi-function remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a boxed function pointer that implements `FnOnce(&mut T, &mut U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::BiMutatingFunctionOnce;
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let func = swap_and_sum.to_box();
    /// let mut a = 20;
    /// let mut b = 22;
    /// assert_eq!(func.apply(&mut a, &mut b), 42);
    /// ```
    fn to_box(&self) -> BoxBiMutatingFunctionOnce<T, U, R>
    where
        Self: Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Converts bi-mutating-function to a closure
    ///
    /// **📌 Borrows `&self`**: The original bi-function remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&mut T, &mut U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::BiMutatingFunctionOnce;
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let func = swap_and_sum.to_fn();
    /// let mut a = 20;
    /// let mut b = 22;
    /// assert_eq!(func(&mut a, &mut b), 42);
    /// ```
    fn to_fn(&self) -> impl FnOnce(&mut T, &mut U) -> R
    where
        Self: Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }
}

// ============================================================================
// BoxBiMutatingFunctionOnce - Box<dyn FnOnce(&mut T, &mut U) -> R>
// ============================================================================

/// BoxBiMutatingFunctionOnce - consuming bi-mutating-function wrapper based on
/// `Box<dyn FnOnce>`
///
/// A bi-mutating-function wrapper that provides single ownership with one-time use
/// semantics. Consumes self and borrows both input values mutably.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(&mut T, &mut U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxBiMutatingFunctionOnce<T, U, R> {
    function: Box<dyn FnOnce(&mut T, &mut U) -> R>,
    name: Option<String>,
}

// Implement BoxBiMutatingFunctionOnce
impl<T, U, R> BoxBiMutatingFunctionOnce<T, U, R>
where
    T: 'static,
    U: 'static,
    R: 'static,
{
    // Generate new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxBiMutatingFunctionOnce<T, U, R>,
        (FnOnce(&mut T, &mut U) -> R + 'static),
        |f| Box::new(f)
    );

    // Generate when(), and_then()
    impl_box_function_methods!(
        BoxBiMutatingFunctionOnce<T, U, R>,
        BoxConditionalBiMutatingFunctionOnce,
        MutatingFunctionOnce
    );
}

// Implement constant method for BoxBiMutatingFunctionOnce
impl_function_constant_method!(BoxBiMutatingFunctionOnce<T, U, R>);

// Use macro to generate Debug and Display implementations
impl_function_debug_display!(BoxBiMutatingFunctionOnce<T, U, R>);

// Implement BiMutatingFunctionOnce trait for BoxBiMutatingFunctionOnce
impl<T, U, R> BiMutatingFunctionOnce<T, U, R> for BoxBiMutatingFunctionOnce<T, U, R> {
    fn apply(self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_fn(), to_box()
    impl_box_once_conversions!(
        BoxBiMutatingFunctionOnce<T, U, R>,
        BiMutatingFunctionOnce,
        FnOnce(&mut T, &mut U) -> R
    );
}

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

/// Implement BiMutatingFunctionOnce<T, U, R> for any type that implements
/// FnOnce(&mut T, &mut U) -> R
///
/// This allows once-callable closures and function pointers to be used
/// directly with our BiMutatingFunctionOnce trait without wrapping.
///
/// # Examples
///
/// ```rust
/// use prism3_function::BiMutatingFunctionOnce;
///
/// fn swap_and_sum(x: &mut i32, y: &mut i32) -> i32 {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// }
///
/// let mut a = 20;
/// let mut b = 22;
/// assert_eq!(swap_and_sum.apply(&mut a, &mut b), 42);
/// assert_eq!(a, 22);
/// assert_eq!(b, 20);
///
/// let owned_a = 20;
/// let owned_b = 22;
/// let swapper = |x: &mut i32, y: &mut i32| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// };
/// let mut a = owned_a;
/// let mut b = owned_b;
/// assert_eq!(swapper.apply(&mut a, &mut b), 42);
/// assert_eq!(a, 22);
/// assert_eq!(b, 20);
/// ```
///
/// # Author
///
/// Haixing Hu
impl<F, T, U, R> BiMutatingFunctionOnce<T, U, R> for F
where
    F: FnOnce(&mut T, &mut U) -> R,
    T: 'static,
    U: 'static,
    R: 'static,
{
    fn apply(self, first: &mut T, second: &mut U) -> R {
        self(first, second)
    }

    fn into_box(self) -> BoxBiMutatingFunctionOnce<T, U, R>
    where
        Self: Sized + 'static,
    {
        BoxBiMutatingFunctionOnce::new(self)
    }

    fn into_fn(self) -> impl FnOnce(&mut T, &mut U) -> R
    where
        Self: Sized + 'static,
    {
        move |first: &mut T, second: &mut U| -> R { self(first, second) }
    }

    // use the default implementation of to_box() from BiMutatingFunctionOnce trait

    fn to_fn(&self) -> impl FnOnce(&mut T, &mut U) -> R
    where
        Self: Clone + 'static,
    {
        self.clone()
    }
}

// ============================================================================
// FnBiMutatingFunctionOnceOps - Extension trait for FnOnce(&mut T, &mut U) -> R bi-functions
// ============================================================================

/// Extension trait for closures implementing `FnOnce(&mut T, &mut U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for one-time use
/// bi-mutating-function closures and function pointers without requiring explicit
/// wrapping in `BoxBiMutatingFunctionOnce`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `FnOnce(&mut T, &mut U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `BiMutatingFunctionOnce<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxBiMutatingFunctionOnce` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use prism3_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps};
///
/// let swap_and_sum = |x: &mut i32, y: &mut i32| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// };
/// let double = |x: i32| x * 2;
///
/// let composed = swap_and_sum.and_then(double);
/// let mut a = 3;
/// let mut b = 5;
/// assert_eq!(composed.apply(&mut a, &mut b), 16); // (5 + 3) * 2 = 16
/// ```
///
/// ## Conditional execution with when
///
/// ```rust
/// use prism3_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps};
///
/// let swap_and_sum = |x: &mut i32, y: &mut i32| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// };
/// let multiply = |x: &mut i32, y: &mut i32| {
///     *x *= *y;
///     *x
/// };
///
/// let conditional = swap_and_sum.when(|x: &mut i32, y: &mut i32| *x > 0 && *y > 0).or_else(multiply);
/// let mut a = 5;
/// let mut b = 3;
/// assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum executed
///
/// let conditional2 = swap_and_sum.when(|x: &mut i32, y: &mut i32| *x > 0 && *y > 0).or_else(multiply);
/// let mut a = -5;
/// let mut b = 3;
/// assert_eq!(conditional2.apply(&mut a, &mut b), -15); // multiply executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnBiMutatingFunctionOnceOps<T, U, R>:
    FnOnce(&mut T, &mut U) -> R + Sized + 'static
{
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-mutating-function that applies this bi-mutating-function first,
    /// then applies the after function to the result. Consumes self and
    /// returns a `BoxBiMutatingFunctionOnce`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after function
    /// * `F` - The type of the after function (must implement Function<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The function to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** Since this is a
    ///   `FnOnce` bi-mutating-function, the parameter will be consumed. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxFunction<R, S>`
    ///   - An `RcFunction<R, S>`
    ///   - An `ArcFunction<R, S>`
    ///   - Any type implementing `Function<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxBiMutatingFunctionOnce<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps,
    ///     BoxFunction};
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let to_string = BoxFunction::new(|x: i32| x.to_string());
    ///
    /// // to_string is moved and consumed
    /// let composed = swap_and_sum.and_then(to_string);
    /// let mut a = 20;
    /// let mut b = 22;
    /// assert_eq!(composed.apply(&mut a, &mut b), "42");
    /// // to_string.apply(10); // Would not compile - moved
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxBiMutatingFunctionOnce<T, U, S>
    where
        S: 'static,
        F: crate::functions::function::Function<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiMutatingFunctionOnce::new(move |t: &mut T, u: &mut U| after.apply(&self(t, u)))
    }

    /// Creates a conditional bi-mutating-function
    ///
    /// Returns a bi-mutating-function that only executes when a bi-predicate is
    /// satisfied. You must call `or_else()` to provide an alternative
    /// bi-mutating-function for when the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. **Note: This parameter is passed
    ///   by value and will transfer ownership.** If you need to preserve the
    ///   original bi-predicate, clone it first (if it implements `Clone`).
    ///   Can be:
    ///   - A closure: `|x: &mut T, y: &mut U| -> bool`
    ///   - A function pointer: `fn(&mut T, &mut U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalBiMutatingFunctionOnce<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use prism3_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps};
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let multiply = |x: &mut i32, y: &mut i32| {
    ///     *x *= *y;
    ///     *x
    /// };
    /// let conditional = swap_and_sum.when(|x: &mut i32, y: &mut i32| *x > 0)
    ///     .or_else(multiply);
    ///
    /// let mut a = 5;
    /// let mut b = 3;
    /// assert_eq!(conditional.apply(&mut a, &mut b), 8);
    /// ```
    ///
    /// ## Preserving bi-predicate with clone
    ///
    /// ```rust
    /// use prism3_function::{BiMutatingFunctionOnce, FnBiMutatingFunctionOnceOps,
    ///     RcBiPredicate};
    ///
    /// let swap_and_sum = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let both_positive = RcBiPredicate::new(|x: &mut i32, y: &mut i32|
    ///     *x > 0 && *y > 0);
    ///
    /// // Clone to preserve original bi-predicate
    /// let conditional = swap_and_sum.when(both_positive.clone())
    ///     .or_else(|x: &mut i32, y: &mut i32| *x * *y);
    ///
    /// let mut a = 5;
    /// let mut b = 3;
    /// assert_eq!(conditional.apply(&mut a, &mut b), 8);
    ///
    /// // Original bi-predicate still usable
    /// let mut test_a = 5;
    /// let mut test_b = 3;
    /// assert!(both_positive.test(&mut test_a, &mut test_b));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalBiMutatingFunctionOnce<T, U, R>
    where
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiMutatingFunctionOnce::new(self).when(predicate)
    }
}

/// Blanket implementation of FnBiMutatingFunctionOnceOps for all closures
///
/// Automatically implements `FnBiMutatingFunctionOnceOps<T, U, R>` for any type that
/// implements `FnOnce(&mut T, &mut U) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, U, R, F> FnBiMutatingFunctionOnceOps<T, U, R> for F
where
    F: FnOnce(&mut T, &mut U) -> R + 'static,
    T: 'static,
    U: 'static,
    R: 'static,
{
    // empty
}

// ============================================================================
// BoxConditionalBiMutatingFunctionOnce - Box-based Conditional BiMutatingFunction
// ============================================================================

/// BoxConditionalBiMutatingFunctionOnce struct
///
/// A conditional consuming bi-mutating-function that only executes when a bi-predicate
/// is satisfied. Uses `BoxBiMutatingFunctionOnce` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiMutatingFunctionOnce::when()` and
/// is designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalBiMutatingFunctionOnce<T, U, R> {
    function: BoxBiMutatingFunctionOnce<T, U, R>,
    predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalBiMutatingFunctionOnce
impl_box_conditional_function!(
    BoxConditionalBiMutatingFunctionOnce<T, U, R>,
    BoxBiMutatingFunctionOnce,
    BiMutatingFunctionOnce
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(BoxConditionalBiMutatingFunctionOnce<T, U, R>);
