/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Mutator Types
//!
//! Provides Java-like `Mutator` interface implementations for performing
//! operations that accept a single mutable input parameter and return no result.
//!
//! This module provides a unified `Mutator` trait and three concrete
//! implementations based on different ownership models:
//!
//! - **`BoxMutator<T>`**: Box-based single ownership implementation for
//!   one-time use scenarios and builder patterns
//! - **`ArcMutator<T>`**: Arc<Mutex<>>-based thread-safe shared ownership
//!   implementation for multi-threaded scenarios
//! - **`RcMutator<T>`**: Rc<RefCell<>>-based single-threaded shared
//!   ownership implementation with no lock overhead
//!
//! It is similar to the `FnMut(&mut T)` trait in the standard library.
//!
//! # Design Philosophy
//!
//! Unlike `Consumer` which observes values without modifying them (`FnMut(&T)`),
//! `Mutator` is designed to **modify input values** using `FnMut(&mut T)`.
//!
//! ## Mutator vs Consumer
//!
//! | Type | Input | Modifies Input? | Modifies Self? | Use Cases |
//! |------|-------|----------------|----------------|-----------|
//! | **Consumer** | `&T` | ❌ | ✅ | Observe, log, count, notify |
//! | **Mutator** | `&mut T` | ✅ | ✅ | Modify, transform, update |
//!
//! **Key Insight**: If you need to modify input values, use `Mutator`.
//! If you only need to observe or accumulate state, use `Consumer`.
//!
//! # Comparison Table
//!
//! | Feature          | BoxMutator | ArcMutator | RcMutator |
//! |------------------|------------|------------|-----------|
//! | Ownership        | Single     | Shared     | Shared    |
//! | Cloneable        | ❌         | ✅         | ✅        |
//! | Thread-Safe      | ❌         | ✅         | ❌        |
//! | Interior Mut.    | N/A        | Mutex      | RefCell   |
//! | `and_then` API   | `self`     | `&self`    | `&self`   |
//! | Lock Overhead    | None       | Yes        | None      |
//!
//! # Use Cases
//!
//! ## BoxMutator
//!
//! - One-time operations that don't require sharing
//! - Builder patterns where ownership naturally flows
//! - Simple scenarios with no reuse requirements
//!
//! ## ArcMutator
//!
//! - Multi-threaded shared operations
//! - Concurrent task processing (e.g., thread pools)
//! - Situations requiring the same mutator across threads
//!
//! ## RcMutator
//!
//! - Single-threaded operations with multiple uses
//! - Event handling in single-threaded UI frameworks
//! - Performance-critical single-threaded scenarios
//!
//! # Examples
//!
//! ## Basic Usage
//!
//! ```rust
//! use prism3_function::{BoxMutator, ArcMutator, RcMutator, Mutator};
//!
//! // BoxMutator: Single ownership, consumes self
//! let mut mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
//! let mut value = 5;
//! mutator.apply(&mut value);
//! assert_eq!(value, 10);
//!
//! // ArcMutator: Shared ownership, cloneable, thread-safe
//! let shared = ArcMutator::new(|x: &mut i32| *x *= 2);
//! let clone = shared.clone();
//! let mut value = 5;
//! let mut m = shared;
//! m.apply(&mut value);
//! assert_eq!(value, 10);
//!
//! // RcMutator: Shared ownership, cloneable, single-threaded
//! let rc = RcMutator::new(|x: &mut i32| *x *= 2);
//! let clone = rc.clone();
//! let mut value = 5;
//! let mut m = rc;
//! m.apply(&mut value);
//! assert_eq!(value, 10);
//! ```
//!
//! ## Method Chaining
//!
//! ```rust
//! use prism3_function::{Mutator, BoxMutator, ArcMutator};
//!
//! // BoxMutator: Consumes self
//! let mut chained = BoxMutator::new(|x: &mut i32| *x *= 2)
//!     .and_then(|x: &mut i32| *x += 10);
//! let mut value = 5;
//! chained.apply(&mut value);
//! assert_eq!(value, 20); // (5 * 2) + 10
//!
//! // ArcMutator: Borrows &self, original still usable
//! let first = ArcMutator::new(|x: &mut i32| *x *= 2);
//! let second = ArcMutator::new(|x: &mut i32| *x += 10);
//! let combined = first.and_then(&second);
//! // first and second are still usable here
//! ```
//!
//! ## Working with Closures
//!
//! All closures automatically implement the `Mutator` trait:
//!
//! ```rust
//! use prism3_function::{Mutator, FnMutatorOps};
//!
//! // Closures can use .apply() directly
//! let mut closure = |x: &mut i32| *x *= 2;
//! let mut value = 5;
//! closure.apply(&mut value);
//! assert_eq!(value, 10);
//!
//! // Closures can be chained, returning BoxMutator
//! let mut chained = (|x: &mut i32| *x *= 2)
//!     .and_then(|x: &mut i32| *x += 10);
//! let mut value = 5;
//! chained.apply(&mut value);
//! assert_eq!(value, 20);
//! ```
//!
//! ## Type Conversions
//!
//! ```rust
//! use prism3_function::Mutator;
//!
//! // Convert closure to concrete type
//! let closure = |x: &mut i32| *x *= 2;
//! let mut box_mutator = closure.into_box();
//!
//! let closure = |x: &mut i32| *x *= 2;
//! let mut rc_mutator = closure.into_rc();
//!
//! let closure = |x: &mut i32| *x *= 2;
//! let mut arc_mutator = closure.into_arc();
//! ```
//!
//! ## Conditional Execution
//!
//! All mutator types support conditional execution through the `when` method,
//! which returns a `ConditionalMutator`. You can optionally add an `or_else`
//! branch to create if-then-else logic:
//!
//! ```rust
//! use prism3_function::{Mutator, BoxMutator};
//!
//! // Simple conditional (if-then)
//! let mut conditional = BoxMutator::new(|x: &mut i32| *x *= 2)
//!     .when(|x: &i32| *x > 0);
//!
//! let mut positive = 5;
//! conditional.apply(&mut positive);
//! assert_eq!(positive, 10); // Executed
//!
//! let mut negative = -5;
//! conditional.apply(&mut negative);
//! assert_eq!(negative, -5); // Not executed
//!
//! // Conditional with else branch (if-then-else)
//! let mut branched = BoxMutator::new(|x: &mut i32| *x *= 2)
//!     .when(|x: &i32| *x > 0)
//!     .or_else(|x: &mut i32| *x -= 1);
//!
//! let mut positive = 5;
//! branched.apply(&mut positive);
//! assert_eq!(positive, 10); // when branch
//!
//! let mut negative = -5;
//! branched.apply(&mut negative);
//! assert_eq!(negative, -6); // or_else branch
//! ```
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

