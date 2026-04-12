/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # BiMutatingFunction Types
//!
//! Provides Rust implementations of bi-mutating-function traits for performing
//! operations that accept two mutable references and return a result.
//!
//! It is similar to the `Fn(&mut T, &mut U) -> R` trait in the standard library.
//!
//! This module provides the `BiMutatingFunction<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiMutatingFunction`]: Single ownership, not cloneable
//! - [`ArcBiMutatingFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcBiMutatingFunction`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu

use std::rc::Rc;
use std::sync::Arc;

use crate::functions::{
    bi_mutating_function_once::BoxBiMutatingFunctionOnce,
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_clone,
        impl_conditional_function_debug_display,
        impl_function_clone,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
        impl_shared_conditional_function,
        impl_shared_function_methods,
    },
    mutating_function::MutatingFunction,
};
use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_closure_trait,
    impl_rc_conversions,
};
use crate::predicates::bi_predicate::{
    ArcBiPredicate,
    BiPredicate,
    BoxBiPredicate,
    RcBiPredicate,
};

// ============================================================================
// Core Trait
// ============================================================================

/// BiMutatingFunction trait - performs operations on two mutable references
///
/// Defines the behavior of a bi-mutating-function: computing a value of type `R`
/// from mutable references to types `T` and `U`, potentially modifying both inputs.
/// This is analogous to `Fn(&mut T, &mut U) -> R` in Rust's standard library.
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
pub trait BiMutatingFunction<T, U, R> {
    /// Applies the bi-mutating-function to two mutable references and returns a result
    ///
    /// # Parameters
    ///
    /// * `first` - Mutable reference to the first input value
    /// * `second` - Mutable reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, first: &mut T, second: &mut U) -> R;

