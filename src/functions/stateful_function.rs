/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # StatefulFunction Types
//!
//! Provides Rust implementations of stateful function traits for stateful value
//! transformation. StatefulFunctions consume input values (taking ownership) and
//! produce output values while allowing internal state modification.
//!
//! This module provides the `StatefulFunction<T, R>` trait and three implementations:
//!
//! - [`BoxStatefulFunction`]: Single ownership, not cloneable
//! - [`ArcStatefulFunction`]: Thread-safe shared ownership, cloneable
//! - [`RcStatefulFunction`]: Single-threaded shared ownership, cloneable
//!
//! # Author
//!
//! Haixing Hu
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::{
    Arc,
    Mutex,
};

use crate::{
    functions::macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_clone,
        impl_conditional_function_debug_display,
        impl_function_clone,
        impl_function_common_methods,
        impl_function_constant_method,
        impl_function_debug_display,
        impl_function_identity_method,
        impl_shared_conditional_function,
        impl_shared_function_methods,
    },
    predicates::predicate::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    },
};

// ============================================================================
// Core Trait
// ============================================================================

/// StatefulFunction trait - transforms values from type T to type R with state
///
/// Defines the behavior of a stateful transformation: converting a value
/// of type `T` to a value of type `R` by consuming the input while
/// allowing modification of internal state. This is analogous to
/// `FnMut(&T) -> R` in Rust's standard library.
///
/// # Type Parameters
///
/// * `T` - The type of the input value (consumed)
/// * `R` - The type of the output value
///
/// # Author
///
/// Haixing Hu
pub trait StatefulFunction<T, R> {
    /// Applies the mapping to the input value to produce an output value
    ///
    /// # Parameters
    ///
    /// * `t` - The input value to transform (consumed)
    ///
    /// # Returns
    ///
    /// The transformed output value
    fn apply(&mut self, t: &T) -> R;