use crate::macros::{
    impl_arc_conversions,
    impl_box_conversions,
    impl_rc_conversions,
};
use crate::mutators::macros::{
    impl_box_conditional_mutator,
    impl_box_mutator_methods,
    impl_conditional_mutator_clone,
    impl_conditional_mutator_conversions,
    impl_conditional_mutator_debug_display,
    impl_mutator_clone,
    impl_mutator_common_methods,
    impl_mutator_debug_display,
    impl_shared_conditional_mutator,
    impl_shared_mutator_methods,
};
use crate::mutators::mutator_once::BoxMutatorOnce;
use crate::predicates::predicate::{
    ArcPredicate,
    BoxPredicate,
    Predicate,
    RcPredicate,
};

// ============================================================================
// 1. Type Aliases
// ============================================================================

/// Type alias for Arc-wrapped mutable mutator function
type ArcMutMutatorFn<T> = Arc<Mutex<dyn FnMut(&mut T) + Send>>;

/// Type alias for Rc-wrapped mutable mutator function
type RcMutMutatorFn<T> = Rc<RefCell<dyn FnMut(&mut T)>>;

// ============================================================================
// 2. Mutator Trait - Unified Mutator Interface
// ============================================================================

/// Mutator trait - Unified mutator interface
///
/// Defines the core behavior of all mutator types. Performs operations that
/// accept a mutable reference and modify the input value (not just side effects).
///
/// This trait is automatically implemented by:
/// - All closures implementing `FnMut(&mut T)`
/// - `BoxMutator<T>`, `ArcMutator<T>`, and `RcMutator<T>`
///
/// # Design Rationale
///
/// The trait provides a unified abstraction over different ownership models,
/// allowing generic code to work with any mutator type. Type conversion
/// methods (`into_box`, `into_arc`, `into_rc`) enable flexible ownership
/// transitions based on usage requirements.
///
/// # Features
///
/// - **Unified Interface**: All mutator types share the same `mutate`
///   method signature
/// - **Automatic Implementation**: Closures automatically implement this
///   trait with zero overhead
/// - **Type Conversions**: Easy conversion between ownership models
/// - **Generic Programming**: Write functions that work with any mutator
///   type
///
/// # Examples
///
/// ## Generic Mutator Function
///
/// ```rust
/// use prism3_function::{Mutator, BoxMutator, ArcMutator};
///
/// fn apply_mutator<M: Mutator<i32>>(
///     mutator: &mut M,
///     value: i32
/// ) -> i32 {
///     let mut val = value;
///     mutator.apply(&mut val);
///     val
/// }
///
/// // Works with any mutator type
/// let mut box_mut = BoxMutator::new(|x: &mut i32| *x *= 2);
/// assert_eq!(apply_mutator(&mut box_mut, 5), 10);
///
/// let mut arc_mut = ArcMutator::new(|x: &mut i32| *x *= 2);
/// assert_eq!(apply_mutator(&mut arc_mut, 5), 10);
///
/// let mut closure = |x: &mut i32| *x *= 2;
/// assert_eq!(apply_mutator(&mut closure, 5), 10);
/// ```
///
/// ## Type Conversion
///
/// ```rust
/// use prism3_function::Mutator;
///
/// let closure = |x: &mut i32| *x *= 2;
///
/// // Convert to different ownership models
/// let box_mutator = closure.into_box();
/// // let rc_mutator = closure.into_rc();  // closure moved
/// // let arc_mutator = closure.into_arc(); // closure moved
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait StatefulMutator<T> {
    /// Performs the mutation operation
    ///
    /// Executes an operation on the given mutable reference. The operation
    /// typically modifies the input value or produces side effects.
    ///
    /// # Parameters
    ///
    /// * `value` - A mutable reference to the value to be mutated
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{Mutator, BoxMutator};
    ///
    /// let mut mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
    /// let mut value = 5;
    /// mutator.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn apply(&mut self, value: &mut T);

    /// Convert this mutator into a `BoxMutator<T>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns a
    /// boxed implementation that forwards calls to the original mutator.
    /// Types that can provide a more efficient conversion may override the
    /// default implementation.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator: the original value will no longer
    /// be available after the call. For cloneable mutators call `.clone()`
    /// before converting if you need to retain the original instance.
    ///
    /// # Returns
    ///
    /// A `BoxMutator<T>` that forwards to the original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::Mutator;
    ///
    /// let closure = |x: &mut i32| *x *= 2;
    /// let mut boxed = closure.into_box();
    /// let mut value = 5;
    /// boxed.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn into_box(mut self) -> BoxStatefulMutator<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        BoxStatefulMutator::new(move |t| self.apply(t))
    }

    /// Convert this mutator into an `RcMutator<T>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns an
    /// `Rc`-backed mutator that forwards calls to the original. Override to
    /// provide a more direct or efficient conversion when available.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator. If you need to keep the original
    /// instance, clone it prior to calling this method.
    ///
    /// # Returns
    ///
    /// An `RcMutator<T>` forwarding to the original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::Mutator;
    ///
    /// let closure = |x: &mut i32| *x *= 2;
    /// let mut rc = closure.into_rc();
    /// let mut value = 5;
    /// rc.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn into_rc(mut self) -> RcStatefulMutator<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        RcStatefulMutator::new(move |t| self.apply(t))
    }

    /// Convert this mutator into an `ArcMutator<T>`.
    ///
    /// This consuming conversion takes ownership of `self` and returns an
    /// `Arc`-wrapped, thread-safe mutator. Types may override the default
    /// implementation to provide a more efficient conversion.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator. Clone the instance first if you
    /// need to retain the original for further use.
    ///
    /// # Returns
    ///
    /// An `ArcMutator<T>` that forwards to the original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::Mutator;
    ///
    /// let closure = |x: &mut i32| *x *= 2;
    /// let mut arc = closure.into_arc();
    /// let mut value = 5;
    /// arc.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn into_arc(mut self) -> ArcStatefulMutator<T>
    where
        Self: Sized + Send + 'static,
        T: Send + 'static,
    {
        ArcStatefulMutator::new(move |t| self.apply(t))
    }

    /// Consume the mutator and return an `FnMut(&mut T)` closure.
    ///
    /// The returned closure forwards calls to the original mutator and is
    /// suitable for use with iterator adapters such as `for_each`.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator. The original instance will not be
    /// available after calling this method.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T)` which forwards to the
    /// original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{Mutator, BoxMutator};
    ///
    /// let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
    /// let mut values = vec![1, 2, 3, 4, 5];
    /// values.iter_mut().for_each(mutator.into_fn());
    /// assert_eq!(values, vec![2, 4, 6, 8, 10]);
    /// ```
    fn into_fn(mut self) -> impl FnMut(&mut T)
    where
        Self: Sized + 'static,
        T: 'static,
    {
        move |t| self.apply(t)
    }

    /// Create a non-consuming `BoxMutator<T>` that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed mutator that calls the cloned instance. Override this
    /// method if a more efficient conversion exists.
    ///
    /// # Returns
    ///
    /// A `BoxMutator<T>` that forwards to a clone of `self`.
    fn to_box(&self) -> BoxStatefulMutator<T>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
    {
        self.clone().into_box()
    }

    /// Create a non-consuming `RcMutator<T>` that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns an `Rc`-backed mutator that forwards calls to the clone.
    /// Override to provide a more direct or efficient conversion if needed.
    ///
    /// # Returns
    ///
    /// An `RcMutator<T>` that forwards to a clone of `self`.
    fn to_rc(&self) -> RcStatefulMutator<T>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
    {
        self.clone().into_rc()
    }

    /// Create a non-consuming `ArcMutator<T>` that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone + Send`) and
    /// returns an `Arc`-wrapped mutator that forwards calls to the clone.
    /// Override when a more efficient conversion is available.
    ///
    /// # Returns
    ///
    /// An `ArcMutator<T>` that forwards to a clone of `self`.
    fn to_arc(&self) -> ArcStatefulMutator<T>
    where
        Self: Sized + Clone + Send + 'static,
        T: Send + 'static,
    {
        self.clone().into_arc()
    }

    /// Create a boxed `FnMut(&mut T)` closure that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed closure that invokes the cloned instance. Override to
    /// provide a more efficient conversion when possible.
    ///
    /// # Returns
    ///
    /// A closure implementing `FnMut(&mut T)` which forwards to the
    /// original mutator.
    fn to_fn(&self) -> impl FnMut(&mut T)
    where
        Self: Sized + Clone + 'static,
        T: 'static,
    {
        self.clone().into_fn()
    }

    /// Convert this mutator into a `BoxMutatorOnce<T>` (consuming).
    ///
    /// This consuming conversion takes ownership of `self` and returns a
    /// boxed one-time mutator that forwards calls to the original mutator.
    /// The returned mutator can only be used once.
    ///
    /// # Consumption
    ///
    /// This method consumes the mutator: the original value will no longer
    /// be available after the call. For cloneable mutators call `.clone()`
    /// before converting if you need to retain the original instance.
    ///
    /// # Returns
    ///
    /// A `BoxMutatorOnce<T>` that forwards to the original mutator.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{StatefulMutator, BoxStatefulMutator,
    ///                       BoxMutatorOnce};
    ///
    /// let mutator = BoxStatefulMutator::new(|x: &mut i32| *x *= 2);
    /// let once_mutator = mutator.into_once();
    /// let mut value = 5;
    /// once_mutator.apply(&mut value);
    /// assert_eq!(value, 10);
    /// ```
    fn into_once(mut self) -> BoxMutatorOnce<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        BoxMutatorOnce::new(move |t| self.apply(t))
    }

    /// Create a non-consuming `BoxMutatorOnce<T>` that forwards to `self`.
    ///
    /// The default implementation clones `self` (requires `Clone`) and
    /// returns a boxed one-time mutator that calls the cloned instance.
    /// Override this method if a more efficient conversion exists.
    ///
    /// # Returns
    ///
    /// A `BoxMutatorOnce<T>` that forwards to a clone of `self`.
    fn to_once(&self) -> BoxMutatorOnce<T>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
    {
        self.clone().into_once()
    }
}

