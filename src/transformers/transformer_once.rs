/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # TransformerOnce Types
//!
//! Provides Rust implementations of consuming transformer traits similar to
//! Rust's `FnOnce` trait, but with value-oriented semantics for functional
//! programming patterns.
//!
//! This module provides the `TransformerOnce<T, R>` trait and one-time use
//! implementations:
//!
//! - [`BoxTransformerOnce`]: Single ownership, one-time use
//!
//! # Author
//!
//! Haixing Hu

use crate::{
    macros::box_conversions::impl_box_once_conversions,
    predicates::predicate::{
        BoxPredicate,
        Predicate,
    },
    transformers::macros::{
        impl_box_conditional_transformer,
        impl_box_transformer_methods,
        impl_conditional_transformer_debug_display,
        impl_transformer_common_methods,
        impl_transformer_constant_method,
        impl_transformer_debug_display,
    },
};

// ============================================================================
// Core Trait
// ============================================================================

/// TransformerOnce trait - consuming transformation that takes ownership
///
/// Defines the behavior of a consuming transformer: converting a value of
/// type `T` to a value of type `R` by taking ownership of both self and the
/// input. This trait is analogous to `FnOnce(T) -> R`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait TransformerOnce<T, R> {
    /// Transforms the input value, consuming both self and input
    ///
    /// # Parameters
    ///
    /// * `input` - The input value (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(self, input: T) -> R;

    /// Converts to BoxTransformerOnce
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxTransformerOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::TransformerOnce;
    ///
    /// let double = |x: i32| x * 2;
    /// let boxed = double.into_box();
    /// assert_eq!(boxed.apply(21), 42);
    /// ```
    fn into_box(self) -> BoxTransformerOnce<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformerOnce::new(move |input: T| self.apply(input))
    }

    /// Converts transformer to a closure
    ///
    /// **⚠️ Consumes `self`**: The original transformer becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::TransformerOnce;
    ///
    /// let double = |x: i32| x * 2;
    /// let func = double.into_fn();
    /// assert_eq!(func(21), 42);
    /// ```
    fn into_fn(self) -> impl FnOnce(T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        move |input: T| self.apply(input)
    }

    /// Converts to BoxTransformerOnce without consuming self
    ///
    /// **📌 Borrows `&self`**: The original transformer remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `BoxTransformerOnce` that
    /// captures a clone. Types implementing `Clone` can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxTransformerOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::TransformerOnce;
    ///
    /// let double = |x: i32| x * 2;
    /// let boxed = double.to_box();
    /// assert_eq!(boxed.apply(21), 42);
    /// ```
    fn to_box(&self) -> BoxTransformerOnce<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Converts transformer to a closure without consuming self
    ///
    /// **📌 Borrows `&self`**: The original transformer remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures a
    /// clone of `self` and calls its `transform` method. Types can
    /// override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `FnOnce(T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::TransformerOnce;
    ///
    /// let double = |x: i32| x * 2;
    /// let func = double.to_fn();
    /// assert_eq!(func(21), 42);
    /// ```
    fn to_fn(&self) -> impl FnOnce(T) -> R
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }
}

// ============================================================================
// BoxTransformerOnce - Box<dyn FnOnce(T) -> R>
// ============================================================================

/// BoxTransformerOnce - consuming transformer wrapper based on
/// `Box<dyn FnOnce>`
///
/// A transformer wrapper that provides single ownership with one-time use
/// semantics. Consumes both self and the input value.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnOnce(T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can only be called once (consumes self and input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxTransformerOnce<T, R> {
    function: Box<dyn FnOnce(T) -> R>,
    name: Option<String>,
}

