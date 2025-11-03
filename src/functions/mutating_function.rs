/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # MutatingFunction Types
//!
//! Provides Java-like `MutatingFunction` interface implementations for
//! performing operations that accept a mutable reference and return a result.
//!
//! This module provides a unified `MutatingFunction` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxMutatingFunction<T, R>`**: Box-based single ownership
//!   implementation
//! - **`ArcMutatingFunction<T, R>`**: Arc-based thread-safe shared ownership
//!   implementation
//! - **`RcMutatingFunction<T, R>`**: Rc-based single-threaded shared
//!   ownership implementation
//!
//! # Design Philosophy
//!
//! `MutatingFunction` bridges the gap between `Function` and `Mutator`:
//!
//! - **Function**: `Fn(&T) -> R` - reads input, returns result
//! - **Mutator**: `Fn(&mut T)` - modifies input, no return
//! - **MutatingFunction**: `Fn(&mut T) -> R` - modifies input AND returns
//!   result
//!
//! ## Comparison with Related Types
//!
//! | Type | Input | Modifies? | Returns? | Use Cases |
//! |------|-------|-----------|----------|-----------|
//! | **Function** | `&T` | ❌ | ✅ | Read-only transform |
//! | **Mutator** | `&mut T` | ✅ | ❌ | In-place modification |
//! | **MutatingFunction** | `&mut T` | ✅ | ✅ | Modify + return info |
//! | **Transformer** | `T` | N/A | ✅ | Consume + transform |
//!
//! **Key Insight**: Use `MutatingFunction` when you need to both modify the
//! input and return information about the modification or the previous state.
//!
//! # Comparison Table
//!
//! | Feature          | Box | Arc | Rc |
//! |------------------|-----|-----|----|
//! | Ownership        | Single | Shared | Shared |
//! | Cloneable        | ❌ | ✅ | ✅ |
//! | Thread-Safe      | ❌ | ✅ | ❌ |
//! | Interior Mut.    | N/A | N/A | N/A |
//! | `and_then` API   | `self` | `&self` | `&self` |
//! | Lock Overhead    | None | None | None |
//!
//! # Use Cases
//!
//! ## Common Scenarios
//!
//! - **Atomic operations**: Increment counter and return new value
//! - **Cache updates**: Update cache and return old value
//! - **Validation**: Validate and fix data, return validation result
//! - **Event handlers**: Process event and return whether to continue
//! - **State machines**: Transition state and return transition info
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use prism3_function::{BoxMutatingFunction, MutatingFunction};
//!
//! // Increment counter and return new value
//! let incrementer = BoxMutatingFunction::new(|x: &mut i32| {
//!     *x += 1;
//!     *x
//! });
//!
//! let mut value = 5;
//! let result = incrementer.apply(&mut value);
//! assert_eq!(value, 6);
//! assert_eq!(result, 6);
//! ```
//!
//! ## Method Chaining
//!
//! ```rust
//! use prism3_function::{BoxMutatingFunction, MutatingFunction};
//!
//! let chained = BoxMutatingFunction::new(|x: &mut i32| {
//!     *x *= 2;
//!     *x
//! })
//! .and_then(|x: &mut i32| {
//!     *x += 10;
//!     *x
//! });
//!
//! let mut value = 5;
//! let result = chained.apply(&mut value);
//! assert_eq!(value, 20); // (5 * 2) + 10
//! assert_eq!(result, 20);
//! ```
//!
//! ## Cache Update Pattern
//!
//! ```rust
//! use prism3_function::{BoxMutatingFunction, MutatingFunction};
//! use std::collections::HashMap;
//!
//! let updater = BoxMutatingFunction::new(
//!     |cache: &mut HashMap<String, i32>| {
//!         cache.insert("key".to_string(), 42)
//!     }
//! );
//!
//! let mut cache = HashMap::new();
//! cache.insert("key".to_string(), 10);
//! let old_value = updater.apply(&mut cache);
//! assert_eq!(old_value, Some(10));
//! assert_eq!(cache.get("key"), Some(&42));
//! ```
//!
//! # Author
//!
//! Haixing Hu
use std::rc::Rc;
use std::sync::Arc;