// ============================================================================
// 3. BoxMutator - Single Ownership Implementation
// ============================================================================

/// BoxMutator struct
///
/// A mutator implementation based on `Box<dyn FnMut(&mut T)>` for single
/// ownership scenarios. This is the simplest and most efficient mutator
/// type when sharing is not required.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, ownership moves on use
/// - **Zero Overhead**: No reference counting or locking
/// - **Mutable State**: Can modify captured environment via `FnMut`
/// - **Builder Pattern**: Method chaining consumes `self` naturally
/// - **Factory Methods**: Convenient constructors for common patterns
///
/// # Use Cases
///
/// Choose `BoxMutator` when:
/// - The mutator is used only once or in a linear flow
/// - Building pipelines where ownership naturally flows
/// - No need to share the mutator across contexts
/// - Performance is critical and no sharing overhead is acceptable
///
/// # Performance
///
/// `BoxMutator` has the best performance among the three mutator types:
/// - No reference counting overhead
/// - No lock acquisition or runtime borrow checking
/// - Direct function call through vtable
/// - Minimal memory footprint (single pointer)
///
/// # Examples
///
/// ```rust
/// use prism3_function::{Mutator, BoxMutator};
///
/// let mut mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
/// let mut value = 5;
/// mutator.apply(&mut value);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxStatefulMutator<T> {
    function: Box<dyn FnMut(&mut T)>,
    name: Option<String>,
}

