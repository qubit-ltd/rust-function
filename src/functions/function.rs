/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Function Types
//!
//! Provides Rust implementations of function traits for computing output values
//! from input references. Functions borrow input values (not consuming them)
//! and produce output values.
//!
//! It is similar to the `Fn(&T) -> R` trait in the standard library.
//!
//! This module provides the `Function<T, R>` trait and three
//! implementations:
//!
//! - [`BoxFunction`]: Single ownership, not cloneable
//! - [`ArcFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcFunction`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu
use std::rc::Rc;
use std::sync::Arc;

use crate::functions::{
    function_once::BoxFunctionOnce,
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_clone,
        impl_conditional_function_debug_display,
        impl_fn_ops_trait,
        impl_function_clone,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
        impl_function_identity_method,
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
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};

// ============================================================================
// Core Trait
// ============================================================================

/// Function trait - computes output from input reference
///
/// Defines the behavior of a function: computing a value of type `R`
/// from a reference to type `T` without consuming the input. This is analogous to
/// `Fn(&T) -> R` in Rust's standard library, similar to Java's `Function<T, R>`.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (borrowed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait Function<T, R> {
    /// Applies the function to the input reference to produce an output value
    ///
    /// # Parameters
    ///
    /// * `t` - Reference to the input value
    ///
    /// # Returns
    ///
    /// The computed output value
    fn apply(&self, t: &T) -> R;

    /// Converts to BoxFunction
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `Box` and creates a
    /// `BoxFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunction<T, R>`
    fn into_box(self) -> BoxFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxFunction::new(move |t| self.apply(t))
    }

    /// Converts to RcFunction
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Rc` and creates an
    /// `RcFunction`. Types can override this method to provide more
    /// efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcFunction<T, R>`
    fn into_rc(self) -> RcFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        RcFunction::new(move |t| self.apply(t))
    }

    /// Converts to ArcFunction
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `Arc` and creates
    /// an `ArcFunction`. Types can override this method to provide
    /// more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcFunction<T, R>`
    fn into_arc(self) -> ArcFunction<T, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: 'static,
    {
        ArcFunction::new(move |t| self.apply(t))
    }

    /// Converts function to a closure
    ///
    /// **⚠️ Consumes `self`**: The original function becomes
    /// unavailable after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a closure that captures `self`
    /// and calls its `transform` method. Types can override this method
    /// to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns a closure that implements `Fn(&T) -> R`
    fn into_fn(self) -> impl Fn(&T) -> R
    where
        Self: Sized + 'static,
    {
        move |t| self.apply(t)
    }

    /// Converts to FunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original function becomes unavailable after calling this method.
    ///
    /// Converts a reusable function to a one-time function that consumes itself on use.
    /// This enables passing `Function` to functions that require `FunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// fn takes_once<F: FunctionOnce<i32, i32>>(func: F, value: &i32) -> i32 {
    ///     func.apply(value)
    /// }
    ///
    /// let func = BoxFunction::new(|x: &i32| x * 2);
    /// let result = takes_once(func.into_once(), &5);
    /// assert_eq!(result, 10);
    /// ```
    fn into_once(self) -> BoxFunctionOnce<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxFunctionOnce::new(move |t| self.apply(t))
    }

    /// Converts to BoxFunction without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `BoxFunction` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `BoxFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: i32| x * 2);
    /// let boxed = double.to_box();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(boxed.apply(21), 42);
    /// ```
    fn to_box(&self) -> BoxFunction<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Converts to RcFunction without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `RcFunction` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `RcFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: i32| x * 2);
    /// let rc = double.to_rc();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(rc.apply(21), 42);
    /// ```
    fn to_rc(&self) -> RcFunction<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Converts to ArcFunction without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
    /// after calling this method.
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new `ArcFunction` that
    /// captures a reference-counted clone. Types implementing `Clone`
    /// can override this method to provide more efficient conversions.
    ///
    /// # Returns
    ///
    /// Returns `ArcFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: i32| x * 2);
    /// let arc = double.to_arc();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(arc.apply(21), 42);
    /// ```
    fn to_arc(&self) -> ArcFunction<T, R>
    where
        Self: Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: Send + Sync + 'static,
    {
        self.clone().into_arc()
    }

    /// Converts function to a closure without consuming self
    ///
    /// **📌 Borrows `&self`**: The original function remains usable
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
    /// Returns a closure that implements `Fn(&T) -> R`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{ArcFunction, Function};
    ///
    /// let double = ArcFunction::new(|x: i32| x * 2);
    /// let closure = double.to_fn();
    ///
    /// // Original function still usable
    /// assert_eq!(double.apply(21), 42);
    /// assert_eq!(closure(21), 42);
    /// ```
    fn to_fn(&self) -> impl Fn(&T) -> R
    where
        Self: Clone + 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to FunctionOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current function and converts the clone to a one-time function.
    ///
    /// # Returns
    ///
    /// Returns a `BoxFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    ///
    /// fn takes_once<F: FunctionOnce<i32, i32>>(func: F, value: &i32) -> i32 {
    ///     func.apply(value)
    /// }
    ///
    /// let func = BoxFunction::new(|x: &i32| x * 2);
    /// let result = takes_once(func.to_once(), &5);
    /// assert_eq!(result, 10);
    /// ```
    fn to_once(&self) -> BoxFunctionOnce<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_once()
    }
}