use crate::{
    functions::{
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
    },
    macros::{
        impl_common_name_methods,
        impl_common_new_methods,
    },
    predicates::predicate::{
        ArcPredicate,
        BoxPredicate,
        Predicate,
        RcPredicate,
    },
};

// =======================================================================
// 1. MutatingFunction Trait - Unified Interface
// =======================================================================

/// MutatingFunction trait - Unified mutating function interface
///
/// Defines the core behavior of all mutating function types. Performs
/// operations that accept a mutable reference, potentially modify it, and
/// return a result.
///
/// This trait is automatically implemented by:
/// - All closures implementing `Fn(&mut T) -> R`
/// - `BoxMutatingFunction<T, R>`, `ArcMutatingFunction<T, R>`, and
///   `RcMutatingFunction<T, R>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models
/// for operations that both modify input and return results. This is useful
/// for scenarios where you need to:
/// - Update state and return information about the update
/// - Perform atomic-like operations (modify and return)
/// - Implement event handlers that modify state and signal continuation
///
/// # Features
///
/// - **Unified Interface**: All mutating function types share the same
///   `apply` method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait
/// - **Type Conversions**: Easy conversion between ownership models
/// - **Generic Programming**: Write functions that work with any mutating
///   function type
///
/// # Examples
///
/// ## Generic Function
///
/// ```rust
/// use prism3_function::{MutatingFunction, BoxMutatingFunction};
///
/// fn apply_and_log<F: MutatingFunction<i32, i32>>(
///     func: &F,
///     value: i32
/// ) -> i32 {
///     let mut val = value;
///     let result = func.apply(&mut val);
///     println!("Modified: {} -> {}, returned: {}", value, val, result);
///     result
/// }
///
/// let incrementer = BoxMutatingFunction::new(|x: &mut i32| {
///     *x += 1;
///     *x
/// });
/// assert_eq!(apply_and_log(&incrementer, 5), 6);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use prism3_function::MutatingFunction;
///
/// let closure = |x: &mut i32| {
///     *x *= 2;
///     *x
/// };
///
/// // Convert to different ownership models
/// let box_func = closure.into_box();
/// // let rc_func = closure.into_rc();  // closure moved
/// // let arc_func = closure.into_arc(); // closure moved
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait MutatingFunction<T, R> {
    /// Applies the function to the mutable reference and returns a result
    ///
    /// Executes an operation on the given mutable reference, potentially
    /// modifying it, and returns a result value.
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
    /// use prism3_function::{MutatingFunction, BoxMutatingFunction};
    ///
    /// let func = BoxMutatingFunction::new(|x: &mut i32| {
    ///     let old = *x;
    ///     *x += 1;
    ///     old
    /// });
    ///
    /// let mut value = 5;
    /// let old_value = func.apply(&mut value);
    /// assert_eq!(old_value, 5);
    /// assert_eq!(value, 6);
    /// ```
    fn apply(&self, t: &mut T) -> R;

    /// Convert this mutating function into a `BoxMutatingFunction<T, R>`.
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
    /// A `BoxMutatingFunction<T, R>` that forwards to the original function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::MutatingFunction;
    ///
    /// let closure = |x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// };
    /// let mut boxed = closure.into_box();
    /// let mut value = 5;
    /// assert_eq!(boxed.apply(&mut value), 10);
    /// ```
    fn into_box(self) -> BoxMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxMutatingFunction::new(move |t| self.apply(t))
    }

    /// Convert this mutating function into an `RcMutatingFunction<T, R>`.
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
    /// An `RcMutatingFunction<T, R>` forwarding to the original function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::MutatingFunction;
    ///
    /// let closure = |x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// };
    /// let mut rc = closure.into_rc();
    /// let mut value = 5;
    /// assert_eq!(rc.apply(&mut value), 10);
    /// ```
    fn into_rc(self) -> RcMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        RcMutatingFunction::new(move |t| self.apply(t))
    }

    /// Convert this mutating function into an `ArcMutatingFunction<T, R>`.
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
    /// An `ArcMutatingFunction<T, R>` that forwards to the original
    /// function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::MutatingFunction;
    ///
    /// let closure = |x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// };
    /// let mut arc = closure.into_arc();
    /// let mut value = 5;
    /// assert_eq!(arc.apply(&mut value), 10);
    /// ```
    fn into_arc(self) -> ArcMutatingFunction<T, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        ArcMutatingFunction::new(move |t| self.apply(t))
    }

    /// Consume the function and return an `Fn(&mut T) -> R` closure.
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
    /// A closure implementing `Fn(&mut T) -> R` which forwards to the
    /// original function.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{MutatingFunction, BoxMutatingFunction};
    ///
    /// let func = BoxMutatingFunction::new(|x: &mut i32| {
    ///     *x *= 2;
    ///     *x
    /// });
    /// let closure = func.into_fn();
    /// let mut value = 5;
    /// assert_eq!(closure(&mut value), 10);
    /// ```
    fn into_fn(self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        move |t| self.apply(t)
    }

    /// Create a non-consuming `BoxMutatingFunction<T, R>` that forwards to
    /// `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed function that calls the cloned instance. Override this
    /// method if a more efficient conversion exists.
    ///
    /// # Returns
    ///
    /// A `BoxMutatingFunction<T, R>` that forwards to a clone of `self`.
    fn to_box(&self) -> BoxMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_box()
    }

    /// Create a non-consuming `RcMutatingFunction<T, R>` that forwards to
    /// `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns an `Rc`-backed function that forwards calls to the clone.
    /// Override to provide a more direct or efficient conversion if needed.
    ///
    /// # Returns
    ///
    /// An `RcMutatingFunction<T, R>` that forwards to a clone of `self`.
    fn to_rc(&self) -> RcMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_rc()
    }

    /// Create a non-consuming `ArcMutatingFunction<T, R>` that forwards to
    /// `self`.
    ///
    /// The default implementation clones `self` (requires
    /// `Clone + Send + Sync`) and returns an `Arc`-wrapped function that
    /// forwards calls to the clone. Override when a more efficient conversion
    /// is available.
    ///
    /// # Returns
    ///
    /// An `ArcMutatingFunction<T, R>` that forwards to a clone of `self`.
    fn to_arc(&self) -> ArcMutatingFunction<T, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Create a boxed `Fn(&mut T) -> R` closure that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed closure that invokes the cloned instance. Override to
    /// provide a more efficient conversion when possible.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn(&mut T) -> R` which forwards to the
    /// original function.
    fn to_fn(&self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone().into_fn()
    }
}

