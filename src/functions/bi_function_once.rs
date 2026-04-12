/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # BiFunctionOnce Types
//!
//! Provides Rust implementations of consuming bi-function traits similar to
//! Rust's `FnOnce(&T, &U) -> R` trait, but with value-oriented semantics for functional
//! programming patterns with two input references.
//!
//! This module provides the `BiFunctionOnce<T, U, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxBiFunctionOnce`]: Single ownership, one-time use
//!
//! # Author
//!
//! Haixing Hu
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::bi_predicate::{
    BiPredicate,
    BoxBiPredicate,
};
use crate::{
    functions::function_once::FunctionOnce,
    functions::macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_debug_display,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
    },
};

// ============================================================================
// Core Trait
// ============================================================================

/// BiFunctionOnce trait - consuming bi-function that takes references
///
/// Defines the behavior of a consuming bi-function: computing a value of
/// type `R` from references to types `T` and `U` by taking ownership of self.
/// This trait is analogous to `FnOnce(&T, &U) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the first input value (borrowed)
/// * `U` - The type of the second input value (borrowed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait BiFunctionOnce<T, U, R> {
    /// Computes output from two input references, consuming self
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first input value
    /// * `second` - Reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(self, first: &T, second: &U) -> R;

    /// Converts to BoxBiFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiFunctionOnce<T, U, R>`
    fn into_box(self) -> BoxBiFunctionOnce<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiFunctionOnce::new(move |t: &T, u: &U| self.apply(t, u))
    }

    /// Converts bi-function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&T, &U) -> R`
    fn into_fn(self) -> impl FnOnce(&T, &U) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        move |t: &T, u: &U| self.apply(t, u)
    }

    /// Converts bi-function to a boxed function pointer
    ///
    /// **📌 Borrows `&self`**: The original bi-function remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a boxed function pointer that implements `FnOnce(&T, &U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiFunctionOnce;
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let func = add.to_box();
    /// assert_eq!(func.apply(&20, &22), 42);
    /// ```
    fn to_box(&self) -> BoxBiFunctionOnce<T, U, R>
    where
        Self: Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Converts bi-function to a closure
    ///
    /// **📌 Borrows `&self`**: The original bi-function remains usable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&T, &U) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::BiFunctionOnce;
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let func = add.to_fn();
    /// assert_eq!(func(&20, &22), 42);
    /// ```
    fn to_fn(&self) -> impl FnOnce(&T, &U) -> R
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
// BoxBiFunctionOnce - Box<dyn FnOnce(&T, &U) -> R>
// ============================================================================

/// BoxBiFunctionOnce - consuming bi-function wrapper based on
/// `Box<dyn FnOnce>`
///
/// A bi-function wrapper that provides single ownership with one-time use
/// semantics. Consumes self and borrows both input values.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(&T, &U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxBiFunctionOnce<T, U, R> {
    function: Box<dyn FnOnce(&T, &U) -> R>,
    name: Option<String>,
}

// Implement BoxBiFunctionOnce
impl<T, U, R> BoxBiFunctionOnce<T, U, R> {
    // Generate new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxBiFunctionOnce<T, U, R>,
        (FnOnce(&T, &U) -> R + 'static),
        |f| Box::new(f)
    );

    // Generate when(), and_then()
    impl_box_function_methods!(
        BoxBiFunctionOnce<T, U, R>,
        BoxConditionalBiFunctionOnce,
        FunctionOnce
    );
}