impl<T> BoxStatefulMutator<T>
where
    T: 'static,
{
    impl_mutator_common_methods!(BoxStatefulMutator<T>, (FnMut(&mut T) + 'static), |f| {
        Box::new(f)
    });

    // Generate box mutator methods (when, and_then, or_else, etc.)
    impl_box_mutator_methods!(
        BoxStatefulMutator<T>,
        BoxConditionalStatefulMutator,
        StatefulMutator
    );
}

impl<T> StatefulMutator<T> for BoxStatefulMutator<T> {
    fn apply(&mut self, value: &mut T) {
        (self.function)(value)
    }

    // Generates: into_box(), into_rc(), into_fn(), into_once()
    impl_box_conversions!(
        BoxStatefulMutator<T>,
        RcStatefulMutator,
        FnMut(&mut T),
        BoxMutatorOnce
    );
}

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(BoxStatefulMutator<T>);

// ============================================================================
// 3. RcMutator - Single-Threaded Shared Ownership Implementation
// ============================================================================

/// RcMutator struct
///
/// A mutator implementation based on `Rc<RefCell<dyn FnMut(&mut T)>>` for
/// single-threaded shared ownership scenarios. This type allows multiple
/// references to the same mutator without the overhead of thread safety.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Interior Mutability**: Uses `RefCell` for runtime borrow checking
/// - **Mutable State**: Can modify captured environment via `FnMut`
/// - **Chainable**: Method chaining via `&self` (non-consuming)
/// - **Performance**: More efficient than `ArcMutator` (no locking)
///
/// # Use Cases
///
/// Choose `RcMutator` when:
/// - The mutator needs to be shared within a single thread
/// - Thread safety is not required
/// - Performance is important (avoiding lock overhead)
///
/// # Examples
///
/// ```rust
/// use prism3_function::{Mutator, RcMutator};
///
/// let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
/// let clone = mutator.clone();
///
/// let mut value = 5;
/// let mut m = mutator;
/// m.mutate(&mut value);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcStatefulMutator<T> {
    function: RcMutMutatorFn<T>,
    name: Option<String>,
}

