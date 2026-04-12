/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # FunctionOnce Types
//!
//! Provides Rust implementations of consuming function traits similar to
//! Rust's `FnOnce(&T) -> R` trait, for computing output from input references.
//!
//! This module provides the `FunctionOnce<T, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxFunctionOnce`]: Single ownership, one-time use
//!
//! # Author
//!
//! Haixing Hu
use crate::functions::macros::{
    impl_box_conditional_function,
    impl_box_function_methods,
    impl_conditional_function_debug_display,
    impl_fn_ops_trait,
    impl_function_common_methods,
    impl_function_constant_method,
    impl_function_debug_display,
    impl_function_identity_method,
};
use crate::macros::{
    impl_box_once_conversions,
    impl_closure_once_trait,
};
use crate::predicates::predicate::{
    BoxPredicate,
    Predicate,
};

// ============================================================================
// Core Trait
// ============================================================================

/// FunctionOnce trait - consuming function that takes ownership
///
/// Defines the behavior of a consuming function: computing a value of
/// type `R` from a reference to type `T` by taking ownership of self.
/// This trait is analogous to `FnOnce(&T) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (borrowed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait FunctionOnce<T, R> {
    /// Applies the function to the input reference, consuming self
    ///
    /// # Parameters
    ///
    /// * `t` - Reference to the input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(self, t: &T) -> R;

    /// Converts to BoxFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::FunctionOnce;
    ///
    /// let double = |x: &i32| x * 2;
    /// let boxed = double.into_box();
    /// assert_eq!(boxed.apply(&21), 42);
    /// ```
    fn into_box(self) -> BoxFunctionOnce<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxFunctionOnce::new(move |input: &T| self.apply(input))
    }

    /// Converts function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::FunctionOnce;
    ///
    /// let double = |x: &i32| x * 2;
    /// let func = double.into_fn();
    /// assert_eq!(func(&21), 42);
    /// ```
    fn into_fn(self) -> impl FnOnce(&T) -> R
    where
        Self: Sized + 'static,
    {
        move |input: &T| self.apply(input)
    }

    /// Converts to BoxFunctionOnce without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `BoxFunctionOnce` that
    /// captures a clone. Types implementing `Clone` can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::FunctionOnce;
    ///
    /// let double = |x: &i32| x * 2;
    /// let boxed = double.to_box();
    /// assert_eq!(boxed.apply(&21), 42);
    /// ```
    fn to_box(&self) -> BoxFunctionOnce<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Converts function to a closure without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures a
    /// clone of `self` and calls its `apply` method. Types can
    /// override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(&T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::FunctionOnce;
    ///
    /// let double = |x: &i32| x * 2;
    /// let func = double.to_fn();
    /// assert_eq!(func(&21), 42);
    /// ```
    fn to_fn(&self) -> impl FnOnce(&T) -> R
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }
}

// ============================================================================
// BoxFunctionOnce - Box<dyn FnOnce(&T) -> R>
// ============================================================================

/// BoxFunctionOnce - consuming transformer wrapper based on
/// `Box<dyn FnOnce>`
///
/// A transformer wrapper that provides single ownership with one-time use
/// semantics. Consumes both self and the input value.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(&T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self and input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxFunctionOnce<T, R> {
    function: Box<dyn FnOnce(&T) -> R>,
    name: Option<String>,
}

impl<T, R> BoxFunctionOnce<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxFunctionOnce<T, R>,
        (FnOnce(&T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxFunctionOnce<T, R>,
        BoxConditionalFunctionOnce,
        FunctionOnce
    );
}

impl<T, R> FunctionOnce<T, R> for BoxFunctionOnce<T, R> {
    fn apply(self, input: &T) -> R {
        (self.function)(input)
    }

    impl_box_once_conversions!(
        BoxFunctionOnce<T, R>,
        FunctionOnce,
        FnOnce(&T) -> R
    );
}

// Generates: constant() method for BoxFunctionOnce<T, R>
impl_function_constant_method!(BoxFunctionOnce<T, R>, 'static);

// Generates: identity() method for BoxFunctionOnce<T, T>
impl_function_identity_method!(BoxFunctionOnce<T, T>);

// Generates: Debug and Display implementations for BoxFunctionOnce<T, R>
impl_function_debug_display!(BoxFunctionOnce<T, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

// Implement FunctionOnce for all FnOnce(&T) -> R using macro
impl_closure_once_trait!(
    FunctionOnce<T, R>,
    apply,
    BoxFunctionOnce,
    FnOnce(input: &T) -> R
);

// ============================================================================
// FnFunctionOnceOps - Extension trait for FnOnce transformers
// ============================================================================

// Generates: FnFunctionOnceOps trait and blanket implementation
impl_fn_ops_trait!(
    (FnOnce(&T) -> R),
    FnFunctionOnceOps,
    BoxFunctionOnce,
    FunctionOnce,
    BoxConditionalFunctionOnce
);

// ============================================================================
// BoxConditionalFunctionOnce - Box-based Conditional Function
// ============================================================================

/// BoxConditionalFunctionOnce struct
///
/// A conditional consuming transformer that only executes when a predicate is
/// satisfied. Uses `BoxFunctionOnce` and `BoxPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxFunctionOnce::when()` and
/// is designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **One-time Use**: Can only be called once
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{FunctionOnce, BoxFunctionOnce};
///
/// let double = BoxFunctionOnce::new(|x: i32| x * 2);
/// let negate = BoxFunctionOnce::new(|x: i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
/// assert_eq!(conditional.apply(5), 10); // when branch executed
///
/// let double2 = BoxFunctionOnce::new(|x: i32| x * 2);
/// let negate2 = BoxFunctionOnce::new(|x: i32| -x);
/// let conditional2 = double2.when(|x: &i32| *x > 0).or_else(negate2);
/// assert_eq!(conditional2.apply(-5), 5); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalFunctionOnce<T, R> {
    function: BoxFunctionOnce<T, R>,
    predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalFunctionOnce<T, R>,
    BoxFunctionOnce,
    FunctionOnce
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalFunctionOnce<T, R>);