    /// Converts to BoxStatefulFunction
    ///
    /// **⚠️ Consumes `self`**: The original stateful function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `BoxStatefulFunction<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in a `BoxStatefulFunction` by creating
    /// a new closure that calls `self.apply()`. This provides a zero-cost
    /// abstraction for most use cases.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulFunction, BoxStatefulFunction};
    ///
    /// struct CustomStatefulFunction {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulFunction<i32, i32> for CustomStatefulFunction {
    ///     fn apply(&mut self, input: i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let function = CustomStatefulFunction { multiplier: 0 };
    /// let mut boxed = function.into_box();
    /// assert_eq!(boxed.apply(10), 10);  // 10 * 1
    /// assert_eq!(boxed.apply(10), 20);  // 10 * 2
    /// ```
    fn into_box(mut self) -> BoxStatefulFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulFunction::new(move |t| self.apply(t))
    }

    /// Converts to RcStatefulFunction
    ///
    /// **⚠️ Consumes `self`**: The original stateful function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `RcStatefulFunction<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation first converts to `BoxStatefulFunction` using
    /// `into_box()`, then wraps it in `RcStatefulFunction`. Specific implementations
    /// may override this for better efficiency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulFunction, RcStatefulFunction};
    ///
    /// struct CustomStatefulFunction {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulFunction<i32, i32> for CustomStatefulFunction {
    ///     fn apply(&mut self, input: i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let function = CustomStatefulFunction { multiplier: 0 };
    /// let mut rc_function = function.into_rc();
    /// assert_eq!(rc_function.apply(10), 10);  // 10 * 1
    /// assert_eq!(rc_function.apply(10), 20);  // 10 * 2
    /// ```
    fn into_rc(mut self) -> RcStatefulFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        RcStatefulFunction::new(move |t| self.apply(t))
    }

    /// Converts to ArcStatefulFunction
    ///
    /// **⚠️ Consumes `self`**: The original stateful function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns `ArcStatefulFunction<T, R>`
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps `self` in an `ArcStatefulFunction` by creating
    /// a new closure that calls `self.apply()`. Note that this requires `self`
    /// to implement `Send` due to Arc's thread-safety requirements.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulFunction, ArcStatefulFunction};
    ///
    /// struct CustomStatefulFunction {
    ///     multiplier: i32,
    /// }
    ///
    /// impl StatefulFunction<i32, i32> for CustomStatefulFunction {
    ///     fn apply(&mut self, input: i32) -> i32 {
    ///         self.multiplier += 1;
    ///         input * self.multiplier
    ///     }
    /// }
    ///
    /// let function = CustomStatefulFunction { multiplier: 0 };
    /// let mut arc_function = function.into_arc();
    /// assert_eq!(arc_function.apply(10), 10);  // 10 * 1
    /// assert_eq!(arc_function.apply(10), 20);  // 10 * 2
    /// ```
    fn into_arc(mut self) -> ArcStatefulFunction<T, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: 'static,
    {
        ArcStatefulFunction::new(move |t| self.apply(t))
    }

    /// Converts to a closure implementing `FnMut(&T) -> R`
    ///
    /// **⚠️ Consumes `self`**: The original stateful function becomes unavailable
    /// after calling this method.
    ///
    /// # Returns
    ///
    /// Returns an implementation of `FnMut(&T) -> R`
    ///
    /// # Default Implementation
    ///
    /// The default implementation creates a new closure that calls `self.apply()`.
    /// Specific implementations may override this for better efficiency.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulFunction, BoxStatefulFunction};
    ///
    /// let function = BoxStatefulFunction::new(|x: i32| x * 2);
    /// let mut closure = function.into_fn();
    /// assert_eq!(closure(10), 20);
    /// assert_eq!(closure(15), 30);
    /// ```
    fn into_fn(mut self) -> impl FnMut(&T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        move |t| self.apply(t)
    }

    /// Non-consuming conversion to `BoxStatefulFunction`.
    ///
    /// Default implementation requires `Self: Clone` and wraps a cloned
    /// instance in a `RefCell` so the returned stateful function can mutate state
    /// across calls.
    fn to_box(&self) -> BoxStatefulFunction<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Non-consuming conversion to `RcStatefulFunction`.
    ///
    /// Default implementation clones `self` into an `Rc<RefCell<_>>` so the
    /// resulting stateful function can be shared within a single thread.
    fn to_rc(&self) -> RcStatefulFunction<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Non-consuming conversion to `ArcStatefulFunction` (thread-safe).
    ///
    /// Default implementation requires `Self: Clone + Send + Sync` and wraps
    /// the cloned instance in `Arc<Mutex<_>>` so it can be used across
    /// threads.
    fn to_arc(&self) -> ArcStatefulFunction<T, R>
    where
        Self: Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Non-consuming conversion to a closure (`FnMut(&T) -> R`).
    ///
    /// Default implementation clones `self` into a `RefCell` and returns a
    /// closure that calls `apply` on the interior mutable value.
    fn to_fn(&self) -> impl FnMut(&T) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }
}

// ============================================================================
// BoxStatefulFunction - Box<dyn FnMut(&T) -> R>
// ============================================================================