// =======================================================================
// 3. BoxMutatingFunction - Single Ownership Implementation
// =======================================================================

/// BoxMutatingFunction struct
///
/// A mutating function implementation based on `Box<dyn Fn(&mut T) -> R>`
/// for single ownership scenarios. This is the simplest and most efficient
/// mutating function type when sharing is not required.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Zero Overhead**: No reference counting or locking
/// - **Stateless**: Cannot modify captured environment (uses `Fn` not
///   `FnMut`)
/// - **Builder Pattern**: Method chaining consumes `self` naturally
/// - **Factory Methods**: Convenient constructors for common patterns
///
/// # Use Cases
///
/// Choose `BoxMutatingFunction` when:
/// - The function is used for stateless operations
/// - Building pipelines where ownership naturally flows
/// - No need to share the function across contexts
/// - Performance is critical and no sharing overhead is acceptable
///
/// # Performance
///
/// `BoxMutatingFunction` has the best performance among the three function
/// types:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use prism3_function::{MutatingFunction, BoxMutatingFunction};
///
/// let func = BoxMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let mut value = 5;
/// assert_eq!(func.apply(&mut value), 10);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxMutatingFunction<T, R> {
    function: Box<dyn Fn(&mut T) -> R>,
    name: Option<String>,
}

impl<T, R> BoxMutatingFunction<T, R>
where
    T: 'static,
    R: 'static,
{
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        BoxMutatingFunction<T, R>,
        (Fn(&mut T) -> R + 'static),
        |f| Box::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_box_function_methods!(
        BoxMutatingFunction<T, R>,
        BoxConditionalMutatingFunction,
        Function  // chains a non-mutating function after this mutating function
    );
}

// Generates: Debug and Display implementations for BoxMutatingFunction<T, R>
impl_function_debug_display!(BoxMutatingFunction<T, R>);

// Generates: identity() method for BoxMutatingFunction<T, T>
impl_function_identity_method!(BoxMutatingFunction<T, T>, mutating);

// Implement MutatingFunction trait for BoxMutatingFunction<T, R>
impl<T, R> MutatingFunction<T, R> for BoxMutatingFunction<T, R> {
    fn apply(&self, t: &mut T) -> R {
        (self.function)(t)
    }

    fn into_box(self) -> BoxMutatingFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self
    }

    fn into_rc(self) -> RcMutatingFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        RcMutatingFunction::new(move |t| (self.function)(t))
    }

    // do NOT override MutatingFunction::into_arc() because
    // BoxMutatingFunction is not Send + Sync and calling
    // BoxMutatingFunction::into_arc() will cause a compile error

    fn into_fn(self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        move |t| (self.function)(t)
    }

    // do NOT override MutatingFunction::to_xxx() because
    // BoxMutatingFunction is not Clone and calling
    // BoxMutatingFunction::to_xxx() will cause a compile error
}

