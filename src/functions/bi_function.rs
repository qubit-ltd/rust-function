/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! # BiFunction Types
//!
//! Provides Rust implementations of bi-function traits for computing output values
//! from two input references. BiFunctions borrow input values (not consuming them)
//! and produce output values.
//!
//! It is similar to the `Fn(&T, &U) -> R` trait in the standard library.
//!
//! This module provides the `BiFunction<T, U, R>` trait and three
//! implementations:
//!
//! - [`BoxBiFunction`]: Single ownership, not cloneable
//! - [`ArcBiFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcBiFunction`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu
use std::rc::Rc;
use std::sync::Arc;

use crate::functions::{
    bi_function_once::BoxBiFunctionOnce,
    function::Function,
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

/// BiFunction trait - computes output from two input references
///
/// Defines the behavior of a bi-function: computing a value of type `R`
/// from references to types `T` and `U` without consuming the inputs. This is analogous to
/// `Fn(&T, &U) -> R` in Rust's standard library, similar to Java's `BiFunction<T, U, R>`.
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
pub trait BiFunction<T, U, R> {
    /// Applies the bi-function to two input references to produce an output value
    ///
    /// # Parameters
    ///
    /// * `first` - Reference to the first input value
    /// * `second` - Reference to the second input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, first: &T, second: &U) -> R;

    /// Converts to BoxBiFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxBiFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxBiFunction<T, U, R>`
    fn into_box(self) -> BoxBiFunction<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts to RcBiFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcBiFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcBiFunction<T, U, R>`
    fn into_rc(self) -> RcBiFunction<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        RcBiFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts to ArcBiFunction
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates
    /// an `ArcBiFunction`. Types can override this method to provide
    /// more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcBiFunction<T, U, R>`
    fn into_arc(self) -> ArcBiFunction<T, U, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        U: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        ArcBiFunction::new(move |t, u| self.apply(t, u))
    }

    /// Converts bi-function to a closure
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
    /// Returns a closure that implements `Fn(&T, &U) -> R`
    fn into_fn(self) -> impl Fn(&T, &U) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        move |t, u| self.apply(t, u)
    }

    /// Converts to BiFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original bi-function becomes unavailable after calling this method.
    ///
    /// Converts a reusable bi-function to a one-time bi-function that consumes itself on use.
    /// This enables passing `BiFunction` to functions that require `BiFunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiFunctionOnce<T, U, R>`
    fn into_once(self) -> BoxBiFunctionOnce<T, U, R>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiFunctionOnce::new(move |t, u| self.apply(t, u))
    }

    /// Non-consuming conversion to `BoxBiFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_box`.
    fn to_box(&self) -> BoxBiFunction<T, U, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcBiFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_rc`.
    fn to_rc(&self) -> RcBiFunction<T, U, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcBiFunction` using `&self`.
    ///
    /// Default implementation clones `self` and delegates to `into_arc`.
    fn to_arc(&self) -> ArcBiFunction<T, U, R>
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
    /// Returns a `Box<dyn Fn(&T, &U) -> R>` that clones `self` and calls
    /// `apply` inside the boxed closure.
    fn to_fn(&self) -> impl Fn(&T, &U) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to BiFunctionOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current bi-function and converts the clone to a one-time bi-function.
    ///
    /// # Returns
    ///
    /// Returns a `BoxBiFunctionOnce<T, U, R>`
    fn to_once(&self) -> BoxBiFunctionOnce<T, U, R>
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
// BoxBiFunction - Box<dyn Fn(&T, &U) -> R>
// ============================================================================

/// BoxBiFunction - bi-function wrapper based on `Box<dyn Fn>`
///
/// A bi-function wrapper that provides single ownership with reusable
/// computation. Borrows both inputs and can be called multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(&T, &U) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (borrows inputs each time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxBiFunction<T, U, R> {
    function: Box<dyn Fn(&T, &U) -> R>,
    name: Option<String>,
}

// Implement BoxBiFunction
impl<T, U, R> BoxBiFunction<T, U, R> {
    impl_function_common_methods!(
        BoxBiFunction<T, U, R>,
        (Fn(&T, &U) -> R + 'static),
        |f| Box::new(f)
    );

    impl_box_function_methods!(
        BoxBiFunction<T, U, R>,
        BoxConditionalBiFunction,
        Function
    );
}

// Implement BiFunction trait for BoxBiFunction
impl<T, U, R> BiFunction<T, U, R> for BoxBiFunction<T, U, R> {
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxBiFunction<T, U, R>,
        RcBiFunction,
        Fn(&T, &U) -> R,
        BoxBiFunctionOnce
    );
}