/// BoxStatefulFunction - stateful function wrapper based on `Box<dyn FnMut>`
///
/// A stateful function wrapper that provides single ownership with reusable stateful
/// transformation. The stateful function consumes the input and can be called
/// multiple times while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Box<dyn FnMut(&T) -> R>`
/// - **Ownership**: Single ownership, cannot be cloned
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync` requirement)
/// - **Statefulness**: Can modify internal state between calls
///
/// # Author
///
/// Haixing Hu
pub struct BoxStatefulFunction<T, R> {
    function: Box<dyn FnMut(&T) -> R>,
    name: Option<String>,
}

impl<T, R> BoxStatefulFunction<T, R>
where
    T: 'static,
    R: 'static,
{
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxStatefulFunction<T, R>,
        (FnMut(&T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxStatefulFunction<T, R>,
        BoxConditionalStatefulFunction,
        StatefulFunction
    );
}

// Generates: constant() method for BoxStatefulFunction<T, R>
impl_function_constant_method!(BoxStatefulFunction<T, R>, 'static);

// Generates: identity() method for BoxStatefulFunction<T, T>
impl_function_identity_method!(BoxStatefulFunction<T, T>);

// Generates: Debug and Display implementations for BoxStatefulFunction<T, R>
impl_function_debug_display!(BoxStatefulFunction<T, R>);

// Implement StatefulFunction trait for BoxStatefulFunction<T, R>
impl<T, R> StatefulFunction<T, R> for BoxStatefulFunction<T, R> {
    fn apply(&mut self, t: &T) -> R {
        (self.function)(t)
    }

    fn into_box(self) -> BoxStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self
    }

    fn into_rc(self) -> RcStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        RcStatefulFunction::new_with_optional_name(self.function, self.name)
    }

    // do NOT override StatefulFunction::into_arc() because BoxStatefulFunction is not Send + Sync
    // and calling BoxStatefulFunction::into_arc() will cause a compile error

    fn into_fn(self) -> impl FnMut(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        self.function
    }

    // do NOT override StatefulFunction::to_xxx() because BoxStatefulFunction is not Clone
    // and calling BoxStatefulFunction::to_xxx() will cause a compile error
}

// ============================================================================
// RcStatefulFunction - Rc<RefCell<dyn FnMut(&T) -> R>>
// ============================================================================

/// RcStatefulFunction - single-threaded function wrapper
///
/// A single-threaded, clonable function wrapper optimized for scenarios
/// that require sharing without thread-safety overhead.
///
/// # Features
///
/// - **Based on**: `Rc<RefCell<dyn FnMut(&T) -> R>>`
/// - **Ownership**: Shared ownership via reference counting (non-atomic)
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Not thread-safe (no `Send + Sync`)
/// - **Clonable**: Cheap cloning via `Rc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
/// # Author
///
/// Haixing Hu
pub struct RcStatefulFunction<T, R> {
    function: RcStatefulFn<T, R>,
    name: Option<String>,
}

type RcStatefulFn<T, R> = Rc<RefCell<dyn FnMut(&T) -> R>>;

impl<T, R> RcStatefulFunction<T, R>
where
    T: 'static,
    R: 'static,
{
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcStatefulFunction<T, R>,
        (FnMut(&T) -> R + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcStatefulFunction<T, R>,
        RcConditionalStatefulFunction,
        into_rc,
        StatefulFunction,
        'static
    );
}

// Generates: constant() method for RcStatefulFunction<T, R>
impl_function_constant_method!(RcStatefulFunction<T, R>, 'static);

// Generates: identity() method for RcStatefulFunction<T, T>
impl_function_identity_method!(RcStatefulFunction<T, T>);

// Generates: Clone implementation for RcStatefulFunction<T, R>
impl_function_clone!(RcStatefulFunction<T, R>);

// Generates: Debug and Display implementations for RcStatefulFunction<T, R>
impl_function_debug_display!(RcStatefulFunction<T, R>);

// Implement StatefulFunction trait for RcStatefulFunction<T, R>
impl<T, R> StatefulFunction<T, R> for RcStatefulFunction<T, R> {
    fn apply(&mut self, t: &T) -> R {
        (self.function.borrow_mut())(t)
    }

    fn into_box(self) -> BoxStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        BoxStatefulFunction::new_with_optional_name(
            move |t| self.function.borrow_mut()(t),
            self.name,
        )
    }

    fn into_rc(self) -> RcStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self
    }

    // do NOT override StatefulFunction::into_arc() because RcStatefulFunction is not Send + Sync
    // and calling RcStatefulFunction::into_arc() will cause a compile error

    fn into_fn(self) -> impl FnMut(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        move |t| (self.function.borrow_mut())(t)
    }

    // Override with optimized implementation: clone the Rc (cheap)
    fn to_box(&self) -> BoxStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        let self_name = self.name.clone();
        BoxStatefulFunction::new_with_optional_name(move |t| self_fn.borrow_mut()(t), self_name)
    }

    // Override with zero-cost implementation: clone itself
    fn to_rc(&self) -> RcStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self.clone()
    }

    // do NOT override RcFunction::to_arc() because RcFunction is not Send + Sync
    // and calling RcFunction::to_arc() will cause a compile error

    // Override with optimized implementation: clone the Rc (cheap)
    fn to_fn(&self) -> impl FnMut(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        move |t| self_fn.borrow_mut()(t)
    }
}