// Implement BiFunctionOnce trait for BoxBiFunctionOnce
impl<T, U, R> BiFunctionOnce<T, U, R> for BoxBiFunctionOnce<T, U, R> {
    fn apply(self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_fn(), to_box()
    impl_box_once_conversions!(
        BoxBiFunctionOnce<T, U, R>,
        BiFunctionOnce,
        FnOnce(&T, &U) -> R
    );
}

// Implement constant method for BoxBiFunctionOnce
impl_function_constant_method!(BoxBiFunctionOnce<T, U, R>);

// Use macro to generate Debug and Display implementations
impl_function_debug_display!(BoxBiFunctionOnce<T, U, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

// Implement BiFunctionOnce for all FnOnce(&T, &U) -> R using macro
impl_closure_once_trait!(
    BiFunctionOnce<T, U, R>,
    apply,
    BoxBiFunctionOnce,
    FnOnce(first: &T, second: &U) -> R
);

// ============================================================================
// FnBiFunctionOnceOps - Extension trait for FnOnce(&T, &U) -> R bi-functions
// ============================================================================

/// Extension trait for closures implementing `FnOnce(&T, &U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for one-time use
/// bi-function closures and function pointers without requiring explicit
/// wrapping in `BoxBiFunctionOnce`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `FnOnce(&T, &U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `BiFunctionOnce<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxBiFunctionOnce` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps};
///
/// let add = |x: &i32, y: &i32| *x + *y;
/// let double = |x: i32| x * 2;
///
/// let composed = add.and_then(double);
/// assert_eq!(composed.apply(&3, &5), 16); // (3 + 5) * 2
/// ```
///
/// ## Conditional execution with when
///
/// ```rust
/// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps};
///
/// let add = |x: &i32, y: &i32| *x + *y;
/// let multiply = |x: &i32, y: &i32| *x * *y;
///
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
/// assert_eq!(conditional.apply(&5, &3), 8); // when branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnBiFunctionOnceOps<T, U, R>: FnOnce(&T, &U) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-function that applies this bi-function first,
    /// then applies the after function to the result. Consumes self and
    /// returns a `BoxBiFunctionOnce`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after function
    /// * `F` - The type of the after function (must implement FunctionOnce<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The function to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** Since this is a
    ///   `FnOnce` bi-function, the parameter will be consumed. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxFunctionOnce<R, S>`
    ///   - Any type implementing `FunctionOnce<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxBiFunctionOnce<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps,
    ///     BoxFunctionOnce};
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let to_string = BoxFunctionOnce::new(|x: i32| x.to_string());
    ///
    /// // to_string is moved and consumed
    /// let composed = add.and_then(to_string);
    /// assert_eq!(composed.apply(&20, &22), "42");
    /// // to_string.apply(10); // Would not compile - moved
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxBiFunctionOnce<T, U, S>
    where
        Self: 'static,
        S: 'static,
        F: crate::functions::function_once::FunctionOnce<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiFunctionOnce::new(move |t: &T, u: &U| after.apply(&self(t, u)))
    }

    /// Creates a conditional bi-function
    ///
    /// Returns a bi-function that only executes when a bi-predicate is
    /// satisfied. You must call `or_else()` to provide an alternative
    /// bi-function for when the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. **Note: This parameter is passed
    ///   by value and will transfer ownership.** If you need to preserve the
    ///   original bi-predicate, clone it first (if it implements `Clone`).
    ///   Can be:
    ///   - A closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalBiFunctionOnce<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps};
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let multiply = |x: &i32, y: &i32| *x * *y;
    /// let conditional = add.when(|x: &i32, y: &i32| *x > 0)
    ///     .or_else(multiply);
    ///
    /// assert_eq!(conditional.apply(&5, &3), 8);
    /// ```
    ///
    /// ## Preserving bi-predicate with clone
    ///
    /// ```rust
    /// use qubit_function::{BiFunctionOnce, FnBiFunctionOnceOps,
    ///     RcBiPredicate};
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let both_positive = RcBiPredicate::new(|x: &i32, y: &i32|
    ///     *x > 0 && *y > 0);
    ///
    /// // Clone to preserve original bi-predicate
    /// let conditional = add.when(both_positive.clone())
    ///     .or_else(|x: &i32, y: &i32| *x * *y);
    ///
    /// assert_eq!(conditional.apply(&5, &3), 8);
    ///
    /// // Original bi-predicate still usable
    /// assert!(both_positive.test(&5, &3));
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalBiFunctionOnce<T, U, R>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiFunctionOnce::new(self).when(predicate)
    }
}

/// Blanket implementation of FnBiFunctionOnceOps for all closures
///
/// Automatically implements `FnBiFunctionOnceOps<T, U, R>` for any type that
/// implements `FnOnce(&T, &U) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, U, R, F> FnBiFunctionOnceOps<T, U, R> for F
where
    F: FnOnce(&T, &U) -> R,
{
    // empty
}

// ============================================================================
// BoxConditionalBiFunctionOnce - Box-based Conditional BiFunction
// ============================================================================

/// BoxConditionalBiFunctionOnce struct
///
/// A conditional consuming bi-function that only executes when a bi-predicate
/// is satisfied. Uses `BoxBiFunctionOnce` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiFunctionOnce::when()` and
/// is designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiFunctionOnce, BoxBiFunctionOnce};
///
/// let add = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
/// let multiply = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x * *y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
/// assert_eq!(conditional.apply(&5, &3), 8); // when branch executed
///
/// let add2 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
/// let multiply2 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x * *y);
/// let conditional2 = add2.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply2);
/// assert_eq!(conditional2.apply(&-5, &3), -15); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalBiFunctionOnce<T, U, R> {
    function: BoxBiFunctionOnce<T, U, R>,
    predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalBiFunctionOnce
impl_box_conditional_function!(
    BoxConditionalBiFunctionOnce<T, U, R>,
    BoxBiFunctionOnce,
    BiFunctionOnce
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(BoxConditionalBiFunctionOnce<T, U, R>);