impl<T> RcStatefulMutator<T>
where
    T: 'static,
{
    impl_mutator_common_methods!(
        RcStatefulMutator<T>,
        (FnMut(&mut T) + 'static),
        |f| Rc::new(RefCell::new(f))
    );

    // Generate shared mutator methods (when, and_then, or_else, conversions)
    impl_shared_mutator_methods!(
        RcStatefulMutator<T>,
        RcConditionalStatefulMutator,
        into_rc,
        StatefulMutator,
        'static
    );
}

impl<T> StatefulMutator<T> for RcStatefulMutator<T> {
    fn apply(&mut self, value: &mut T) {
        (self.function.borrow_mut())(value)
    }

    // Generate all conversion methods using the unified macro
    impl_rc_conversions!(
        RcStatefulMutator<T>,
        BoxStatefulMutator,
        BoxMutatorOnce,
        FnMut(t: &mut T)
    );
}

// Use macro to generate Clone implementation
impl_mutator_clone!(RcStatefulMutator<T>);

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(RcStatefulMutator<T>);

// ============================================================================
// 4. ArcMutator - Thread-Safe Shared Ownership Implementation
// ============================================================================

/// ArcMutator struct
///
/// A mutator implementation based on `Arc<Mutex<dyn FnMut(&mut T) + Send>>`
/// for thread-safe shared ownership scenarios. This type allows the mutator
/// to be safely shared and used across multiple threads.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Interior Mutability**: Uses `Mutex` for safe concurrent mutations
/// - **Mutable State**: Can modify captured environment via `FnMut`
/// - **Chainable**: Method chaining via `&self` (non-consuming)
///
/// # Use Cases
///
/// Choose `ArcMutator` when:
/// - The mutator needs to be shared across multiple threads
/// - Concurrent task processing (e.g., thread pools)
/// - Thread safety is required (Send + Sync)
///
/// # Examples
///
/// ```rust
/// use prism3_function::{Mutator, ArcMutator};
///
/// let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
/// let clone = mutator.clone();
///
/// let mut value = 5;
/// let mut m = mutator;
/// m.mutate(&mut value);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcStatefulMutator<T> {
    function: ArcMutMutatorFn<T>,
    name: Option<String>,
}