// ============================================================================
// ArcStatefulFunction - Arc<Mutex<dyn FnMut(&T) -> R + Send>>
// ============================================================================

/// ArcStatefulFunction - thread-safe function wrapper
///
/// A thread-safe, clonable function wrapper suitable for multi-threaded
/// scenarios. Can be called multiple times and shared across threads
/// while maintaining internal state.
///
/// # Features
///
/// - **Based on**: `Arc<Mutex<dyn FnMut(&T) -> R + Send>>`
/// - **Ownership**: Shared ownership via reference counting
/// - **Reusability**: Can be called multiple times (each call consumes
///   its input)
/// - **Thread Safety**: Thread-safe (`Send` required)
/// - **Clonable**: Cheap cloning via `Arc::clone`
/// - **Statefulness**: Can modify internal state between calls
///
/// # Author
///
/// Haixing Hu
pub struct ArcStatefulFunction<T, R> {
    function: ArcStatefulFn<T, R>,
    name: Option<String>,
}

type ArcStatefulFn<T, R> = Arc<Mutex<dyn FnMut(&T) -> R>>;

impl<T, R> ArcStatefulFunction<T, R>
where
    T: 'static,
    R: 'static,
{
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcStatefulFunction<T, R>,
        (FnMut(&T) -> R + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcStatefulFunction<T, R>,
        ArcConditionalStatefulFunction,
        into_arc,
        StatefulFunction,
        Send + Sync + 'static
    );
}

// Generates: constant() method for ArcStatefulFunction<T, R>
impl_function_constant_method!(ArcStatefulFunction<T, R>, Send + Sync + 'static);

// Generates: identity() method for ArcStatefulFunction<T, T>
impl_function_identity_method!(ArcStatefulFunction<T, T>);

// Generates: Clone implementation for ArcStatefulFunction<T, R>
impl_function_clone!(ArcStatefulFunction<T, R>);

// Generates: Debug and Display implementations for ArcStatefulFunction<T, R>
impl_function_debug_display!(ArcStatefulFunction<T, R>);

// Implement StatefulFunction trait for ArcStatefulFunction<T, R>
impl<T, R> StatefulFunction<T, R> for ArcStatefulFunction<T, R> {
    fn apply(&mut self, t: &T) -> R {
        (self.function.lock().unwrap())(t)
    }

    fn into_box(self) -> BoxStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        BoxStatefulFunction::new_with_optional_name(
            move |t| self.function.lock().unwrap()(t),
            self.name,
        )
    }

    fn into_rc(self) -> RcStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        RcStatefulFunction::new_with_optional_name(
            move |t| self.function.lock().unwrap()(t),
            self.name,
        )
    }

    fn into_arc(self) -> ArcStatefulFunction<T, R>
    where
        T: Send + Sync + 'static,
        R: 'static,
    {
        self
    }

    fn into_fn(self) -> impl FnMut(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        move |t| (self.function.lock().unwrap())(t)
    }

    fn to_box(&self) -> BoxStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        let self_name = self.name.clone();
        BoxStatefulFunction::new_with_optional_name(move |t| self_fn.lock().unwrap()(t), self_name)
    }

    fn to_rc(&self) -> RcStatefulFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        let self_name = self.name.clone();
        RcStatefulFunction::new_with_optional_name(move |t| self_fn.lock().unwrap()(t), self_name)
    }

    fn to_arc(&self) -> ArcStatefulFunction<T, R>
    where
        T: Send + Sync + 'static,
        R: Send + 'static,
    {
        self.clone()
    }

    fn to_fn(&self) -> impl FnMut(&T) -> R
    where
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        move |t| self_fn.lock().unwrap()(t)
    }
}