// ============================================================================
// BoxFunction - Box<dyn Fn(&T) -> R>
// ============================================================================

/// BoxFunction - function wrapper based on `Box<dyn Fn>`
///
/// A function wrapper that provides single ownership with reusable
/// transformation. The function consumes the input and can be called
/// multiple times.
///
/// # Features
///
/// - **Based on**: `Box<dyn Fn(&T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
///
/// # Author
///
/// Haixing Hu
pub struct BoxFunction<T, R> {
    function: Box<dyn Fn(&T) -> R>,
    name: Option<String>,
}

impl<T, R> BoxFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxFunction<T, R>,
        (Fn(&T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxFunction<T, R>,
        BoxConditionalFunction,
        Function
    );
}

// Generates: constant() method for BoxFunction<T, R>
impl_function_constant_method!(BoxFunction<T, R>, 'static);

// Generates: identity() method for BoxFunction<T, T>
impl_function_identity_method!(BoxFunction<T, T>);

// Generates: Debug and Display implementations for BoxFunction<T, R>
impl_function_debug_display!(BoxFunction<T, R>);

// Implement Function trait for BoxFunction<T, R>
impl<T, R> Function<T, R> for BoxFunction<T, R> {
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxFunction<T, R>,
        RcFunction,
        Fn(&T) -> R,
        BoxFunctionOnce
    );
}

// ============================================================================
// RcFunction - Rc<dyn Fn(&T) -> R>
// ============================================================================

/// RcFunction - single-threaded function wrapper
///
/// A single-threaded, clonable function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<dyn Fn(&T) -> R>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct RcFunction<T, R> {
    function: Rc<dyn Fn(&T) -> R>,
    name: Option<String>,
}

impl<T, R> RcFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcFunction<T, R>,
        (Fn(&T) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcFunction<T, R>,
        RcConditionalFunction,
        into_rc,
        Function,
        'static
    );
}

// Generates: constant() method for RcFunction<T, R>
impl_function_constant_method!(RcFunction<T, R>, 'static);

// Generates: identity() method for RcFunction<T, T>
impl_function_identity_method!(RcFunction<T, T>);

// Generates: Clone implementation for RcFunction<T, R>
impl_function_clone!(RcFunction<T, R>);

// Generates: Debug and Display implementations for RcFunction<T, R>
impl_function_debug_display!(RcFunction<T, R>);

// Implement Function trait for RcFunction<T, R>
impl<T, R> Function<T, R> for RcFunction<T, R> {
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }

    // Use macro to implement conversion methods
    impl_rc_conversions!(
        RcFunction<T, R>,
        BoxFunction,
        BoxFunctionOnce,
        Fn(t: &T) -> R
    );
}

// ============================================================================
// ArcFunction - Arc<dyn Fn(&T) -> R + Send + Sync>
// ============================================================================

/// ArcFunction - thread-safe function wrapper
///
/// A thread-safe, clonable function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads.
///
/// # Features
///
/// - **Based on**: `Arc<dyn Fn(&T) -> R + Send + Sync>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes its
///   input)
/// - **Thread Safety**: Thread-safe (`Send + Sync` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
///
/// # Author
///
/// Haixing Hu
pub struct ArcFunction<T, R> {
    function: Arc<dyn Fn(&T) -> R + Send + Sync>,
    name: Option<String>,
}