// =======================================================================
// 4. RcMutatingFunction - Single-Threaded Shared Ownership
// =======================================================================

/// RcMutatingFunction struct
///
/// A mutating function implementation based on `Rc<dyn Fn(&mut T) -> R>` for
/// single-threaded shared ownership scenarios. This type allows multiple
/// references to the same function without the overhead of thread safety.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Stateless**: Cannot modify captured environment (uses `Fn` not
///   `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
/// - **Performance**: More efficient than `ArcMutatingFunction` (no locking)
///
/// # Use Cases
///
/// Choose `RcMutatingFunction` when:
/// - The function needs to be shared within a single thread for stateless
///   operations
/// - Thread safety is not required
/// - Performance is important (avoiding lock overhead)
///
/// # Examples
///
/// ```rust
/// use prism3_function::{MutatingFunction, RcMutatingFunction};
///
/// let func = RcMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let clone = func.clone();
///
/// let mut value = 5;
/// assert_eq!(func.apply(&mut value), 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcMutatingFunction<T, R> {
    function: Rc<dyn Fn(&mut T) -> R>,
    name: Option<String>,
}

impl<T, R> RcMutatingFunction<T, R>
where
    T: 'static,
    R: 'static,
{
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        RcMutatingFunction<T, R>,
        (Fn(&mut T) -> R + 'static),
        |f| Rc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        RcMutatingFunction<T, R>,
        RcConditionalMutatingFunction,
        into_rc,
        Function,  // chains a non-mutating function after this mutating function
        'static
    );
}

// Generates: Clone implementation for RcMutatingFunction<T, R>
impl_function_clone!(RcMutatingFunction<T, R>);

// Generates: Debug and Display implementations for RcMutatingFunction<T, R>
impl_function_debug_display!(RcMutatingFunction<T, R>);

// Generates: identity() method for RcMutatingFunction<T, T>
impl_function_identity_method!(RcMutatingFunction<T, T>, mutating);

impl<T, R> MutatingFunction<T, R> for RcMutatingFunction<T, R> {
    fn apply(&self, input: &mut T) -> R {
        (self.function)(input)
    }

    fn into_box(self) -> BoxMutatingFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        BoxMutatingFunction::new(move |t| (self.function)(t))
    }

    fn into_rc(self) -> RcMutatingFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        self
    }

    // do NOT override MutatingFunction::into_arc() because
    // RcMutatingFunction is not Send + Sync and calling
    // RcMutatingFunction::into_arc() will cause a compile error

    fn into_fn(self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        move |t| (self.function)(t)
    }

    fn to_box(&self) -> BoxMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        BoxMutatingFunction::new(move |t| (self_fn)(t))
    }

    fn to_rc(&self) -> RcMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone()
    }

    // do NOT override MutatingFunction::to_arc() because
    // RcMutatingFunction is not Send + Sync and calling
    // RcMutatingFunction::to_arc() will cause a compile error

    fn to_fn(&self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        move |t| (self_fn)(t)
    }
}

// =======================================================================
// 5. ArcMutatingFunction - Thread-Safe Shared Ownership
// =======================================================================