// ============================================================================
// Blanket implementation for standard FnMut trait
// ============================================================================

/// Implement StatefulFunction<T, R> for any type that implements FnMut(&T) -> R
///
/// This allows closures to be used directly with our StatefulFunction trait
/// without wrapping.
///
/// # Examples
///
/// ```rust
/// use prism3_function::StatefulFunction;
///
/// let mut counter = 0;
/// let mut function = |x: i32| {
///     counter += 1;
///     x + counter
/// };
///
/// assert_eq!(function.apply(10), 11);
/// assert_eq!(function.apply(10), 12);
/// ```
///
/// # Author
///
/// Haixing Hu
impl<F, T, R> StatefulFunction<T, R> for F
where
    F: FnMut(&T) -> R,
    T: 'static,
    R: 'static,
{
    fn apply(&mut self, t: &T) -> R {
        self(t)
    }

    fn into_box(self) -> BoxStatefulFunction<T, R>
    where
        Self: Sized + 'static,
    {
        BoxStatefulFunction::new(self)
    }

    fn into_rc(self) -> RcStatefulFunction<T, R>
    where
        Self: Sized + 'static,
    {
        RcStatefulFunction::new(self)
    }

    fn into_arc(self) -> ArcStatefulFunction<T, R>
    where
        Self: Sized + Send + 'static,
        T: Send + Sync + 'static,
        R: 'static,
    {
        ArcStatefulFunction::new(self)
    }

    fn into_fn(self) -> impl FnMut(&T) -> R
    where
        Self: Sized + 'static,
    {
        self
    }

    fn to_box(&self) -> BoxStatefulFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_box()
    }

    fn to_rc(&self) -> RcStatefulFunction<T, R>
    where
        Self: Sized + Clone + 'static,
    {
        self.clone().into_rc()
    }

    fn to_arc(&self) -> ArcStatefulFunction<T, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        T: Send + Sync + 'static,
        R: 'static,
    {
        self.clone().into_arc()
    }

    fn to_fn(&self) -> impl FnMut(&T) -> R
    where
        Self: Sized + Clone + 'static,
    {
        self.clone()
    }
}

// ============================================================================
// FnStatefulFunctionOps - Extension trait for closure functions
// ============================================================================