impl<T, R> ArcFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcFunction<T, R>,
        (Fn(&T) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcFunction<T, R>,
        ArcConditionalFunction,
        into_arc,
        Function,
        Send + Sync + 'static
    );
}

// Generates: constant() method for ArcFunction<T, R>
impl_function_constant_method!(ArcFunction<T, R>, Send + Sync + 'static);

// Generates: identity() method for ArcFunction<T, T>
impl_function_identity_method!(ArcFunction<T, T>);

// Generates: Clone implementation for ArcFunction<T, R>
impl_function_clone!(ArcFunction<T, R>);

// Generates: Debug and Display implementations for ArcFunction<T, R>
impl_function_debug_display!(ArcFunction<T, R>);

// Implement Function trait for ArcFunction<T, R>
impl<T, R> Function<T, R> for ArcFunction<T, R> {
    fn apply(&self, t: &T) -> R {
        (self.function)(t)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcFunction<T, R>,
        BoxFunction,
        RcFunction,
        BoxFunctionOnce,
        Fn(t: &T) -> R
    );
}

// ============================================================================
// Blanket implementation for standard Fn trait
// ============================================================================

// Implement Function<T, R> for any type that implements Fn(&T) -> R
impl_closure_trait!(
    Function<T, R>,
    apply,
    BoxFunctionOnce,
    Fn(input: &T) -> R
);

// ============================================================================
// FnFunctionOps - Extension trait for closure functions
// ============================================================================

// Generates: FnFunctionOps trait and blanket implementation
impl_fn_ops_trait!(
    (Fn(&T) -> R),
    FnFunctionOps,
    BoxFunction,
    Function,
    BoxConditionalFunction
);

// ============================================================================
// BoxConditionalFunction - Box-based Conditional Function
// ============================================================================

/// BoxConditionalFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxFunction` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements Function**: Can be used anywhere a `Function` is expected
///
/// # Examples
///
/// ## With or_else Branch
///
/// ```rust
/// use qubit_function::{Function, BoxFunction};
///
/// let double = BoxFunction::new(|x: i32| x * 2);
/// let negate = BoxFunction::new(|x: i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// assert_eq!(conditional.apply(5), 10); // when branch executed
/// assert_eq!(conditional.apply(-5), 5); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalFunction<T, R> {
    function: BoxFunction<T, R>,
    predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalFunction<T, R>,
    BoxFunction,
    Function
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalFunction<T, R>);

// ============================================================================
// RcConditionalFunction - Rc-based Conditional Function
// ============================================================================

/// RcConditionalFunction struct
///
/// A single-threaded conditional function that only executes when a
/// predicate is satisfied. Uses `RcFunction` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalFunction`
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Function, RcFunction};
///
/// let double = RcFunction::new(|x: i32| x * 2);
/// let identity = RcFunction::<i32, i32>::identity();
/// let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5), 10);
/// assert_eq!(conditional_clone.apply(-5), -5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalFunction<T, R> {
    function: RcFunction<T, R>,
    predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalFunction<T, R>,
    RcFunction,
    Function,
    'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalFunction<T, R>);

// ============================================================================
// ArcConditionalFunction - Arc-based Conditional Function
// ============================================================================

/// ArcConditionalFunction struct
///
/// A thread-safe conditional function that only executes when a predicate is
/// satisfied. Uses `ArcFunction` and `ArcPredicate` for shared ownership
/// across threads.
///
/// This type is typically created by calling `ArcFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only transforms when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use qubit_function::{Function, ArcFunction};
///
/// let double = ArcFunction::new(|x: i32| x * 2);
/// let identity = ArcFunction::<i32, i32>::identity();
/// let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
///
/// let conditional_clone = conditional.clone();
///
/// assert_eq!(conditional.apply(5), 10);
/// assert_eq!(conditional_clone.apply(-5), -5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalFunction<T, R> {
    function: ArcFunction<T, R>,
    predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalFunction<T, R>,
    ArcFunction,
    Function,
    Send + Sync + 'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalFunction<T, R>);