/// ArcMutatingFunction struct
///
/// A mutating function implementation based on
/// `Arc<dyn Fn(&mut T) -> R + Send + Sync>` for thread-safe shared ownership
/// scenarios. This type allows the function to be safely shared and used
/// across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Stateless**: Cannot modify captured environment (uses `Fn` not
///   `FnMut`)
/// - **Chainable**: Method chaining via `&self` (non-consuming)
///
/// # Use Cases
///
/// Choose `ArcMutatingFunction` when:
/// - The function needs to be shared across multiple threads for stateless
///   operations
/// - Concurrent task processing (e.g., thread pools)
/// - Thread safety is required (Send + Sync)
///
/// # Examples
///
/// ```rust
/// use prism3_function::{MutatingFunction, ArcMutatingFunction};
///
/// let func = ArcMutatingFunction::new(|x: &mut i32| {
///     *x *= 2;
///     *x
/// });
/// let clone = func.clone();
///
/// let mut value = 5;
/// assert_eq!(func.apply(&mut value), 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcMutatingFunction<T, R> {
    function: Arc<dyn Fn(&mut T) -> R + Send + Sync>,
    name: Option<String>,
}

impl<T, R> ArcMutatingFunction<T, R>
where
    T: 'static,
    R: 'static,
{
    // Generates: new(), new_with_name(), new_with_optional_name(), name(), set_name()
    impl_function_common_methods!(
        ArcMutatingFunction<T, R>,
        (Fn(&mut T) -> R + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: when(), and_then(), compose()
    impl_shared_function_methods!(
        ArcMutatingFunction<T, R>,
        ArcConditionalMutatingFunction,
        into_arc,
        Function,  // chains a non-mutating function after this mutating function
        Send + Sync + 'static
    );
}

// Generates: Clone implementation for ArcMutatingFunction<T, R>
impl_function_clone!(ArcMutatingFunction<T, R>);

// Generates: Debug and Display implementations for ArcMutatingFunction<T, R>
impl_function_debug_display!(ArcMutatingFunction<T, R>);

// Generates: identity() method for ArcMutatingFunction<T, T>
impl_function_identity_method!(ArcMutatingFunction<T, T>, mutating);

impl<T, R> MutatingFunction<T, R> for ArcMutatingFunction<T, R> {
    fn apply(&self, input: &mut T) -> R {
        (self.function)(input)
    }

    fn into_box(self) -> BoxMutatingFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        BoxMutatingFunction::new(move |t| (self.function)(t))
    }

    fn into_rc(self) -> RcMutatingFunction<T, R>
    where
        T: 'static,
        R: 'static,
    {
        RcMutatingFunction::new(move |t| (self.function)(t))
    }

    fn into_arc(self) -> ArcMutatingFunction<T, R>
    where
        T: Send + 'static,
        R: Send + 'static,
    {
        self
    }

    fn into_fn(self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        move |t| (self.function)(t)
    }

    fn to_box(&self) -> BoxMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        BoxMutatingFunction::new(move |t| (self_fn)(t))
    }

    fn to_rc(&self) -> RcMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        RcMutatingFunction::new(move |t| (self_fn)(t))
    }

    fn to_arc(&self) -> ArcMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        self.clone()
    }

    fn to_fn(&self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        let self_fn = self.function.clone();
        move |t| (self_fn)(t)
    }
}

// =======================================================================
// 6. Implement MutatingFunction trait for closures
// =======================================================================

impl<T, R, F> MutatingFunction<T, R> for F
where
    F: Fn(&mut T) -> R,
{
    fn apply(&self, input: &mut T) -> R {
        self(input)
    }

    fn into_box(self) -> BoxMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        BoxMutatingFunction::new(self)
    }

    fn into_rc(self) -> RcMutatingFunction<T, R>
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        RcMutatingFunction::new(self)
    }

    fn into_arc(self) -> ArcMutatingFunction<T, R>
    where
        Self: Sized + Send + Sync + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        ArcMutatingFunction::new(self)
    }

    fn into_fn(self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + 'static,
        T: 'static,
        R: 'static,
    {
        self
    }

    fn to_box(&self) -> BoxMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        let cloned = self.clone();
        BoxMutatingFunction::new(cloned)
    }

    fn to_rc(&self) -> RcMutatingFunction<T, R>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        let cloned = self.clone();
        RcMutatingFunction::new(cloned)
    }

    fn to_arc(&self) -> ArcMutatingFunction<T, R>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        T: Send + 'static,
        R: Send + 'static,
    {
        let cloned = self.clone();
        ArcMutatingFunction::new(cloned)
    }

    fn to_fn(&self) -> impl Fn(&mut T) -> R
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        R: 'static,
    {
        self.clone()
    }
}

