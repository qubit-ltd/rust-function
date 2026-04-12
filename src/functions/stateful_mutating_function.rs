/*******************************************************************************
 *
 *    Copyright (c) 2025 - 2026.
 *    Haixing Hu, Qubit Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # StatefulMutatingFunction Types
//!
//! Provides Java-like `StatefulMutatingFunction` interface implementations
//! for performing operations that accept a mutable reference, potentially
//! modify internal state, and return a result.
//!
//! It is similar to the `FnMut(&mut T) -> R` trait in the standard library.
//!
//! This module provides a unified `StatefulMutatingFunction` trait and three
//! concrete implementations based on different ownership models:
//!
//! - **`BoxStatefulMutatingFunction<T, R>`**: Box-based single ownership
//!   implementation
//! - **`ArcStatefulMutatingFunction<T, R>`**: Arc<Mutex<>>-based thread-safe
//!   shared ownership implementation
//! - **`RcStatefulMutatingFunction<T, R>`**: Rc<RefCell<>>-based
//!   single-threaded shared ownership implementation
//!
//! # Design Philosophy
//!
//! `StatefulMutatingFunction` extends `MutatingFunction` with the ability to
//! maintain internal state:
//!
//! - **MutatingFunction**: `Fn(&mut T) -> R` - stateless, immutable self
//! - **StatefulMutatingFunction**: `FnMut(&mut T) -> R` - stateful, mutable
//!   self
//!
//! ## Comparison with Related Types
//!
//! | Type | Self | Input | Modifies Self? | Modifies Input? | Returns? |
//! |------|------|-------|----------------|-----------------|----------|
//! | **StatefulFunction** | `&mut self` | `&T` | ✅ | ❌ | ✅ |
//! | **StatefulMutator** | `&mut self` | `&mut T` | ✅ | ✅ | ❌ |
//! | **StatefulMutatingFunction** | `&mut self` | `&mut T` | ✅ | ✅ | ✅ |
//!
//! **Key Insight**: Use `StatefulMutatingFunction` when you need to:
//! - Maintain internal state (counters, accumulators, etc.)
//! - Modify the input value
//! - Return information about the operation
//!
//! # Comparison Table
//!
//! | Feature          | Box | Arc | Rc |
//! |------------------|-----|-----|----|
//! | Ownership        | Single | Shared | Shared |
//! | Cloneable        | ❌ | ✅ | ✅ |
//! | Thread-Safe      | ❌ | ✅ | ❌ |
//! | Interior Mut.    | N/A | Mutex | RefCell |
//! | `and_then` API   | `self` | `&self` | `&self` |
//! | Lock Overhead    | None | Yes | None |
//!
//! # Use Cases
//!
//! ## Common Scenarios
//!
//! - **Stateful counters**: Increment and track modification count
//! - **Accumulators**: Collect statistics while modifying data
//! - **Rate limiters**: Track calls and conditionally modify
//! - **Validators**: Accumulate errors while fixing data
//! - **Stateful transformers**: Apply transformations based on history
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use qubit_function::{BoxStatefulMutatingFunction,
//!                       StatefulMutatingFunction};
//!
//! // Counter that increments value and tracks calls
//! let mut counter = {
//!     let mut call_count = 0;
//!     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
//!         call_count += 1;
//!         *x += 1;
//!         call_count
//!     })
//! };
//!
//! let mut value = 5;
//! assert_eq!(counter.apply(&mut value), 1);
//! assert_eq!(value, 6);
//! assert_eq!(counter.apply(&mut value), 2);
//! assert_eq!(value, 7);
//! ```
//!
//! ## Accumulator Pattern
//!
//! ```rust
//! use qubit_function::{BoxStatefulMutatingFunction,
//!                       StatefulMutatingFunction};
//!
//! // Accumulate sum while doubling values
//! let mut accumulator = {
//!     let mut sum = 0;
//!     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
//!         *x *= 2;
//!         sum += *x;
//!         sum
//!     })
//! };
//!
//! let mut value = 5;
//! assert_eq!(accumulator.apply(&mut value), 10);
//! assert_eq!(value, 10);
//!
//! let mut value2 = 3;
//! assert_eq!(accumulator.apply(&mut value2), 16); // 10 + 6
//! assert_eq!(value2, 6);
//! ```
//!
//! # Author
//!
//! Haixing Hu
use std::cell::RefCell;
use std::rc::Rc;
use std::sync::Arc;

use parking_lot::Mutex;

use crate::functions::{
    function::Function,
    macros::{
        impl_box_conditional_function,
        impl_box_function_methods,
        impl_conditional_function_clone,
        impl_conditional_function_debug_display,
        impl_fn_ops_trait,
        impl_function_clone,
        impl_function_common_methods,
        impl_function_debug_display,
        impl_function_identity_method,
        impl_shared_conditional_function,
        impl_shared_function_methods,
    },
    mutating_function_once::BoxMutatingFunctionOnce,
};
use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_rc_conversions,
};
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};

// =======================================================================
// 1. StatefulMutatingFunction Trait - Unified Interface
// =======================================================================

/// StatefulMutatingFunction trait - Unified stateful mutating function
/// interface
///
/// It is similar to the `FnMut(&mut T) -> R` trait in the standard library.
///
/// Defines the core behavior of all stateful mutating function types.
/// Performs operations that accept a mutable reference, potentially modify
/// both the function's internal state and the input, and return a result.
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnMut(&mut T) -> R`
/// - `BoxStatefulMutatingFunction<T, R>`,
///   `ArcStatefulMutatingFunction<T, R>`, and
///   `RcStatefulMutatingFunction<T, R>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models
/// for operations that need to maintain state while modifying input and
/// returning results. This is useful for scenarios where you need to:
/// - Track statistics or counts during modifications
/// - Accumulate information across multiple calls
/// - Implement stateful validators or transformers
///
/// # Features
///
/// - **Unified Interface**: All stateful mutating function types share the
///   same `apply` method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait
/// - **Type Conversions**: Easy conversion between ownership models
/// - **Generic Programming**: Write functions that work with any stateful
///   mutating function type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction,
///                       BoxStatefulMutatingFunction};
///
/// fn apply_and_log<F: StatefulMutatingFunction<i32, i32>>(
///     func: &mut F,
///     value: i32
/// ) -> i32 {
///     let mut val = value;
///     let result = func.apply(&mut val);
///     println!("Modified: {} -> {}, returned: {}", value, val, result);
///     result
/// }
///
/// let mut counter = {
///     let mut count = 0;
///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
///         count += 1;
///         *x += 1;
///         count
///     })
/// };
/// assert_eq!(apply_and_log(&mut counter, 5), 1);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use qubit_function::StatefulMutatingFunction;
///
/// let mut count = 0;
/// let closure = move |x: &mut i32| {
///     count += 1;
///     *x *= 2;
///     count
/// };
///
/// // Convert to different ownership models
/// let mut box_func = closure.into_box();
/// // let mut rc_func = closure.into_rc();  // closure moved
/// // let mut arc_func = closure.into_arc(); // closure moved
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait StatefulMutatingFunction<T, R> {
    /// Applies the function to the mutable reference and returns a result
    ///
    /// Executes an operation on the given mutable reference, potentially
    /// modifying both the function's internal state and the input, and
    /// returns a result value.
    ///
    /// # Parameters
    ///
    /// * `t` - A mutable reference to the input value
    ///
    /// # Returns
    ///
    /// The computed result value
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulMutatingFunction,
    ///                       BoxStatefulMutatingFunction};
    ///
    /// let mut counter = {
    ///     let mut count = 0;
    ///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
    ///         count += 1;
    ///         let old = *x;
    ///         *x += 1;
    ///         (old, count)
    ///     })
    /// };
    ///
    /// let mut value = 5;
    /// let (old_value, call_count) = counter.apply(&mut value);
    /// assert_eq!(old_value, 5);
    /// assert_eq!(call_count, 1);
    /// assert_eq!(value, 6);
    /// ```
    fn apply(&mut self, t: &mut T) -> R;

    /// Convert this function into a `BoxStatefulMutatingFunction<T, R>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns a
    /// boxed implementation that forwards calls to the original function.
    /// Types that can provide a more efficient conversion may override the
    /// default implementation.
    ///
    /// # Consumption
    ///
    /// This method consumes the function: the original value will no longer
    /// be available after the call. For cloneable functions call `.clone()`
    /// before converting if you need to retain the original instance.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulMutatingFunction<T, R>` that forwards to the original
    /// function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::StatefulMutatingFunction;
    ///
    /// let mut count = 0;
    /// let closure = move |x: &mut i32| {
    ///     count += 1;
    ///     *x *= 2;
    ///     count
    /// };
    /// let mut boxed = closure.into_box();
    /// let mut value = 5;
    /// assert_eq!(boxed.apply(&mut value), 1);
    /// ```
    fn into_box(mut self) -> BoxStatefulMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulMutatingFunction::new(move |t| self.apply(t))
    }

    /// Convert this function into an `RcStatefulMutatingFunction<T, R>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns an
    /// `Rc`-backed function that forwards calls to the original. Override to
    /// provide a more direct or efficient conversion when available.
    ///
    /// # Consumption
    ///
    /// This method consumes the function. If you need to keep the original
    /// instance, clone it prior to calling this method.
    ///
    /// # Returns
    ///
    /// An `RcStatefulMutatingFunction<T, R>` forwarding to the original
    /// function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::StatefulMutatingFunction;
    ///
    /// let mut count = 0;
    /// let closure = move |x: &mut i32| {
    ///     count += 1;
    ///     *x *= 2;
    ///     count
    /// };
    /// let mut rc = closure.into_rc();
    /// let mut value = 5;
    /// assert_eq!(rc.apply(&mut value), 1);
    /// ```
    fn into_rc(mut self) -> RcStatefulMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        RcStatefulMutatingFunction::new(move |t| self.apply(t))
    }

    /// Convert this function into an `ArcStatefulMutatingFunction<T, R>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns an
    /// `Arc`-wrapped, thread-safe function. Types may override the default
    /// implementation to provide a more efficient conversion.
    ///
    /// # Consumption
    ///
    /// This method consumes the function. Clone the instance first if you
    /// need to retain the original for further use.
    ///
    /// # Returns
    ///
    /// An `ArcStatefulMutatingFunction<T, R>` that forwards to the original
    /// function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::StatefulMutatingFunction;
    ///
    /// let mut count = 0;
    /// let closure = move |x: &mut i32| {
    ///     count += 1;
    ///     *x *= 2;
    ///     count
    /// };
    /// let mut arc = closure.into_arc();
    /// let mut value = 5;
    /// assert_eq!(arc.apply(&mut value), 1);
    /// ```
    fn into_arc(mut self) -> ArcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Send + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        ArcStatefulMutatingFunction::new(move |t| self.apply(t))
    }

    /// Consume the function and return an `FnMut(&mut T) -> R` closure.
    ///
    /// The returned closure forwards calls to the original function and is
    /// suitable for use with iterator adapters or other contexts expecting
    /// closures.
    ///
    /// # Consumption
    ///
    /// This method consumes the function. The original instance will not be
    /// available after calling this method.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> R` which forwards to the
    /// original function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulMutatingFunction,
    ///                       BoxStatefulMutatingFunction};
    ///
    /// let func = {
    ///     let mut sum = 0;
    ///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
    ///         *x *= 2;
    ///         sum += *x;
    ///         sum
    ///     })
    /// };
    /// let mut closure = func.into_fn();
    /// let mut value = 5;
    /// assert_eq!(closure(&mut value), 10);
    /// ```
    fn into_fn(mut self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        move |t| self.apply(t)
    }

    /// Create a non-consuming `BoxStatefulMutatingFunction<T, R>` that
    /// forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed function that calls the cloned instance. Override this
    /// method if a more efficient conversion exists.
    ///
    /// # Returns
    ///
    /// A `BoxStatefulMutatingFunction<T, R>` that forwards to a clone of
    /// `self`.
    fn to_box(&self) -> BoxStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Create a non-consuming `RcStatefulMutatingFunction<T, R>` that
    /// forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns an `Rc`-backed function that forwards calls to the clone.
    /// Override to provide a more direct or efficient conversion if needed.
    ///
    /// # Returns
    ///
    /// An `RcStatefulMutatingFunction<T, R>` that forwards to a clone of
    /// `self`.
    fn to_rc(&self) -> RcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Create a non-consuming `ArcStatefulMutatingFunction<T, R>` that
    /// forwards to `self`.
    ///
    /// The default implementation clones `self` (requires
    /// `Clone + Send`) and returns an `Arc`-wrapped function that forwards
    /// calls to the clone. Override when a more efficient conversion is
    /// available.
    ///
    /// # Returns
    ///
    /// An `ArcStatefulMutatingFunction<T, R>` that forwards to a clone of
    /// `self`.
    fn to_arc(&self) -> ArcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + Send + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Create a boxed `FnMut(&mut T) -> R` closure that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed closure that invokes the cloned instance. Override to
    /// provide a more efficient conversion when possible.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T) -> R` which forwards to the
    /// original function.
    fn to_fn(&self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }

    /// Convert to StatefulMutatingFunctionOnce
    ///
    /// **⚠️ Consumes `self`**: The original function will be unavailable
    /// after calling this method.
    ///
    /// Converts a reusable stateful mutating function to a one-time function
    /// that consumes itself on use. This enables passing `StatefulMutatingFunction`
    /// to functions that require `StatefulMutatingFunctionOnce`.
    ///
    /// # Returns
    ///
    /// Returns a `BoxMutatingFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulMutatingFunctionOnce,
    ///                       StatefulMutatingFunction,
    ///                       BoxStatefulMutatingFunction};
    ///
    /// fn takes_once<F: StatefulMutatingFunctionOnce<i32, i32>>(func: F, value: &mut i32) {
    ///     let result = func.apply(value);
    ///     println!("Result: {}", result);
    /// }
    ///
    /// let func = BoxStatefulMutatingFunction::new(|x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// });
    /// let mut value = 5;
    /// takes_once(func.into_once(), &mut value);
    /// ```
    fn into_once(mut self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxMutatingFunctionOnce::new(move |t| self.apply(t))
    }

    /// Convert to StatefulMutatingFunctionOnce without consuming self
    ///
    /// **⚠️ Requires Clone**: This method requires `Self` to implement `Clone`.
    /// Clones the current function and converts the clone to a one-time function.
    ///
    /// # Returns
    ///
    /// Returns a `BoxMutatingFunctionOnce<T, R>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use qubit_function::{StatefulMutatingFunctionOnce,
    ///                       StatefulMutatingFunction,
    ///                       BoxStatefulMutatingFunction};
    ///
    /// fn takes_once<F: StatefulMutatingFunctionOnce<i32, i32>>(func: F, value: &mut i32) {
    ///     let result = func.apply(value);
    ///     println!("Result: {}", result);
    /// }
    ///
    /// let func = BoxStatefulMutatingFunction::new(|x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// });
    /// let mut value = 5;
    /// takes_once(func.to_once(), &mut value);
    /// ```
    fn to_once(&self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_once()
    }
}

// =======================================================================
// 2. Type Aliases
// =======================================================================

/// Type alias for Arc-wrapped stateful mutating function
type ArcStatefulMutatingFunctionFn<T, R> = Arc<Mutex<dyn FnMut(&mut T) -> R + Send + 'static>>;

/// Type alias for Rc-wrapped stateful mutating function
type RcStatefulMutatingFunctionFn<T, R> = Rc<RefCell<dyn FnMut(&mut T) -> R>>;

// =======================================================================
// 3. BoxStatefulMutatingFunction - Single Ownership Implementation
// =======================================================================

/// BoxStatefulMutatingFunction struct
///
/// A stateful mutating function implementation based on
/// `Box<dyn FnMut(&mut T) -> R>` for single ownership scenarios. This is the
/// simplest and most efficient stateful mutating function type when sharing
/// is not required.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Zero Overhead**: No reference counting or locking
/// - **Stateful**: Can modify captured environment (uses `FnMut`)
/// - **Builder Pattern**: Method chaining consumes `self` naturally
/// - **Factory Methods**: Convenient constructors for common patterns
///
/// # Use Cases
///
/// Choose `BoxStatefulMutatingFunction` when:
/// - The function needs to maintain internal state
/// - Building pipelines where ownership naturally flows
/// - No need to share the function across contexts
/// - Performance is critical and no sharing overhead is acceptable
///
/// # Performance
///
/// `BoxStatefulMutatingFunction` has the best performance among the three
/// function types:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction,
///                       BoxStatefulMutatingFunction};
///
/// let mut counter = {
///     let mut count = 0;
///     BoxStatefulMutatingFunction::new(move |x: &mut i32| {
///         count += 1;
///         *x *= 2;
///         count
///     })
/// };
/// let mut value = 5;
/// assert_eq!(counter.apply(&mut value), 1);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxStatefulMutatingFunction<T, R> {
    function: Box<dyn FnMut(&mut T) -> R>,
    name: Option<String>,
}

impl<T, R> BoxStatefulMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxStatefulMutatingFunction<T, R>,
        (FnMut(&mut T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxStatefulMutatingFunction<T, R>,
        BoxConditionalStatefulMutatingFunction,
        Function        // chains a non-mutating function after this mutating function
    );
}

// Generates: Debug and Display implementations for BoxStatefulMutatingFunction<T, R>
impl_function_debug_display!(BoxStatefulMutatingFunction<T, R>);

// Generates: identity() method for BoxStatefulMutatingFunction<T, T>
impl_function_identity_method!(BoxStatefulMutatingFunction<T, T>, mutating);

// Implement StatefulMutatingFunction trait for BoxStatefulMutatingFunction<T, R>
impl<T, R> StatefulMutatingFunction<T, R> for BoxStatefulMutatingFunction<T, R> {
    fn apply(&mut self, t: &mut T) -> R {
        (self.function)(t)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxStatefulMutatingFunction<T, R>,
        RcStatefulMutatingFunction,
        FnMut(&mut T) -> R,
        BoxMutatingFunctionOnce
    );
}

// =======================================================================
// 4. RcStatefulMutatingFunction - Single-Threaded Shared Ownership
// =======================================================================

/// RcStatefulMutatingFunction struct
///
/// A stateful mutating function implementation based on
/// `Rc<RefCell<dyn FnMut(&mut T) -> R>>` for single-threaded shared
/// ownership scenarios. This type allows multiple references to the same
/// function without the overhead of thread safety.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Stateful**: Can modify captured environment (uses `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
/// - **Performance**: More efficient than `ArcStatefulMutatingFunction` (no
///   locking)
///
/// # Use Cases
///
/// Choose `RcStatefulMutatingFunction` when:
/// - The function needs to be shared within a single thread for stateful
///   operations
/// - Thread safety is not required
/// - Performance is important (avoiding lock overhead)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction,
///                       RcStatefulMutatingFunction};
///
/// let counter = {
///     let mut count = 0;
///     RcStatefulMutatingFunction::new(move |x: &mut i32| {
///         count += 1;
///         *x *= 2;
///         count
///     })
/// };
/// let mut clone = counter.clone();
///
/// let mut value = 5;
/// assert_eq!(clone.apply(&mut value), 1);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcStatefulMutatingFunction<T, R> {
    function: RcStatefulMutatingFunctionFn<T, R>,
    name: Option<String>,
}

impl<T, R> RcStatefulMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcStatefulMutatingFunction<T, R>,
        (FnMut(&mut T) -> R + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcStatefulMutatingFunction<T, R>,
        RcConditionalStatefulMutatingFunction,
        into_rc,
        Function,  // chains a non-mutating function after this mutating function
        'static
    );
}

// Generates: Clone implementation for RcStatefulMutatingFunction<T, R>
impl_function_clone!(RcStatefulMutatingFunction<T, R>);

// Generates: Debug and Display implementations for RcStatefulMutatingFunction<T, R>
impl_function_debug_display!(RcStatefulMutatingFunction<T, R>);

// Generates: identity() method for RcStatefulMutatingFunction<T, T>
impl_function_identity_method!(RcStatefulMutatingFunction<T, T>, mutating);

// Implement StatefulMutatingFunction trait for RcStatefulMutatingFunction<T, R>
impl<T, R> StatefulMutatingFunction<T, R> for RcStatefulMutatingFunction<T, R> {
    fn apply(&mut self, t: &mut T) -> R {
        (self.function.borrow_mut())(t)
    }

    // Use macro to implement conversion methods
    impl_rc_conversions!(
        RcStatefulMutatingFunction<T, R>,
        BoxStatefulMutatingFunction,
        BoxMutatingFunctionOnce,
        FnMut(input: &mut T) -> R
    );
}

// =======================================================================
// 5. ArcStatefulMutatingFunction - Thread-Safe Shared Ownership
// =======================================================================

/// ArcStatefulMutatingFunction struct
///
/// A stateful mutating function implementation based on
/// `Arc<Mutex<dyn FnMut(&mut T) -> R + Send>>` for thread-safe shared
/// ownership scenarios. This type allows the function to be safely shared
/// and used across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Stateful**: Can modify captured environment (uses `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
///
/// # Use Cases
///
/// Choose `ArcStatefulMutatingFunction` when:
/// - The function needs to be shared across multiple threads for stateful
///   operations
/// - Concurrent task processing (e.g., thread pools)
/// - Thread safety is required (Send + Sync)
///
/// # Examples
///
/// ```rust
/// use qubit_function::{StatefulMutatingFunction,
///                       ArcStatefulMutatingFunction};
///
/// let counter = {
///     let mut count = 0;
///     ArcStatefulMutatingFunction::new(move |x: &mut i32| {
///         count += 1;
///         *x *= 2;
///         count
///     })
/// };
/// let mut clone = counter.clone();
///
/// let mut value = 5;
/// assert_eq!(clone.apply(&mut value), 1);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcStatefulMutatingFunction<T, R> {
    function: ArcStatefulMutatingFunctionFn<T, R>,
    name: Option<String>,
}

impl<T, R> ArcStatefulMutatingFunction<T, R> {
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcStatefulMutatingFunction<T, R>,
        (FnMut(&mut T) -> R + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcStatefulMutatingFunction<T, R>,
        ArcConditionalStatefulMutatingFunction,
        into_arc,
        Function,  // chains a non-mutating function after this mutating function
        Send + Sync + 'static
    );
}

// Generates: Clone implementation for ArcStatefulMutatingFunction<T, R>
impl_function_clone!(ArcStatefulMutatingFunction<T, R>);

// Generates: Debug and Display implementations for ArcStatefulMutatingFunction<T, R>
impl_function_debug_display!(ArcStatefulMutatingFunction<T, R>);

// Generates: identity() method for ArcStatefulMutatingFunction<T, T>
impl_function_identity_method!(ArcStatefulMutatingFunction<T, T>, mutating);

// Implement StatefulMutatingFunction trait for ArcStatefulMutatingFunction<T, R>
impl<T, R> StatefulMutatingFunction<T, R> for ArcStatefulMutatingFunction<T, R> {
    fn apply(&mut self, t: &mut T) -> R {
        (self.function.lock())(t)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulMutatingFunction<T, R>,
        BoxStatefulMutatingFunction,
        RcStatefulMutatingFunction,
        BoxMutatingFunctionOnce,
        FnMut(input: &mut T) -> R
    );
}

// =======================================================================
// 6. Implement StatefulMutatingFunction trait for closures
// =======================================================================

impl<T, R, F> StatefulMutatingFunction<T, R> for F
where
    F: FnMut(&mut T) -> R,
{
    fn apply(&mut self, input: &mut T) -> R {
        self(input)
    }

    fn into_box(self) -> BoxStatefulMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxStatefulMutatingFunction::new(self)
    }

    fn into_rc(self) -> RcStatefulMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        RcStatefulMutatingFunction::new(self)
    }

    fn into_arc(self) -> ArcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Send + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        ArcStatefulMutatingFunction::new(self)
    }

    fn into_fn(self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        self
    }

    fn to_box(&self) -> BoxStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        let cloned = self.clone();
        BoxStatefulMutatingFunction::new(cloned)
    }

    fn to_rc(&self) -> RcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        let cloned = self.clone();
        RcStatefulMutatingFunction::new(cloned)
    }

    fn to_arc(&self) -> ArcStatefulMutatingFunction<T, R>
    where
        Self: Sized + Clone + Send + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        let cloned = self.clone();
        ArcStatefulMutatingFunction::new(cloned)
    }

    fn to_fn(&self) -> impl FnMut(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone()
    }

    fn into_once(self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxMutatingFunctionOnce::new(self)
    }

    fn to_once(&self) -> BoxMutatingFunctionOnce<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        BoxMutatingFunctionOnce::new(self.clone())
    }
}

// =======================================================================
// 7. Provide extension methods for closures
// =======================================================================

// Generates: FnMutStatefulMutatingFunctionOps trait and blanket implementation
impl_fn_ops_trait!(
    (FnMut(&mut T) -> R),
    FnStatefulMutatingFunctionOps,
    BoxStatefulMutatingFunction,
    StatefulMutatingFunction,
    BoxConditionalStatefulMutatingFunction
);

// ============================================================================
// BoxConditionalStatefulMutatingFunction - Box-based Conditional Stateful Mutating Function
// ============================================================================

/// BoxConditionalStatefulMutatingFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxStatefulMutatingFunction` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxStatefulMutatingFunction::when()` and is
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
/// use qubit_function::{StatefulMutatingFunction, BoxStatefulMutatingFunction};
///
/// let double = BoxStatefulMutatingFunction::new(|x: &mut i32| x * 2);
/// let negate = BoxStatefulMutatingFunction::new(|x: &mut i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// assert_eq!(conditional.apply(5), 10); // when branch executed
/// assert_eq!(conditional.apply(-5), 5); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalStatefulMutatingFunction<T, R> {
    function: BoxStatefulMutatingFunction<T, R>,
    predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalStatefulMutatingFunction<T, R>,
    BoxStatefulMutatingFunction,
    StatefulMutatingFunction
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalStatefulMutatingFunction<T, R>);

// ============================================================================
// RcConditionalStatefulMutatingFunction - Rc-based Conditional Stateful Mutating Function
// ============================================================================

/// RcConditionalStatefulMutatingFunction struct
///
/// A single-threaded conditional function that only executes when a
/// predicate is satisfied. Uses `RcStatefulMutatingFunction` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcStatefulMutatingFunction::when()` and is
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
/// use qubit_function::{StatefulMutatingFunction, RcStatefulMutatingFunction};
///
/// let double = RcStatefulMutatingFunction::new(|x: &mut i32| x * 2);
/// let identity = RcStatefulMutatingFunction::<i32, i32>::identity();
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
pub struct RcConditionalStatefulMutatingFunction<T, R> {
    function: RcStatefulMutatingFunction<T, R>,
    predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalStatefulMutatingFunction<T, R>,
    RcStatefulMutatingFunction,
    StatefulMutatingFunction,
    'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalStatefulMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalStatefulMutatingFunction<T, R>);

// ============================================================================
// ArcConditionalStatefulMutatingFunction - Arc-based Conditional Stateful Mutating Function
// ============================================================================

/// ArcConditionalStatefulMutatingFunction struct
///
/// A thread-safe conditional function that only executes when a predicate is
/// satisfied. Uses `ArcStatefulMutatingFunction` and `ArcPredicate` for shared ownership
/// across threads.
///
/// This type is typically created by calling `ArcStatefulMutatingFunction::when()` and is
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
/// use qubit_function::{StatefulMutatingFunction, ArcStatefulMutatingFunction};
///
/// let double = ArcStatefulMutatingFunction::new(|x: &mut i32| x * 2);
/// let identity = ArcStatefulMutatingFunction::<i32, i32>::identity();
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
pub struct ArcConditionalStatefulMutatingFunction<T, R> {
    function: ArcStatefulMutatingFunction<T, R>,
    predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalStatefulMutatingFunction<T, R>,
    ArcStatefulMutatingFunction,
    StatefulMutatingFunction,
    Send + Sync + 'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalStatefulMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalStatefulMutatingFunction<T, R>);