/// Extension trait for closures implementing `FnMut(&T) -> R`
///
/// Provides composition methods (`and_then`, `compose`, `when`) for
/// closures without requiring explicit wrapping in `BoxStatefulFunction`,
/// `RcStatefulFunction`, or `ArcStatefulFunction`.
///
/// This trait is automatically implemented for all closures that
/// implement `FnMut(&T) -> R`.
///
/// # Design Rationale
///
/// While closures automatically implement `StatefulFunction<T, R>` through blanket
/// implementation, they don't have access to instance methods like
/// `and_then`, `compose`, and `when`. This extension trait provides
/// those methods, returning `BoxStatefulFunction` for maximum flexibility.
///
/// # Examples
///
/// ## Chain composition with and_then
///
/// ```rust
/// use prism3_function::{StatefulFunction, FnStatefulFunctionOps};
///
/// let mut counter1 = 0;
/// let function1 = move |x: i32| {
///     counter1 += 1;
///     x + counter1
/// };
///
/// let mut counter2 = 0;
/// let function2 = move |x: i32| {
///     counter2 += 1;
///     x * counter2
/// };
///
/// let mut composed = function1.and_then(function2);
/// assert_eq!(composed.apply(10), 11);  // (10 + 1) * 1
/// ```
///
/// ## Reverse composition with compose
///
/// ```rust
/// use prism3_function::{StatefulFunction, FnStatefulFunctionOps};
///
/// let mut counter = 0;
/// let function = move |x: i32| {
///     counter += 1;
///     x * counter
/// };
///
/// let mut composed = function.compose(|x: i32| x + 1);
/// assert_eq!(composed.apply(10), 11); // (10 + 1) * 1
/// ```
///
/// ## Conditional mapping with when
///
/// ```rust
/// use prism3_function::{StatefulFunction, FnStatefulFunctionOps};
///
/// let mut function = (|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// assert_eq!(function.apply(5), 10);
/// assert_eq!(function.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnStatefulFunctionOps<T, R>: FnMut(&T) -> R + Sized + 'static {
    /// Chain composition - applies self first, then after
    ///
    /// Creates a new function that applies this function first, then applies
    /// the after function to the result. Consumes self and returns a
    /// `BoxStatefulFunction`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The output type of the after function
    /// * `F` - The type of the after function (must implement StatefulFunction<R, S>)
    ///
    /// # Parameters
    ///
    /// * `after` - The function to apply after self. Can be:
    ///   - A closure: `|x: R| -> S`
    ///   - A `BoxStatefulFunction<R, S>`
    ///   - An `RcStatefulFunction<R, S>`
    ///   - An `ArcStatefulFunction<R, S>`
    ///   - Any type implementing `StatefulFunction<R, S>`
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulFunction<T, S>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulFunction, FnStatefulFunctionOps, BoxStatefulFunction};
    ///
    /// let mut counter1 = 0;
    /// let function1 = move |x: i32| {
    ///     counter1 += 1;
    ///     x + counter1
    /// };
    ///
    /// let mut counter2 = 0;
    /// let function2 = BoxStatefulFunction::new(move |x: i32| {
    ///     counter2 += 1;
    ///     x * counter2
    /// });
    ///
    /// let mut composed = function1.and_then(function2);
    /// assert_eq!(composed.apply(10), 11);
    /// ```
    fn and_then<S, F>(self, after: F) -> BoxStatefulFunction<T, S>
    where
        S: 'static,
        F: StatefulFunction<R, S> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulFunction::new(self).and_then(after)
    }

    /// Reverse composition - applies before first, then self
    ///
    /// Creates a new function that applies the before function first, then
    /// applies this function to the result. Consumes self and returns a
    /// `BoxStatefulFunction`.
    ///
    /// # Type Parameters
    ///
    /// * `S` - The input type of the before function
    /// * `F` - The type of the before function (must implement StatefulFunction<S, T>)
    ///
    /// # Parameters
    ///
    /// * `before` - The function to apply before self. Can be:
    ///   - A closure: `|x: S| -> T`
    ///   - A `BoxStatefulFunction<S, T>`
    ///   - An `RcStatefulFunction<S, T>`
    ///   - An `ArcStatefulFunction<S, T>`
    ///   - Any type implementing `StatefulFunction<S, T>`
    ///
    /// # Returns
    ///
    /// A new `BoxStatefulFunction<S, R>` representing the composition
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulFunction, FnStatefulFunctionOps, BoxStatefulFunction};
    ///
    /// let mut counter = 0;
    /// let function = move |x: i32| {
    ///     counter += 1;
    ///     x * counter
    /// };
    ///
    /// let before = BoxStatefulFunction::new(|x: i32| x + 1);
    ///
    /// let mut composed = function.compose(before);
    /// assert_eq!(composed.apply(10), 11); // (10 + 1) * 1
    /// ```
    fn compose<S, F>(self, before: F) -> BoxStatefulFunction<S, R>
    where
        S: 'static,
        F: StatefulFunction<S, T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulFunction::new(self).compose(before)
    }

    /// Creates a conditional function
    ///
    /// Returns a function that only executes when a predicate is satisfied.
    /// You must call `or_else()` to provide an alternative function for
    /// when the condition is not satisfied.
    ///
    /// # Parameters
    ///
    /// * `predicate` - The condition to check. Can be:
    ///   - A closure: `|x: &T| -> bool`
    ///   - A function pointer: `fn(&T) -> bool`
    ///   - A `BoxPredicate<T>`
    ///   - An `RcPredicate<T>`
    ///   - An `ArcPredicate<T>`
    ///   - Any type implementing `Predicate<T>`
    ///
    /// # Returns
    ///
    /// Returns `BoxConditionalStatefulFunction<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulFunction, FnStatefulFunctionOps};
    ///
    /// let mut function = (|x: i32| x * 2)
    ///     .when(|x: &i32| *x > 0)
    ///     .or_else(|x: i32| -x);
    ///
    /// assert_eq!(function.apply(5), 10);
    /// assert_eq!(function.apply(-5), 5);
    /// ```
    fn when<P>(self, predicate: P) -> BoxConditionalStatefulFunction<T, R>
    where
        P: Predicate<T> + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulFunction::new(self).when(predicate)
    }
}