// =======================================================================
// 7. Provide extension methods for closures
// =======================================================================

// Generates: FnFunctionOps trait and blanket implementation
impl_fn_ops_trait!(
    (Fn(&mut T) -> R),
    FnMutatingFunctionOps,
    BoxMutatingFunction,
    Function, // chains a non-mutating function after this mutating function
    BoxConditionalMutatingFunction
);

// ============================================================================
// BoxConditionalMutatingFunction - Box-based Conditional Mutating Function
// ============================================================================

/// BoxConditionalMutatingFunction struct
///
/// A conditional function that only executes when a predicate is satisfied.
/// Uses `BoxMutatingFunction` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxMutatingFunction::when()` and is
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
/// use prism3_function::{MutatingFunction, BoxMutatingFunction};
///
/// let double = BoxMutatingFunction::new(|x: &mut i32| x * 2);
/// let negate = BoxMutatingFunction::new(|x: &mut i32| -x);
/// let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
///
/// assert_eq!(conditional.apply(5), 10); // when branch executed
/// assert_eq!(conditional.apply(-5), 5); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalMutatingFunction<T, R> {
    function: BoxMutatingFunction<T, R>,
    predicate: BoxPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_box_conditional_function!(
    BoxConditionalMutatingFunction<T, R>,
    BoxMutatingFunction,
    MutatingFunction
);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(BoxConditionalMutatingFunction<T, R>);

// ============================================================================
// RcConditionalMutatingFunction - Rc-based Conditional Mutating Function
// ============================================================================

/// RcConditionalMutatingFunction struct
///
/// A single-threaded conditional function that only executes when a
/// predicate is satisfied. Uses `RcMutatingFunction` and `RcPredicate` for shared
/// ownership within a single thread.
///
/// This type is typically created by calling `RcMutatingFunction::when()` and is
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
/// use prism3_function::{MutatingFunction, RcMutatingFunction};
///
/// let double = RcMutatingFunction::new(|x: &mut i32| x * 2);
/// let identity = RcMutatingFunction::<i32, i32>::identity();
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
pub struct RcConditionalMutatingFunction<T, R> {
    function: RcMutatingFunction<T, R>,
    predicate: RcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    RcConditionalMutatingFunction<T, R>,
    RcMutatingFunction,
    MutatingFunction,
    'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(RcConditionalMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(RcConditionalMutatingFunction<T, R>);

// ============================================================================
// ArcConditionalMutatingFunction - Arc-based Conditional Mutating Function
// ============================================================================

/// ArcConditionalMutatingFunction struct
///
/// A thread-safe conditional function that only executes when a predicate is
/// satisfied. Uses `ArcMutatingFunction` and `ArcPredicate` for shared ownership
/// across threads.
///
/// This type is typically created by calling `ArcMutatingFunction::when()` and is
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
/// use prism3_function::{MutatingFunction, ArcMutatingFunction};
///
/// let double = ArcMutatingFunction::new(|x: &mut i32| x * 2);
/// let identity = ArcMutatingFunction::<i32, i32>::identity();
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
pub struct ArcConditionalMutatingFunction<T, R> {
    function: ArcMutatingFunction<T, R>,
    predicate: ArcPredicate<T>,
}

// Use macro to generate conditional function implementations
impl_shared_conditional_function!(
    ArcConditionalMutatingFunction<T, R>,
    ArcMutatingFunction,
    MutatingFunction,
    Send + Sync + 'static
);

// Use macro to generate conditional function clone implementations
impl_conditional_function_clone!(ArcConditionalMutatingFunction<T, R>);

// Use macro to generate conditional function debug and display implementations
impl_conditional_function_debug_display!(ArcConditionalMutatingFunction<T, R>);