impl<T> ArcStatefulMutator<T>
where
    T: 'static,
{
    impl_mutator_common_methods!(
        ArcStatefulMutator<T>,
        (FnMut(&mut T) + Send + 'static),
        |f| Arc::new(Mutex::new(f))
    );

    // Generate shared mutator methods (when, and_then, or_else, conversions)
    impl_shared_mutator_methods!(
        ArcStatefulMutator<T>,
        ArcConditionalStatefulMutator,
        into_arc,
        StatefulMutator,
        Send + Sync + 'static
    );
}

impl<T> StatefulMutator<T> for ArcStatefulMutator<T> {
    fn apply(&mut self, value: &mut T) {
        (self.function.lock().unwrap())(value)
    }

    // Use macro to implement conversion methods
    impl_arc_conversions!(
        ArcStatefulMutator<T>,
        BoxStatefulMutator,
        RcStatefulMutator,
        BoxMutatorOnce,
        FnMut(input: &mut T)
    );
}

// Use macro to generate Clone implementation
impl_mutator_clone!(ArcStatefulMutator<T>);

// Generate Debug and Display trait implementations
impl_mutator_debug_display!(ArcStatefulMutator<T>);

// ============================================================================
// 5. Implement Mutator trait for closures
// ============================================================================

impl<T, F> StatefulMutator<T> for F
where
    F: FnMut(&mut T),
{
    fn apply(&mut self, value: &mut T) {
        self(value)
    }

    fn into_box(mut self) -> BoxStatefulMutator<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        BoxStatefulMutator::new(move |t| self.apply(t))
    }

    fn into_rc(mut self) -> RcStatefulMutator<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        RcStatefulMutator::new(move |t| self.apply(t))
    }

    fn into_arc(mut self) -> ArcStatefulMutator<T>
    where
        Self: Sized + Send + 'static,
        T: Send + 'static,
    {
        ArcStatefulMutator::new(move |t| self.apply(t))
    }

    fn into_fn(self) -> impl FnMut(&mut T)
    where
        Self: Sized + 'static,
        T: 'static,
    {
        self
    }

    fn to_box(&self) -> BoxStatefulMutator<T>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
    {
        let mut cloned = self.clone();
        BoxStatefulMutator::new(move |t| cloned.apply(t))
    }

    fn to_rc(&self) -> RcStatefulMutator<T>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
    {
        let mut cloned = self.clone();
        RcStatefulMutator::new(move |t| cloned.apply(t))
    }

    fn to_arc(&self) -> ArcStatefulMutator<T>
    where
        Self: Sized + Clone + Send + 'static,
        T: Send + 'static,
    {
        let mut cloned = self.clone();
        ArcStatefulMutator::new(move |t| cloned.apply(t))
    }

    fn to_fn(&self) -> impl FnMut(&mut T)
    where
        Self: Sized + Clone + 'static,
        T: 'static,
    {
        self.clone()
    }
}

// ============================================================================
// 6. Provide extension methods for closures
// ============================================================================