// Implement constant method for BoxBiFunction
impl_function_constant_method!(BoxBiFunction<T, U, R>);

// Implement Debug and Display for BoxBiFunction
impl_function_debug_display!(BoxBiFunction<T, U, R>);

// ============================================================================
// RcBiFunction - Rc<dyn Fn(&T, &U) -> R>
// ============================================================================

/// RcBiFunction - single-threaded bi-function wrapper
///
/// A single-threaded, clonable bi-function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(&T, &U) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (borrows inputs each time)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct RcBiFunction<T, U, R> {
    function: Rc<dyn Fn(&T, &U) -> R>,
    name: Option<String>,
}

impl<T, U, R> RcBiFunction<T, U, R> {
    impl_function_common_methods!(
        RcBiFunction<T, U, R>,
        (Fn(&T, &U) -> R + 'static),
        |f| Rc::new(f)
    );
    impl_shared_function_methods!(
        RcBiFunction<T, U, R>,
        RcConditionalBiFunction,
        into_rc,
        Function,
        'static
    );
}

// Implement BiFunction trait for RcBiFunction
impl<T, U, R> BiFunction<T, U, R> for RcBiFunction<T, U, R> {
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }

    // Generate into_box(), into_rc(), into_fn(), into_once(), to_box(), to_rc(), to_fn(), to_once()
    impl_rc_conversions!(
        RcBiFunction<T, U, R>,
        BoxBiFunction,
        BoxBiFunctionOnce,
        Fn(first: &T, second: &U) -> R
    );
}

// Implement constant method for RcBiFunction
impl_function_constant_method!(RcBiFunction<T, U, R>);

// Implement Debug and Display for RcBiFunction
impl_function_debug_display!(RcBiFunction<T, U, R>);

// Implement Clone for RcBiFunction
impl_function_clone!(RcBiFunction<T, U, R>);

// ============================================================================
// ArcBiFunction - Arc<dyn Fn(&T, &U) -> R + Send + Sync>
// ============================================================================

/// ArcBiFunction - thread-safe bi-function wrapper
///
/// A thread-safe, clonable bi-function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(&T, &U) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (borrows inputs each time)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct ArcBiFunction<T, U, R> {
    function: Arc<dyn Fn(&T, &U) -> R + Send + Sync>,
    name: Option<String>,
}