    /// Converts to BoxBiMutatingFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxBiMutatingFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiMutatingFunction<T, U, R>`
    fn into_box(self) -> BoxBiMutatingFunction<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiMutatingFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts to RcBiMutatingFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcBiMutatingFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcBiMutatingFunction<T, U, R>`
    fn into_rc(self) -> RcBiMutatingFunction<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        RcBiMutatingFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts to ArcBiMutatingFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates
    /// an `ArcBiMutatingFunction`. Types can override this method to provide
    /// more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcBiMutatingFunction<T, U, R>`
    fn into_arc(self) -> ArcBiMutatingFunction<T, U, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        ArcBiMutatingFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts bi-mutating-function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures `self`
    /// and calls its `apply` method. Types can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `Fn(&mut T, &mut U) -> R`
    fn into_fn(self) -> impl Fn(&mut T, &mut U) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        move |t, u| self.apply(t, u)
    }

    /// Converts to BiMutatingFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable after calling this method.
    ///
    /// Converts a reusable bi-mutating-function to a one-time bi-mutating-function that consumes itself on use.
    /// This enables passing `BiMutatingFunction` to functions that require `BiMutatingFunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiMutatingFunctionOnce<T, U, R>`
    fn into_once(self) -> BoxBiMutatingFunctionOnce<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiMutatingFunctionOnce::new(move |t, u| self.apply(t, u))
    }

    /// Non-consuming conversion to `BoxBiMutatingFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_box`.
    fn to_box(&self) -> BoxBiMutatingFunction<T, U, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcBiMutatingFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_rc`.
    fn to_rc(&self) -> RcBiMutatingFunction<T, U, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcBiMutatingFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_arc`.
    fn to_arc(&self) -> ArcBiMutatingFunction<T, U, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a boxed function using `&self`.
    ///
    /// Returns a `Box<dyn Fn(&mut T, &mut U) -> R>` that clones `self` and calls
    /// `apply` inside the boxed closure.
    fn to_fn(&self) -> impl Fn(&mut T, &mut U) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to BiMutatingFunctionOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current bi-function and converts the clone to a one-time bi-function.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiMutatingFunctionOnce<T, U, R>`
    fn to_once(&self) -> BoxBiMutatingFunctionOnce<T, U, R>
    where
        Self: Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_once()
    }
}

// ============================================================================
// BoxBiMutatingFunction - Box<dyn Fn(&mut T, &mut U) -> R>
// ============================================================================

/// BoxBiMutatingFunction - bi-mutating-function wrapper based on `Box<dyn Fn>`
///
/// A bi-mutating-function wrapper that provides single ownership with reusable
/// computation. Borrows both inputs mutably and can be called multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(&mut T, &mut U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (borrows inputs mutably each time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxBiMutatingFunction<T, U, R> {
    function: Box<dyn Fn(&mut T, &mut U) -> R>,
    name: Option<String>,
}

// Implement BoxBiMutatingFunction
impl<T, U, R> BoxBiMutatingFunction<T, U, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxBiMutatingFunction<T, U, R>,
        (Fn(&mut T, &mut U) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then()
    impl_box_function_methods!(
        BoxBiMutatingFunction<T, U, R>,
        BoxConditionalBiMutatingFunction,
        MutatingFunction
    );
}

// Implement BiMutatingFunction trait for BoxBiMutatingFunction
impl<T, U, R> BiMutatingFunction<T, U, R> for BoxBiMutatingFunction<T, U, R> {
    fn apply(&self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxBiMutatingFunction<T, U, R>,
        RcBiMutatingFunction,
        Fn(&mut T, &mut U) -> R,
        BoxBiMutatingFunctionOnce
    );
}

// Implement constant method for BoxBiMutatingFunction
impl_function_constant_method!(BoxBiMutatingFunction<T, U, R>);

// Implement Debug and Display for BoxBiMutatingFunction
impl_function_debug_display!(BoxBiMutatingFunction<T, U, R>);

// ============================================================================
// RcBiMutatingFunction - Rc<dyn Fn(&mut T, &mut U) -> R>
// ============================================================================

/// RcBiMutatingFunction - single-threaded bi-mutating-function wrapper
///
/// A single-threaded, clonable bi-mutating-function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(&mut T, &mut U) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (borrows inputs mutably each time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct RcBiMutatingFunction<T, U, R> {
    function: Rc<dyn Fn(&mut T, &mut U) -> R>,
    name: Option<String>,
}

impl<T, U, R> RcBiMutatingFunction<T, U, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcBiMutatingFunction<T, U, R>,
        (Fn(&mut T, &mut U) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_shared_function_methods!(
        RcBiMutatingFunction<T, U, R>,
        RcConditionalBiMutatingFunction,
        into_rc,
        MutatingFunction,
        'static
    );
}

// Implement BiMutatingFunction trait for RcBiMutatingFunction
impl<T, U, R> BiMutatingFunction<T, U, R> for RcBiMutatingFunction<T, U, R> {
    fn apply(&self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_rc_conversions!(
        RcBiMutatingFunction<T, U, R>,
        BoxBiMutatingFunction,
        BoxBiMutatingFunctionOnce,
        Fn(first: &mut T, second: &mut U) -> R
    );
}

// Implement constant method for RcBiMutatingFunction
impl_function_constant_method!(RcBiMutatingFunction<T, U, R>);

// Implement Debug and Display for RcBiMutatingFunction
impl_function_debug_display!(RcBiMutatingFunction<T, U, R>);

// Implement Clone for RcBiMutatingFunction
impl_function_clone!(RcBiMutatingFunction<T, U, R>);

// ============================================================================
// ArcBiMutatingFunction - Arc<dyn Fn(&mut T, &mut U) -> R + Send + Sync>
// ============================================================================

/// ArcBiMutatingFunction - thread-safe bi-mutating-function wrapper
///
/// A thread-safe, clonable bi-mutating-function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(&mut T, &mut U) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (borrows inputs mutably each time)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct ArcBiMutatingFunction<T, U, R> {
    function: Arc<dyn Fn(&mut T, &mut U) -> R + Send + Sync>,
    name: Option<String>,
}

impl<T, U, R> ArcBiMutatingFunction<T, U, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcBiMutatingFunction<T, U, R>,
        (Fn(&mut T, &mut U) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_shared_function_methods!(
        ArcBiMutatingFunction<T, U, R>,
        ArcConditionalBiMutatingFunction,
        into_arc,
        MutatingFunction,
        Send + Sync + 'static
    );
}

// Implement BiMutatingFunction trait for ArcBiMutatingFunction
impl<T, U, R> BiMutatingFunction<T, U, R> for ArcBiMutatingFunction<T, U, R> {
    fn apply(&self, first: &mut T, second: &mut U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_arc_conversions!(
        ArcBiMutatingFunction<T, U, R>,
        BoxBiMutatingFunction,
        RcBiMutatingFunction,
        BoxBiMutatingFunctionOnce,
        Fn(first: &mut T, second: &mut U) -> R
    );
}

// Implement constant method for ArcBiMutatingFunction
impl_function_constant_method!(ArcBiMutatingFunction<T, U, R>, Send + Sync + 'static);

// Implement Debug and Display for ArcBiMutatingFunction
impl_function_debug_display!(ArcBiMutatingFunction<T, U, R>);

// Implement Clone for ArcBiMutatingFunction
impl_function_clone!(ArcBiMutatingFunction<T, U, R>);

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement BiMutatingFunction<T, U, R> for any type that implements Fn(&mut T, &mut U) -> R
impl_closure_trait!(
    BiMutatingFunction<T, U, R>,
    apply,
    BoxBiMutatingFunctionOnce,
    Fn(first: &mut T, second: &mut U) -> R
);

// ============================================================================
// FnBiMutatingFunctionOps - Extension trait for Fn(&mut T, &mut U) -> R bi-functions
// ============================================================================

/// Extension trait for closures implementing `Fn(&mut T, &mut U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for bi-mutating-function
/// closures and function pointers without requiring explicit wrapping in
/// `BoxBiMutatingFunction`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `Fn(&mut T, &mut U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `BiMutatingFunction<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxBiMutatingFunction` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{BiMutatingFunction, FnBiMutatingFunctionOps};
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
/// use qubit_function::{BiMutatingFunction, FnBiMutatingFunctionOps};
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
///
/// let mut a = 5;
/// let mut b = 3;
/// assert_eq!(conditional.apply(&mut a, &mut b), 8);   // swap_and_sum: (3 + 5)
///
/// let mut a = -5;
/// let mut b = 3;
/// assert_eq!(conditional.apply(&mut a, &mut b), -15); // multiply: (-5 * 3)
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnBiMutatingFunctionOps<T, U, R>: Fn(&mut T, &mut U) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-mutating-function that applies this bi-mutating-function first,
    /// then applies the after function to the result. Consumes self and
    /// returns a `BoxBiMutatingFunction`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after function
    /// * `F` - The type of the after function (must implement Function<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The function to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original function, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxFunction<R, S>`
    ///   - An `RcFunction<R, S>`
    ///   - An `ArcFunction<R, S>`
    ///   - Any type implementing `Function<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxBiMutatingFunction<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use qubit_function::{BiMutatingFunction, FnBiMutatingFunctionOps,
    ///     BoxFunction};
    ///
    /// let swap = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let to_string = BoxFunction::new(|x: i32| x.to_string());
    ///
    /// // to_string is moved here
    /// let composed = swap.and_then(to_string);
    /// let mut a = 20;
    /// let mut b = 22;
    /// assert_eq!(composed.apply(&mut a, &mut b), "42");
    /// // to_string.apply(10); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with clone
    ///
    /// ```rust
    /// use qubit_function::{BiMutatingFunction, FnBiMutatingFunctionOps,
    ///     BoxFunction};
    ///
    /// let swap = |x: &mut i32, y: &mut i32| {
    ///     let temp = *x;
    ///     *x = *y;
    ///     *y = temp;
    ///     *x + *y
    /// };
    /// let to_string = BoxFunction::new(|x: i32| x.to_string());
    ///
    /// // Clone to preserve original
    /// let composed = swap.and_then(to_string.clone());
    /// let mut a = 20;
    /// let mut b = 22;
    /// assert_eq!(composed.apply(&mut a, &mut b), "42");
    ///
    /// // Original still usable
    /// assert_eq!(to_string.apply(&10), "10");
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxBiMutatingFunction<T, U, S>
    where
        Self: 'static,
        S: 'static,
        F: crate::functions::function::Function<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiMutatingFunction::new(move |t: &mut T, u: &mut U| after.apply(&self(t, u)))
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
    /// Returns `BoxConditionalBiMutatingFunction<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{BiMutatingFunction, FnBiMutatingFunctionOps};
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
    /// use qubit_function::{BiMutatingFunction, FnBiMutatingFunctionOps,
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
    fn when<P>(self, predicate: P) -> BoxConditionalBiMutatingFunction<T, U, R>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiMutatingFunction::new(self).when(predicate)
    }
}