/// Extension trait providing mutator composition methods for closures
///
/// Provides `and_then` and other composition methods for all closures that
/// implement `FnMut(&mut T)`, enabling direct method chaining on closures
/// without explicit wrapper types.
///
/// # Features
///
/// - **Natural Syntax**: Chain operations directly on closures
/// - **Returns BoxMutator**: Composition results are `BoxMutator<T>` for
///   continued chaining
/// - **Zero Cost**: No overhead when composing closures
/// - **Automatic Implementation**: All `FnMut(&mut T)` closures get these
///   methods automatically
///
/// # Examples
///
/// ```rust
/// use prism3_function::{Mutator, FnMutatorOps};
///
/// let chained = (|x: &mut i32| *x *= 2)
///     .and_then(|x: &mut i32| *x += 10);
/// let mut value = 5;
/// let mut result = chained;
/// result.mutate(&mut value);
/// assert_eq!(value, 20); // (5 * 2) + 10
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnMutStatefulMutatorOps<T>: FnMut(&mut T) + Sized {
    /// Chains another mutator in sequence
    ///
    /// Returns a new mutator that first executes the current operation, then
    /// executes the next operation. Consumes the current closure and returns
    /// `BoxMutator<T>`.
    ///
    /// # Parameters
    ///
    /// * `next` - The mutator to execute after the current operation. **Note:
    ///   This parameter is passed by value and will transfer ownership.** If you
    ///   need to preserve the original mutator, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - A closure: `|x: &mut T|`
    ///   - A `BoxMutator<T>`
    ///   - An `ArcMutator<T>`
    ///   - An `RcMutator<T>`
    ///   - Any type implementing `Mutator<T>`
    ///
    /// # Returns
    ///
    /// Returns the composed `BoxMutator<T>`
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::{Mutator, FnMutatorOps};
    ///
    /// let chained = (|x: &mut i32| *x *= 2)
    ///     .and_then(|x: &mut i32| *x += 10)
    ///     .and_then(|x: &mut i32| println!("Result: {}", x));
    ///
    /// let mut value = 5;
    /// let mut result = chained;
    /// result.mutate(&mut value); // Prints: Result: 20
    /// assert_eq!(value, 20);
    /// ```
    fn and_then<C>(self, next: C) -> BoxStatefulMutator<T>
    where
        Self: 'static,
        C: StatefulMutator<T> + 'static,
        T: 'static,
    {
        let mut first = self;
        let mut second = next.into_fn();
        BoxStatefulMutator::new(move |t| {
            (first)(t);
            second(t);
        })
    }
}

/// Implements FnMutatorOps for all closure types
impl<T, F> FnMutStatefulMutatorOps<T> for F where F: FnMut(&mut T) {}

// ============================================================================
// 7. BoxConditionalMutator - Box-based Conditional Mutator
// ============================================================================

/// BoxConditionalMutator struct
///
/// A conditional mutator that only executes when a predicate is satisfied.
/// Uses `BoxMutator` and `BoxPredicate` for single ownership semantics.
///
/// This type is typically created by calling `BoxMutator::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Single Ownership**: Not cloneable, consumes `self` on use
/// - **Conditional Execution**: Only mutates when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
/// - **Implements Mutator**: Can be used anywhere a `Mutator` is expected
///
/// # Examples
///
/// ## Basic Conditional Execution
///
/// ```rust
/// use prism3_function::{Mutator, BoxMutator};
///
/// let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
/// let mut conditional = mutator.when(|x: &i32| *x > 0);
///
/// let mut positive = 5;
/// conditional.apply(&mut positive);
/// assert_eq!(positive, 10); // Executed
///
/// let mut negative = -5;
/// conditional.apply(&mut negative);
/// assert_eq!(negative, -5); // Not executed
/// ```
///
/// ## With or_else Branch
///
/// ```rust
/// use prism3_function::{Mutator, BoxMutator};
///
/// let mut mutator = BoxMutator::new(|x: &mut i32| *x *= 2)
///     .when(|x: &i32| *x > 0)
///     .or_else(|x: &mut i32| *x -= 1);
///
/// let mut positive = 5;
/// mutator.apply(&mut positive);
/// assert_eq!(positive, 10); // when branch executed
///
/// let mut negative = -5;
/// mutator.apply(&mut negative);
/// assert_eq!(negative, -6); // or_else branch executed
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxConditionalStatefulMutator<T> {
    mutator: BoxStatefulMutator<T>,
    predicate: BoxPredicate<T>,
}

// Generate box conditional mutator methods (and_then, or_else)
impl_box_conditional_mutator!(
    BoxConditionalStatefulMutator<T>,
    BoxStatefulMutator,
    StatefulMutator
);