impl<T, U, R> ArcBiFunction<T, U, R> {
    impl_function_common_methods!(
        ArcBiFunction<T, U, R>,
        (Fn(&T, &U) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );
    impl_shared_function_methods!(
        ArcBiFunction<T, U, R>,
        ArcConditionalBiFunction,
        into_arc,
        Function,
        Send + Sync + 'static
    );
}

// Implement constant method for ArcBiFunction
impl_function_constant_method!(ArcBiFunction<T, U, R>, Send + Sync + 'static);

// Implement Debug and Display for ArcBiFunction
impl_function_debug_display!(ArcBiFunction<T, U, R>);

// Implement Clone for ArcBiFunction
impl_function_clone!(ArcBiFunction<T, U, R>);

// Implement BiFunction trait for ArcBiFunction
impl<T, U, R> BiFunction<T, U, R> for ArcBiFunction<T, U, R> {
    fn apply(&self, first: &T, second: &U) -> R {
        (self.function)(first, second)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcBiFunction<T, U, R>,
        BoxBiFunction,
        RcBiFunction,
        BoxBiFunctionOnce,
        Fn(t: &T, u: &U) -> R
    );
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement BiFunction<T, U, R> for any type that implements Fn(&T, &U) -> R
impl_closure_trait!(
    BiFunction<T, U, R>,
    apply,
    BoxBiFunctionOnce,
    Fn(first: &T, second: &U) -> R
);

// ============================================================================
// FnBiFunctionOps - Extension trait for Fn(&T, &U) -> R bi-functions
// ============================================================================

/// Extension trait for closures implementing `Fn(&T, &U) -> R`
///
/// Provides composition methods (`and_then`, `when`) for bi-function
/// closures and function pointers without requiring explicit wrapping in
/// `BoxBiFunction`.
///
/// This trait is automatically implemented for all closures and function
/// pointers that implement `Fn(&T, &U) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `BiFunction<T, U, R>` through
/// blanket implementation, they don't have access to instance methods like
/// `and_then` and `when`. This extension trait provides those methods,
/// returning `BoxBiFunction` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use qubit_function::{BiFunction, FnBiFunctionOps};
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
/// use qubit_function::{BiFunction, FnBiFunctionOps};
///
/// let add = |x: &i32, y: &i32| *x + *y;
/// let multiply = |x: &i32, y: &i32| *x * *y;
///
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
///
/// assert_eq!(conditional.apply(&5, &3), 8);   // add
/// assert_eq!(conditional.apply(&-5, &3), -15); // multiply
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnBiFunctionOps<T, U, R>: Fn(&T, &U) -> R + Sized {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new bi-function that applies this bi-function first,
    /// then applies the after function to the result. Consumes self and
    /// returns a `BoxBiFunction`.
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
    /// A new `BoxBiFunction<T, U, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ## Direct value passing (ownership transfer)
    ///
    /// ```rust
    /// use qubit_function::{BiFunction, FnBiFunctionOps,
    ///     BoxFunction};
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let to_string = BoxFunction::new(|x: i32| x.to_string());
    ///
    /// // to_string is moved here
    /// let composed = add.and_then(to_string);
    /// assert_eq!(composed.apply(&20, &22), "42");
    /// // to_string.apply(10); // Would not compile - moved
    /// ```
    ///
    /// ## Preserving original with clone
    ///
    /// ```rust
    /// use qubit_function::{BiFunction, FnBiFunctionOps,
    ///     BoxFunction};
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let to_string = BoxFunction::new(|x: i32| x.to_string());
    ///
    /// // Clone to preserve original
    /// let composed = add.and_then(to_string.clone());
    /// assert_eq!(composed.apply(&20, &22), "42");
    ///
    /// // Original still usable
    /// assert_eq!(to_string.apply(&10), "10");
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxBiFunction<T, U, S>
    where
        Self: 'static,
        S: 'static,
        F: crate::functions::function::Function<R, S> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiFunction::new(move |t: &T, u: &U| after.apply(&self(t, u)))
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
    /// Returns `BoxConditionalBiFunction<T, U, R>`
    ///
    /// # Examples
    ///
    /// ## Basic usage with or_else
    ///
    /// ```rust
    /// use qubit_function::{BiFunction, FnBiFunctionOps};
    ///
    /// let add = |x: &i32, y: &i32| *x + *y;
    /// let conditional = add.when(|x: &i32, y: &i32| *x > 0)
    ///     .or_else(|x: &i32, y: &i32| *x * *y);
    ///
    /// assert_eq!(conditional.apply(&5, &3), 8);
    /// assert_eq!(conditional.apply(&-5, &3), -15);
    /// ```
    ///
    /// ## Preserving bi-predicate with clone
    ///
    /// ```rust
    /// use qubit_function::{BiFunction, FnBiFunctionOps,
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
    fn when<P>(self, predicate: P) -> BoxConditionalBiFunction<T, U, R>
    where
        Self: 'static,
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
        R: 'static,
    {
        BoxBiFunction::new(self).when(predicate)
    }
}

/// Blanket implementation of FnBiFunctionOps for all closures
///
/// Automatically implements `FnBiFunctionOps<T, U, R>` for any type that
/// implements `Fn(&T, &U) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, U, R, F> FnBiFunctionOps<T, U, R> for F where F: Fn(&T, &U) -> R {}

// ============================================================================
// Type Aliases for BinaryOperator (BiFunction<T, T, R>)
// ============================================================================

/// Type alias for `BoxBiFunction<T, T, R>`
///
/// Represents a binary function that takes two values of type `T` and produces
/// a value of type `R`, with single ownership semantics. Similar to Java's
/// `BiFunction<T, T, R>` but with different type parameters.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BoxBinaryFunction, BiFunction};
///
/// let add: BoxBinaryFunction<i32, i32> = BoxBinaryFunction::new(|x, y| *x + *y);
/// assert_eq!(add.apply(&20, &22), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type BoxBinaryFunction<T, R> = BoxBiFunction<T, T, R>;

/// Type alias for `ArcBiFunction<T, T, R>`
///
/// Represents a thread-safe binary function that takes two values of type `T`
/// and produces a value of type `R`. Similar to Java's `BiFunction<T, T, R>`
/// with shared, thread-safe ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{ArcBinaryFunction, BiFunction};
///
/// let multiply: ArcBinaryFunction<i32, i32> = ArcBinaryFunction::new(|x, y| *x * *y);
/// let multiply_clone = multiply.clone();
/// assert_eq!(multiply.apply(&6, &7), 42);
/// assert_eq!(multiply_clone.apply(&6, &7), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type ArcBinaryFunction<T, R> = ArcBiFunction<T, T, R>;

/// Type alias for `RcBiFunction<T, T, R>`
///
/// Represents a single-threaded binary function that takes two values of type `T`
/// and produces a value of type `R`. Similar to Java's `BiFunction<T, T, R>`
/// with shared, single-threaded ownership.
///
/// # Examples
///
/// ```rust
/// use qubit_function::{RcBinaryFunction, BiFunction};
///
/// let max: RcBinaryFunction<i32, i32> = RcBinaryFunction::new(|x, y| if x > y { *x } else { *y });
/// let max_clone = max.clone();
/// assert_eq!(max.apply(&30, &42), 42);
/// assert_eq!(max_clone.apply(&30, &42), 42);
/// ```
///
/// # Author
///
/// Haixing Hu
pub type RcBinaryFunction<T, R> = RcBiFunction<T, T, R>;

// ============================================================================
// BoxConditionalBiFunction - Box-based Conditional BiFunction
// ============================================================================

/// BoxConditionalBiFunction struct
///
/// A conditional bi-function that only executes when a bi-predicate is
/// satisfied. Uses `BoxBiFunction` and `BoxBiPredicate` for single
/// ownership semantics.
///
/// This type is typically created by calling `BoxBiFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements BiFunction**: Can be used anywhere a `BiFunction` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{BiFunction, BoxBiFunction};
///
/// let add = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
/// let multiply = BoxBiFunction::new(|x: &i32, y: &i32| *x * *y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// assert_eq!(conditional.apply(&5, &3), 8);  // when branch executed
/// assert_eq!(conditional.apply(&-5, &3), -15); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalBiFunction<T, U, R> {
    function: BoxBiFunction<T, U, R>,
    predicate: BoxBiPredicate<T, U>,
}

// Implement BoxConditionalBiFunction
impl_box_conditional_function!(
    BoxConditionalBiFunction<T, U, R>,
    BoxBiFunction,
    BiFunction
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(BoxConditionalBiFunction<T, U, R>);

// ============================================================================
// RcConditionalBiFunction - Rc-based Conditional BiFunction
// ============================================================================

/// RcConditionalBiFunction struct
///
/// A single-threaded conditional bi-function that only executes when a
/// bi-predicate is satisfied. Uses `RcBiFunction` and `RcBiPredicate` for
/// shared ownership within a single thread.
///
/// This type is typically created by calling `RcBiFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalBiFunction`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiFunction, RcBiFunction};
///
/// let add = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
/// let multiply = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(&5, &3), 8);
/// assert_eq!(conditional_clone.apply(&-5, &3), -15);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalBiFunction<T, U, R> {
    function: RcBiFunction<T, U, R>,
    predicate: RcBiPredicate<T, U>,
}

// Implement RcConditionalBiFunction
impl_shared_conditional_function!(
    RcConditionalBiFunction<T, U, R>,
    RcBiFunction,
    BiFunction,
    into_rc,
    'static
);

// Use macro to generate Debug and Display implementations
impl_conditional_function_debug_display!(RcConditionalBiFunction<T, U, R>);

// Implement Clone for RcConditionalBiFunction
impl_conditional_function_clone!(RcConditionalBiFunction<T, U, R>);

// ============================================================================
// ArcConditionalBiFunction - Arc-based Conditional BiFunction
// ============================================================================

/// ArcConditionalBiFunction struct
///
/// A thread-safe conditional bi-function that only executes when a
/// bi-predicate is satisfied. Uses `ArcBiFunction` and `ArcBiPredicate` for
/// shared ownership across threads.
///
/// This type is typically created by calling `ArcBiFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only computes when bi-predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{BiFunction, ArcBiFunction};
///
/// let add = ArcBiFunction::new(|x: &i32, y: &i32| *x + *y);
/// let multiply = ArcBiFunction::new(|x: &i32, y: &i32| *x * *y);
/// let conditional = add.when(|x: &i32, y: &i32| *x > 0).or_else(multiply);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(&5, &3), 8);
/// assert_eq!(conditional_clone.apply(&-5, &3), -15);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalBiFunction<T, U, R> {
    function: ArcBiFunction<T, U, R>,
    predicate: ArcBiPredicate<T, U>,
}

// Implement ArcConditionalBiFunction
impl_shared_conditional_function!(
    ArcConditionalBiFunction<T, U, R>,
    ArcBiFunction,
    BiFunction,
    into_arc,
    Send + Sync + 'static
);

// Implement Debug and Display for ArcConditionalBiFunction
impl_conditional_function_debug_display!(ArcConditionalBiFunction<T, U, R>);

// Implement Clone for ArcConditionalBiFunction
impl_conditional_function_clone!(ArcConditionalBiFunction<T, U, R>);