/// Blanket implementation of FnStatefulFunctionOps for all closures
///
/// Automatically implements `FnStatefulFunctionOps<T, R>` for any type that
/// implements `FnMut(&T) -> R`.
///
/// # Author
///
/// Haixing Hu
impl<T, R, F> FnStatefulFunctionOps<T, R> for F where F: FnMut(&T) -> R + 'static {}

// ============================================================================
// BoxConditionalStatefulFunction - Box-based Conditional StatefulFunction
// ============================================================================

/// BoxConditionalStatefulFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxStatefulFunction` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxStatefulFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else
///   logic
/// - **Implements StatefulFunction**: Can be used anywhere a `StatefulFunction` is expected
///
/// # Examples
///
/// ```rust
/// use prism3_function::{StatefulFunction, BoxStatefulFunction};
///
/// let mut high_count = 0;
/// let mut low_count = 0;
///
/// let mut function = BoxStatefulFunction::new(move |x: i32| {
///     high_count += 1;
///     x * 2
/// })
/// .when(|x: &i32| *x >= 10)
/// .or_else(move |x| {
///     low_count += 1;
///     x + 1
/// });
///
/// assert_eq!(function.apply(15), 30); // when branch executed
/// assert_eq!(function.apply(5), 6);   // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalStatefulFunction<T, R> {
    function: BoxStatefulFunction<T, R>,
    predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalStatefulFunction<T, R>,
    BoxStatefulFunction,
    StatefulFunction
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalStatefulFunction<T, R>);

// ============================================================================
// RcConditionalStatefulFunction - Rc-based Conditional StatefulFunction
// ============================================================================

/// RcConditionalStatefulFunction struct
///
/// A single-threaded conditional function that only executes when a
/// predicate is satisfied. Uses `RcStatefulFunction` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcStatefulFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalStatefulFunction`
///
/// # Examples
///
/// ```rust
/// use prism3_function::{StatefulFunction, RcStatefulFunction};
///
/// let mut function = RcStatefulFunction::new(|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// let mut function_clone = function.clone();
///
/// assert_eq!(function.apply(5), 10);
/// assert_eq!(function_clone.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalStatefulFunction<T, R> {
    function: RcStatefulFunction<T, R>,
    predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalStatefulFunction<T, R>,
    RcStatefulFunction,
    StatefulFunction,
    'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalStatefulFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalStatefulFunction<T, R>);

// ============================================================================
// ArcConditionalStatefulFunction - Arc-based Conditional StatefulFunction
// ============================================================================

/// ArcConditionalStatefulFunction struct
///
/// A thread-safe conditional function that only executes when a predicate
/// is satisfied. Uses `ArcStatefulFunction` and `ArcPredicate` for shared
/// ownership across threads.
///
/// This type is typically created by calling `ArcStatefulFunction::when()` and is
/// designed to work with the `or_else()` method to create if-then-else
/// logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send`, safe for concurrent use
/// - **Conditional Execution**: Only maps when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else
///   logic
///
/// # Examples
///
/// ```rust
/// use prism3_function::{StatefulFunction, ArcStatefulFunction};
///
/// let mut function = ArcStatefulFunction::new(|x: i32| x * 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: i32| -x);
///
/// let mut function_clone = function.clone();
///
/// assert_eq!(function.apply(5), 10);
/// assert_eq!(function_clone.apply(-5), 5);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalStatefulFunction<T, R> {
    function: ArcStatefulFunction<T, R>,
    predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalStatefulFunction<T, R>,
    ArcStatefulFunction,
    StatefulFunction,
    Send + Sync + 'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalStatefulFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalStatefulFunction<T, R>);