/// Blanket implementation of FnBiMutatingFunctionOps for all closures
///
/// Automatically implements `FnBiMutatingFunctionOps<T, U, R>` for any type that
/// implements `Fn(&mut T, &mut U) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, U, R, F> FnBiMutatingFunctionOps<T, U, R> for F where F: Fn(&mut T, &mut U) -> R {}

// ============================================================================
// Type Aliases for BinaryMutatingOperator (BiMutatingFunction<T, U, R> where T == U)
// ============================================================================

/// Type alias for `BoxBiMutatingFunction<T, T, R>`
///
/// Represents a binary mutating function that takes two values of type `T` and produces
/// a value of type `R`, with single ownership semantics. Similar to Java's
/// `BiFunction<T, T, R>` but with mutable references.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxBinaryMutatingFunction, BiMutatingFunction};
///
/// let swap_and_sum: BoxBinaryMutatingFunction<i32, i32> = BoxBinaryMutatingFunction::new(|x, y| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// });
/// let mut a = 5;
/// let mut b = 10;
/// assert_eq!(swap_and_sum.apply(&mut a, &mut b), 15);
/// assert_eq!(a, 10);
/// assert_eq!(b, 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type BoxBinaryMutatingFunction<T, R> = BoxBiMutatingFunction<T, T, R>;