// Implement BoxTransformerOnce
impl<T, R> BoxTransformerOnce<T, R>
where
    T: 'static,
    R: 'static,
{
    impl_transformer_common_methods!(
        BoxTransformerOnce<T, R>,
        (FnOnce(T) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_transformer_methods!(
        BoxTransformerOnce<T, R>,
        BoxConditionalTransformerOnce,
        TransformerOnce
    );
}

// Implement TransformerOnce trait for BoxTransformerOnce
impl<T, R> TransformerOnce<T, R> for BoxTransformerOnce<T, R> {
    fn apply(self, input: T) -> R {
        (self.function)(input)
    }

    impl_box_once_conversions!(
        BoxTransformerOnce<T, R>,
        TransformerOnce,
        FnOnce(T) -> R
    );
}

// Implement constant method for BoxTransformerOnce
impl_transformer_constant_method!(BoxTransformerOnce<T, R>);

// Use macro to generate Debug and Display implementations
impl_transformer_debug_display!(BoxTransformerOnce<T, R>);

// ============================================================================
// Blanket implementation for standard FnOnce trait
// ============================================================================

/// Implement TransformerOnce<T, R> for any type that implements
/// FnOnce(T) -> R
///
/// This allows once-callable closures and function pointers to be used
/// directly with our TransformerOnce trait without wrapping.
///
/// # Examples
///
/// ```rust
/// use prism3_function::TransformerOnce;
///
/// fn parse(s: String) -> i32 {
///     s.parse().unwrap_or(0)
/// }
///
/// assert_eq!(parse.apply("42".to_string()), 42);
///
/// let owned_value = String::from("hello");
/// let consume = |s: String| {
///     format!("{} world", s)
/// };
/// assert_eq!(consume.apply(owned_value), "hello world");
/// ```
///
/// # Author
///
/// Haixing Hu
impl<F, T, R> TransformerOnce<T, R> for F
where
    F: FnOnce(T) -> R,
    T: 'static,
    R: 'static,
{
    fn apply(self, input: T) -> R {
        self(input)
    }

    fn into_box(self) -> BoxTransformerOnce<T, R>
    where
        Self: Sized + 'static,
    {
        BoxTransformerOnce::new(self)
    }

    fn into_fn(self) -> impl FnOnce(T) -> R
    where
        Self: Sized + 'static,
    {
        self
    }

    // use the default implementation of to_box() from TransformerOnce trait

    fn to_fn(&self) -> impl FnOnce(T) -> R
    where
        Self: Clone + Sized + 'static,
    {
        self.clone()
    }
}

// ============================================================================
// FnTransformerOnceOps - Extension trait for FnOnce transformers
// ============================================================================

/// Extension trait for closures implementing `FnOnce(T) -> R`
///
/// Provides composition methods (`and_then`, `compose`, `when`) for one-time
/// use closures and function pointers without requiring explicit wrapping in
/// `BoxTransformerOnce`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `FnOnce(T) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `TransformerOnce<T, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then`, `compose`, and `when`. This extension trait provides those
/// methods, returning `BoxTransformerOnce` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use prism3_function::{TransformerOnce, FnTransformerOnceOps};
///
/// let parse = |s: String| s.parse::<i32>().unwrap_or(0);
/// let double = |x: i32| x * 2;
///
/// let composed = parse.and_then(double);
/// assert_eq!(composed.apply("21".to_string()), 42);
/// ```
///
/// ## Reverse composition with compose
///
/// ```rust
/// use prism3_function::{TransformerOnce, FnTransformerOnceOps};
///
/// let double = |x: i32| x * 2;
/// let to_string = |x: i32| x.to_string();
///
/// let composed = to_string.compose(double);
/// assert_eq!(composed.apply(21), "42");
/// ```
///
/// ## Conditional transformation with when
///
/// ```rust
/// use prism3_function::{TransformerOnce, FnTransformerOnceOps};
///
/// let double = |x: i32| x * 2;
/// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
///
/// assert_eq!(conditional.apply(5), 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnTransformerOnceOps<T, R>: FnOnce(T) -> R + Sized + 'static {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new transformer that applies this transformer first, then
    /// applies the after transformer to the result. Consumes self and returns
    /// a `BoxTransformerOnce`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after transformer
    /// * `G` - The type of the after transformer (must implement
    ///   TransformerOnce<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The transformer to apply after self. **Note: This parameter
    ///   is passed by value and will transfer ownership.** Since this is a
    ///   `FnOnce` transformer, the parameter will be consumed. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A function pointer: `fn(R) -> S`
    ///   - A `BoxTransformerOnce<R, S>`
    ///   - Any type implementing `TransformerOnce<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxTransformerOnce<T, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{TransformerOnce, FnTransformerOnceOps,
    ///     BoxTransformerOnce};
    ///
    /// let parse = |s: String| s.parse::<i32>().unwrap_or(0);
    /// let double = BoxTransformerOnce::new(|x: i32| x * 2);
    ///
    /// // double is moved and consumed
    /// let composed = parse.and_then(double);
    /// assert_eq!(composed.apply("21".to_string()), 42);
    /// // double.apply(5); // Would not compile - moved
    /// ```
    fn and_then<S, G>(self, after: G) -> BoxTransformerOnce<T, S>
    where
        S: 'static,
        G: TransformerOnce<R, S> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformerOnce::new(move |x: T| {
            let intermediate = self(x);
            after.apply(intermediate)
        })
    }

    /// Creates a conditional transformer
    ///
    /// Returns a transformer that only executes when a predicate is satisfied.
    /// You must call `or_else()` to provide an alternative transformer for when
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
    /// Returns `BoxConditionalTransformerOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use prism3_function::{TransformerOnce, FnTransformerOnceOps};
    ///
    /// let double = |x: i32| x * 2;
    /// let conditional = double.when(|x: &i32| *x > 0).or_else(|x: i32| -x);
    ///
    /// assert_eq!(conditional.apply(5), 10);
    /// ```
    ///
    /// ## Preserving predicate with clone
    ///
    /// ```rust
    /// use prism3_function::{TransformerOnce, FnTransformerOnceOps,
    ///     RcPredicate};
    ///
    /// let double = |x: i32| x * 2;
    /// let is_positive = RcPredicate::new(|x: &i32| *x > 0);
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
    fn when<P>(self, predicate: P) -> BoxConditionalTransformerOnce<T, R>
    where
        P: Predicate<T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxTransformerOnce::new(self).when(predicate)
    }
}