impl<T> StatefulMutator<T> for BoxConditionalStatefulMutator<T>
where
    T: 'static,
{
    fn apply(&mut self, value: &mut T) {
        if self.predicate.test(value) {
            self.mutator.apply(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_mutator_conversions!(BoxStatefulMutator<T>, RcStatefulMutator, FnMut);
}

// Generate Debug and Display trait implementations for conditional mutator
impl_conditional_mutator_debug_display!(BoxConditionalStatefulMutator<T>);

// ============================================================================
// 8. RcConditionalMutator - Rc-based Conditional Mutator
// ============================================================================

/// RcConditionalMutator struct
///
/// A single-threaded conditional mutator that only executes when a predicate is
/// satisfied. Uses `RcMutator` and `RcPredicate` for shared ownership within a
/// single thread.
///
/// This type is typically created by calling `RcMutator::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Rc`, multiple owners allowed
/// - **Single-Threaded**: Not thread-safe, cannot be sent across threads
/// - **Conditional Execution**: Only mutates when predicate returns `true`
/// - **No Lock Overhead**: More efficient than `ArcConditionalMutator`
///
/// # Examples
///
/// ```rust
/// use prism3_function::{Mutator, RcMutator};
///
/// let conditional = RcMutator::new(|x: &mut i32| *x *= 2)
///     .when(|x: &i32| *x > 0);
///
/// let conditional_clone = conditional.clone();
///
/// let mut value = 5;
/// let mut m = conditional;
/// m.mutate(&mut value);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcConditionalStatefulMutator<T> {
    mutator: RcStatefulMutator<T>,
    predicate: RcPredicate<T>,
}

// Generate shared conditional mutator methods (and_then, or_else, conversions)
impl_shared_conditional_mutator!(
    RcConditionalStatefulMutator<T>,
    RcStatefulMutator,
    StatefulMutator,
    into_rc,
    'static
);

impl<T> StatefulMutator<T> for RcConditionalStatefulMutator<T>
where
    T: 'static,
{
    fn apply(&mut self, value: &mut T) {
        if self.predicate.test(value) {
            self.mutator.apply(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_mutator_conversions!(BoxStatefulMutator<T>, RcStatefulMutator, FnMut);
}

// Generate Clone trait implementation for conditional mutator
impl_conditional_mutator_clone!(RcConditionalStatefulMutator<T>);

// Generate Debug and Display trait implementations for conditional mutator
impl_conditional_mutator_debug_display!(RcConditionalStatefulMutator<T>);

// ============================================================================
// 9. ArcConditionalMutator - Arc-based Conditional Mutator
// ============================================================================

/// ArcConditionalMutator struct
///
/// A thread-safe conditional mutator that only executes when a predicate is
/// satisfied. Uses `ArcMutator` and `ArcPredicate` for shared ownership across
/// threads.
///
/// This type is typically created by calling `ArcMutator::when()` and is
/// designed to work with the `or_else()` method to create if-then-else logic.
///
/// # Features
///
/// - **Shared Ownership**: Cloneable via `Arc`, multiple owners allowed
/// - **Thread-Safe**: Implements `Send + Sync`, safe for concurrent use
/// - **Conditional Execution**: Only mutates when predicate returns `true`
/// - **Chainable**: Can add `or_else` branch to create if-then-else logic
///
/// # Examples
///
/// ```rust
/// use prism3_function::{Mutator, ArcMutator};
///
/// let conditional = ArcMutator::new(|x: &mut i32| *x *= 2)
///     .when(|x: &i32| *x > 0);
///
/// let conditional_clone = conditional.clone();
///
/// let mut value = 5;
/// let mut m = conditional;
/// m.mutate(&mut value);
/// assert_eq!(value, 10);
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcConditionalStatefulMutator<T> {
    mutator: ArcStatefulMutator<T>,
    predicate: ArcPredicate<T>,
}

// Generate shared conditional mutator methods (and_then, or_else, conversions)
impl_shared_conditional_mutator!(
    ArcConditionalStatefulMutator<T>,
    ArcStatefulMutator,
    StatefulMutator,
    into_arc,
    Send + Sync + 'static
);

impl<T> StatefulMutator<T> for ArcConditionalStatefulMutator<T>
where
    T: Send + 'static,
{
    fn apply(&mut self, value: &mut T) {
        if self.predicate.test(value) {
            self.mutator.apply(value);
        }
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_conditional_mutator_conversions!(BoxStatefulMutator<T>, RcStatefulMutator, FnMut);
}

// Generate Clone trait implementation for conditional mutator
impl_conditional_mutator_clone!(ArcConditionalStatefulMutator<T>);

// Generate Debug and Display trait implementations for conditional mutator
impl_conditional_mutator_debug_display!(ArcConditionalStatefulMutator<T>);