/// Type alias for `ArcBiMutatingFunction<T, T, R>`
///
/// Represents a thread-safe binary mutating function that takes two values of type `T`
/// and produces a value of type `R`. Similar to Java's `BiFunction<T, T, R>`
/// with shared, thread-safe ownership and mutable references.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcBinaryMutatingFunction, BiMutatingFunction};
///
/// let swap_and_sum: ArcBinaryMutatingFunction<i32, i32> = ArcBinaryMutatingFunction::new(|x, y| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// });
/// let swap_clone = swap_and_sum.clone();
/// let mut a = 5;
/// let mut b = 10;
/// assert_eq!(swap_and_sum.apply(&mut a, &mut b), 15);
/// assert_eq!(swap_clone.apply(&mut a, &mut b), 25);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type ArcBinaryMutatingFunction<T, R> = ArcBiMutatingFunction<T, T, R>;

/// Type alias for `RcBiMutatingFunction<T, T, R>`
///
/// Represents a single-threaded binary mutating function that takes two values of type `T`
/// and produces a value of type `R`. Similar to Java's `BiFunction<T, T, R>`
/// with shared, single-threaded ownership and mutable references.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RcBinaryMutatingFunction, BiMutatingFunction};
///
/// let swap_and_sum: RcBinaryMutatingFunction<i32, i32> = RcBinaryMutatingFunction::new(|x, y| {
///     let temp = *x;
///     *x = *y;
///     *y = temp;
///     *x + *y
/// });
/// let swap_clone = swap_and_sum.clone();
/// let mut a = 5;
/// let mut b = 10;
/// assert_eq!(swap_and_sum.apply(&mut a, &mut b), 15);
/// assert_eq!(swap_clone.apply(&mut a, &mut b), 25);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type RcBinaryMutatingFunction<T, R> = RcBiMutatingFunction<T, T, R>;

// ============================================================================
// BoxConditionalBiMutatingFunction - Box-based Conditional BiMutatingFunction
// ============================================================================

/// BoxConditionalBiMutatingFunction struct
///
/// A conditional bi-mutating-function that only executes when a bi-predicate is
/// satisfied. Uses `BoxBiMutatingFunction` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiMutatingFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiMutatingFunction**: Can be used anywhere a `BiMutatingFunction` is expected
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalBiMutatingFunction<T, U, R> {
    function: BoxBiMutatingFunction<T, U, R>,
    predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalBiMutatingFunction
impl_box_conditional_function!(
    BoxConditionalBiMutatingFunction<T, U, R>,
    BoxBiMutatingFunction,
    BiMutatingFunction
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(BoxConditionalBiMutatingFunction<T, U, R>);

// ============================================================================
// RcConditionalBiMutatingFunction - Rc-based Conditional BiMutatingFunction
// ============================================================================

/// RcConditionalBiMutatingFunction struct
///
/// A single-threaded conditional bi-mutating-function that only executes when a
/// bi-predicate is satisfied. Uses `RcBiMutatingFunction` and `RcBiPredicate` for
/// shared ownership within a single thread.
///
/// This type is typically created by calling `RcBiMutatingFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalBiMutatingFunction`
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalBiMutatingFunction<T, U, R> {
    function: RcBiMutatingFunction<T, U, R>,
    predicate: RcBiPredicate<T, U>,
}

// Implement RcConditionalBiMutatingFunction
impl_shared_conditional_function!(
    RcConditionalBiMutatingFunction<T, U, R>,
    RcBiMutatingFunction,
    BiMutatingFunction,
    into_rc,
    'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(RcConditionalBiMutatingFunction<T, U, R>);

// Generate Clone implementation
impl_conditional_function_clone!(RcConditionalBiMutatingFunction<T, U, R>);

// ============================================================================
// ArcConditionalBiMutatingFunction - Arc-based Conditional BiMutatingFunction
// ============================================================================

/// ArcConditionalBiMutatingFunction struct
///
/// A thread-safe conditional bi-mutating-function that only executes when a
/// bi-predicate is satisfied. Uses `ArcBiMutatingFunction` and `ArcBiPredicate` for
/// shared ownership across threads.
///
/// This type is typically created by calling `ArcBiMutatingFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalBiMutatingFunction<T, U, R> {
    function: ArcBiMutatingFunction<T, U, R>,
    predicate: ArcBiPredicate<T, U>,
}

// Implement ArcConditionalBiMutatingFunction
impl_shared_conditional_function!(
    ArcConditionalBiMutatingFunction<T, U, R>,
    ArcBiMutatingFunction,
    BiMutatingFunction,
    into_arc,
    Send + Sync + 'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(ArcConditionalBiMutatingFunction<T, U, R>);

// Generate Clone implementation
impl_conditional_function_clone!(ArcConditionalBiMutatingFunction<T, U, R>);