/// Blanket implementation of FnTransformerOnceOps for all FnOnce closures
///
/// Automatically implements `FnTransformerOnceOps<T, R>` for any type that
/// implements `FnOnce(T) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, R, F> FnTransformerOnceOps<T, R> for F where F: FnOnce(T) -> R + 'static {}

// ============================================================================
// UnaryOperatorOnce Trait - Marker trait for TransformerOnce<T, T>
// ============================================================================

/// UnaryOperatorOnce trait - marker trait for one-time use unary operators
///
/// A one-time use unary operator transforms a value of type `T` to another
/// value of the same type `T`, consuming self in the process. This trait
/// extends `TransformerOnce<T, T>` to provide semantic clarity for same-type
/// transformations with consuming semantics. Equivalent to Java's
/// `UnaryOperator<T>` but with FnOnce semantics.
///
/// # Automatic Implementation
///
/// This trait is automatically implemented for all types that implement
/// `TransformerOnce<T, T>`, so you don't need to implement it manually.
///
/// # Type Parameters
///
/// * `T` - The type of both input and output values
///
/// # Examples
///
/// ## Using in generic constraints
///
/// ```rust
/// use prism3_function::{UnaryOperatorOnce, TransformerOnce};
///
/// fn apply<T, O>(value: T, op: O) -> T
/// where
///     O: UnaryOperatorOnce<T>,
/// {
///     op.apply(value)
/// }
///
/// let double = |x: i32| x * 2;
/// assert_eq!(apply(21, double), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait UnaryOperatorOnce<T>: TransformerOnce<T, T> {}

/// Blanket implementation of UnaryOperatorOnce for all TransformerOnce<T, T>
///
/// This automatically implements `UnaryOperatorOnce<T>` for any type that
/// implements `TransformerOnce<T, T>`.
///
/// # Author
///
/// Haixing Hu
impl<F, T> UnaryOperatorOnce<T> for F
where
    F: TransformerOnce<T, T>,
    T: 'static,
{
    // empty
}

// ============================================================================
// Type Aliases for UnaryOperatorOnce (TransformerOnce<T, T>)
// ============================================================================

/// Type alias for `BoxTransformerOnce<T, T>`
///
/// Represents a one-time use unary operator that transforms a value of type `T`
/// to another value of the same type `T`. Equivalent to Java's `UnaryOperator<T>`
/// with consuming semantics (FnOnce).
///
/// # Examples
///
/// ```rust
/// use prism3_function::{BoxUnaryOperatorOnce, TransformerOnce};
///
/// let increment: BoxUnaryOperatorOnce<i32> = BoxUnaryOperatorOnce::new(|x| x + 1);
/// assert_eq!(increment.apply(41), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type BoxUnaryOperatorOnce<T> = BoxTransformerOnce<T, T>;

// ============================================================================
// BoxConditionalTransformerOnce - Box-based Conditional Transformer
// ============================================================================

/// BoxConditionalTransformerOnce struct
///
/// A conditional consuming transformer that only executes when a predicate is
/// satisfied. Uses `BoxTransformerOnce` and `BoxPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxTransformerOnce::when()` and
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
/// use prism3_function::{TransformerOnce, BoxTransformerOnce};
///
/// let double = BoxTransformerOnce::new(|x: i32| x * 2);
/// let negate = BoxTransformerOnce::new(|x: i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
/// assert_eq!(conditional.apply(5), 10); // when branch executed
///
/// let double2 = BoxTransformerOnce::new(|x: i32| x * 2);
/// let negate2 = BoxTransformerOnce::new(|x: i32| -x);
/// let conditional2 = double2.when(|x: &i32| *x > 0).or_else(negate2);
/// assert_eq!(conditional2.apply(-5), 5); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalTransformerOnce<T, R> {
    transformer: BoxTransformerOnce<T, R>,
    predicate: BoxPredicate<T>,
}

// Implement BoxConditionalTransformerOnce
impl_box_conditional_transformer!(
    BoxConditionalTransformerOnce<T, R>,
    BoxTransformerOnce,
    TransformerOnce
);

// Use macro to generate Debug and Display implementations
impl_conditional_transformer_debug_display!(BoxConditionalTransformerOnce<T, R>);
