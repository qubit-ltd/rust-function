/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Predicate Abstraction
//!
//! Provides a Rust implementation similar to Java's `Predicate` interface
//! for condition testing and logical composition.
//!
//! ## Core Semantics
//!
//! A **Predicate** is fundamentally a pure judgment operation that tests
//! whether a value satisfies a specific condition. It should be:
//!
//! - **Read-only**: Does not modify the tested value
//! - **Side-effect free**: Does not change external state (from the user's
//!   perspective)
//! - **Repeatable**: Same input should produce the same result
//! - **Deterministic**: Judgment logic should be predictable
//!
//! It is similar to the `Fn(&T) -> bool` trait in the standard library.
//!
//! ## Design Philosophy
//!
//! This module follows these principles:
//!
//! 1. **Single Trait**: Only one `Predicate<T>` trait with `&self`, keeping
//!    the API simple and semantically clear
//! 2. **No PredicateMut**: All stateful scenarios use interior mutability
//!    (`RefCell`, `Cell`, `Mutex`) instead of `&mut self`
//! 3. **No PredicateOnce**: Violates predicate semantics - judgments should
//!    be repeatable
//! 4. **Three Implementations**: `BoxPredicate`, `RcPredicate`, and
//!    `ArcPredicate` cover all ownership scenarios
//!
//! ## Type Selection Guide
//!
//! | Scenario | Recommended Type | Reason |
//! |----------|------------------|--------|
//! | One-time use | `BoxPredicate` | Single ownership, no overhead |
//! | Multi-threaded | `ArcPredicate` | Thread-safe, clonable |
//! | Single-threaded reuse | `RcPredicate` | Better performance |
//! | Stateful predicate | Any type + `RefCell`/`Cell`/`Mutex` | Interior mutability |
//!
//! ## Examples
//!
//! ### Basic Usage with Closures
//!
//! ```rust
//! use prism3_function::predicate::Predicate;
//!
//! let is_positive = |x: &i32| *x > 0;
//! assert!(is_positive.test(&5));
//! assert!(!is_positive.test(&-3));
//! ```
//!
//! ### BoxPredicate - Single Ownership
//!
//! ```rust
//! use prism3_function::predicate::{Predicate, BoxPredicate};
//!
//! let pred = BoxPredicate::new(|x: &i32| *x > 0)
//!     .and(BoxPredicate::new(|x| x % 2 == 0));
//! assert!(pred.test(&4));
//! ```
//!
//! ### Closure Composition with Extension Methods
//!
//! Closures automatically gain `and`, `or`, `not` methods through the
//! `FnPredicateOps` extension trait, returning `BoxPredicate`:
//!
//! ```rust
//! use prism3_function::predicate::{Predicate, FnPredicateOps, BoxPredicate};
//!
//! // Compose closures directly - result is BoxPredicate
//! let is_positive = |x: &i32| *x > 0;
//! let is_even = |x: &i32| x % 2 == 0;
//!
//! let positive_and_even = is_positive.and(is_even);
//! assert!(positive_and_even.test(&4));
//! assert!(!positive_and_even.test(&3));
//!
//! // Can chain multiple operations
//! let pred = (|x: &i32| *x > 0)
//!     .and(|x: &i32| x % 2 == 0)
//!     .and(BoxPredicate::new(|x: &i32| *x < 100));
//! assert!(pred.test(&42));
//!
//! // Use `or` for disjunction
//! let negative_or_large = (|x: &i32| *x < 0)
//!     .or(|x: &i32| *x > 100);
//! assert!(negative_or_large.test(&-5));
//! assert!(negative_or_large.test(&200));
//!
//! // Use `not` for negation
//! let not_zero = (|x: &i32| *x == 0).not();
//! assert!(not_zero.test(&5));
//! assert!(!not_zero.test(&0));
//! ```
//!
//! ### Complex Predicate Composition
//!
//! Build complex predicates by mixing closures and predicate types:
//!
//! ```rust
//! use prism3_function::predicate::{Predicate, BoxPredicate, FnPredicateOps};
//!
//! // Start with a closure, compose with BoxPredicate
//! let in_range = (|x: &i32| *x >= 0)
//!     .and(BoxPredicate::new(|x| *x <= 100));
//!
//! // Use in filtering
//! let numbers = vec![-10, 5, 50, 150, 75];
//! let filtered: Vec<_> = numbers.iter()
//!     .copied()
//!     .filter(in_range.into_fn())
//!     .collect();
//! assert_eq!(filtered, vec![5, 50, 75]);
//! ```
//!
//! ### RcPredicate - Single-threaded Reuse
//!
//! ```rust
//! use prism3_function::predicate::{Predicate, RcPredicate};
//!
//! let pred = RcPredicate::new(|x: &i32| *x > 0);
//! let combined1 = pred.and(RcPredicate::new(|x| x % 2 == 0));
//! let combined2 = pred.or(RcPredicate::new(|x| *x > 100));
//!
//! // Original predicate is still usable
//! assert!(pred.test(&5));
//! ```
//!
//! ### ArcPredicate - Thread-safe Sharing
//!
//! ```rust
//! use prism3_function::predicate::{Predicate, ArcPredicate};
//! use std::thread;
//!
//! let pred = ArcPredicate::new(|x: &i32| *x > 0);
//! let pred_clone = pred.clone();
//!
//! let handle = thread::spawn(move || {
//!     pred_clone.test(&10)
//! });
//!
//! assert!(handle.join().unwrap());
//! assert!(pred.test(&5));  // Original still usable
//! ```
//!
//! ### Stateful Predicates with Interior Mutability
//!
//! ```rust
//! use prism3_function::predicate::{Predicate, BoxPredicate};
//! use std::cell::Cell;
//!
//! let count = Cell::new(0);
//! let pred = BoxPredicate::new(move |x: &i32| {
//!     count.set(count.get() + 1);
//!     *x > 0
//! });
//!
//! // No need for `mut` - interior mutability handles state
//! assert!(pred.test(&5));
//! assert!(!pred.test(&-3));
//! ```
//!
//! ## Author
//!
//! Haixing Hu
use std::rc::Rc;
use std::sync::Arc;

use crate::macros::impl_box_into_conversions;
use crate::predicates::macros::{
    constants::{
        ALWAYS_FALSE_NAME,
        ALWAYS_TRUE_NAME,
    },
    impl_box_predicate_methods,
    impl_predicate_clone,
    impl_predicate_common_methods,
    impl_predicate_debug_display,
    impl_shared_predicate_methods,
};

/// A predicate trait for testing whether a value satisfies a condition.
///
/// This trait represents a **pure judgment operation** - it tests whether
/// a given value meets certain criteria without modifying either the value
/// or the predicate itself (from the user's perspective). This semantic
/// clarity distinguishes predicates from consumers or transformers.
///
/// ## Design Rationale
///
/// This is a **minimal trait** that only defines:
/// - The core `test` method using `&self` (immutable borrow)
/// - Type conversion methods (`into_box`, `into_rc`, `into_arc`)
/// - Closure conversion method (`into_fn`)
///
/// Logical composition methods (`and`, `or`, `not`) are intentionally
/// **not** part of the trait. Instead, they are implemented on concrete
/// types (`BoxPredicate`, `RcPredicate`, `ArcPredicate`), allowing each
/// implementation to maintain its specific ownership characteristics:
///
/// - `BoxPredicate`: Methods consume `self` (single ownership)
/// - `RcPredicate`: Methods borrow `&self` (shared ownership)
/// - `ArcPredicate`: Methods borrow `&self` (thread-safe shared ownership)
///
/// ## Why `&self` Instead of `&mut self`?
///
/// Predicates use `&self` because:
///
/// 1. **Semantic Clarity**: A predicate is a judgment, not a mutation
/// 2. **Flexibility**: Can be used in immutable contexts
/// 3. **Simplicity**: No need for `mut` in user code
/// 4. **Interior Mutability**: State (if needed) can be managed with
///    `RefCell`, `Cell`, or `Mutex`
///
/// ## Automatic Implementation for Closures
///
/// Any closure matching `Fn(&T) -> bool` automatically implements this
/// trait, providing seamless integration with Rust's closure system.
///
/// ## Examples
///
/// ### Basic Usage
///
/// ```rust
/// use prism3_function::predicate::Predicate;
///
/// let is_positive = |x: &i32| *x > 0;
/// assert!(is_positive.test(&5));
/// assert!(!is_positive.test(&-3));
/// ```
///
/// ### Type Conversion
///
/// ```rust
/// use prism3_function::predicate::{Predicate, BoxPredicate};
///
/// let closure = |x: &i32| *x > 0;
/// let boxed: BoxPredicate<i32> = closure.into_box();
/// assert!(boxed.test(&5));
/// ```
///
/// ### Stateful Predicate with Interior Mutability
///
/// ```rust
/// use prism3_function::predicate::{Predicate, BoxPredicate};
/// use std::cell::Cell;
///
/// let count = Cell::new(0);
/// let counting_pred = BoxPredicate::new(move |x: &i32| {
///     count.set(count.get() + 1);
///     *x > 0
/// });
///
/// // Note: No `mut` needed - interior mutability handles state
/// assert!(counting_pred.test(&5));
/// assert!(!counting_pred.test(&-3));
/// ```
///
/// ## Author
///
/// Haixing Hu
pub trait Predicate<T> {
    /// Tests whether the given value satisfies this predicate.
    ///
    /// # Parameters
    ///
    /// * `value` - The value to test.
    ///
    /// # Returns
    ///
    /// `true` if the value satisfies this predicate, `false` otherwise.
    fn test(&self, value: &T) -> bool;

    /// Converts this predicate into a `BoxPredicate`.
    ///
    /// The default implementation wraps the predicate in a closure that
    /// calls the `test` method. Concrete types may override this with
    /// more efficient implementations.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` wrapping this predicate.
    fn into_box(self) -> BoxPredicate<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| self.test(value))
    }

    /// Converts this predicate into an `RcPredicate`.
    ///
    /// The default implementation wraps the predicate in a closure that
    /// calls the `test` method. Concrete types may override this with
    /// more efficient implementations.
    ///
    /// # Returns
    ///
    /// An `RcPredicate` wrapping this predicate.
    fn into_rc(self) -> RcPredicate<T>
    where
        Self: Sized + 'static,
        T: 'static,
    {
        RcPredicate::new(move |value: &T| self.test(value))
    }

    /// Converts this predicate into an `ArcPredicate`.
    ///
    /// The default implementation wraps the predicate in a closure that
    /// calls the `test` method. Concrete types may override this with
    /// more efficient implementations.
    ///
    /// # Returns
    ///
    /// An `ArcPredicate` wrapping this predicate.
    fn into_arc(self) -> ArcPredicate<T>
    where
        Self: Sized + Send + Sync + 'static,
        T: 'static,
    {
        ArcPredicate::new(move |value: &T| self.test(value))
    }

    /// Converts this predicate into a closure that can be used directly
    /// with standard library methods.
    ///
    /// This method consumes the predicate and returns a closure with
    /// signature `Fn(&T) -> bool`. Since `Fn` is a subtrait of `FnMut`,
    /// the returned closure can be used in any context that requires
    /// either `Fn(&T) -> bool` or `FnMut(&T) -> bool`, making it
    /// compatible with methods like `Iterator::filter`,
    /// `Iterator::filter_map`, `Vec::retain`, and similar standard
    /// library APIs.
    ///
    /// The default implementation returns a closure that calls the
    /// `test` method. Concrete types may override this with more
    /// efficient implementations.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn(&T) -> bool` (also usable as
    /// `FnMut(&T) -> bool`).
    ///
    /// # Examples
    ///
    /// ## Using with `Iterator::filter` (requires `FnMut`)
    ///
    /// ```rust
    /// use prism3_function::predicate::{Predicate, BoxPredicate};
    ///
    /// let pred = BoxPredicate::new(|x: &i32| *x > 0);
    ///
    /// let numbers = vec![-2, -1, 0, 1, 2, 3];
    /// let positives: Vec<_> = numbers.iter()
    ///     .copied()
    ///     .filter(pred.into_fn())
    ///     .collect();
    /// assert_eq!(positives, vec![1, 2, 3]);
    /// ```
    ///
    /// ## Using with `Vec::retain` (requires `FnMut`)
    ///
    /// ```rust
    /// use prism3_function::predicate::{Predicate, BoxPredicate};
    ///
    /// let pred = BoxPredicate::new(|x: &i32| *x % 2 == 0);
    /// let mut numbers = vec![1, 2, 3, 4, 5, 6];
    /// numbers.retain(pred.into_fn());
    /// assert_eq!(numbers, vec![2, 4, 6]);
    /// ```
    fn into_fn(self) -> impl Fn(&T) -> bool
    where
        Self: Sized + 'static,
        T: 'static,
    {
        move |value: &T| self.test(value)
    }

    /// Converts a reference to this predicate into a `BoxPredicate`.
    ///
    /// This method clones the predicate and then converts it to a
    /// `BoxPredicate`. The original predicate remains usable after this call.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` wrapping a clone of this predicate.
    fn to_box(&self) -> BoxPredicate<T>
    where
        Self: Clone + Sized + 'static,
        T: 'static,
    {
        self.clone().into_box()
    }

    /// Converts a reference to this predicate into an `RcPredicate`.
    ///
    /// This method clones the predicate and then converts it to an
    /// `RcPredicate`. The original predicate remains usable after this call.
    ///
    /// # Returns
    ///
    /// An `RcPredicate` wrapping a clone of this predicate.
    fn to_rc(&self) -> RcPredicate<T>
    where
        Self: Clone + Sized + 'static,
        T: 'static,
    {
        self.clone().into_rc()
    }

    /// Converts a reference to this predicate into an `ArcPredicate`.
    ///
    /// This method clones the predicate and then converts it to an
    /// `ArcPredicate`. The original predicate remains usable after this call.
    ///
    /// # Returns
    ///
    /// An `ArcPredicate` wrapping a clone of this predicate.
    fn to_arc(&self) -> ArcPredicate<T>
    where
        Self: Clone + Sized + Send + Sync + 'static,
        T: 'static,
    {
        self.clone().into_arc()
    }

    /// Converts a reference to this predicate into a closure that can be
    /// used directly with standard library methods.
    ///
    /// This method clones the predicate and then converts it to a closure.
    /// The original predicate remains usable after this call.
    ///
    /// The returned closure has signature `Fn(&T) -> bool`. Since `Fn` is a
    /// subtrait of `FnMut`, it can be used in any context that requires
    /// either `Fn(&T) -> bool` or `FnMut(&T) -> bool`, making it compatible
    /// with methods like `Iterator::filter`, `Iterator::filter_map`,
    /// `Vec::retain`, and similar standard library APIs.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn(&T) -> bool` (also usable as
    /// `FnMut(&T) -> bool`).
    fn to_fn(&self) -> impl Fn(&T) -> bool
    where
        Self: Clone + Sized + 'static,
        T: 'static,
    {
        self.clone().into_fn()
    }
}

/// A Box-based predicate with single ownership.
///
/// This type is suitable for one-time use scenarios where the predicate does
/// not need to be cloned or shared. Composition methods consume `self`,
/// reflecting the single-ownership model.
///
/// # Examples
///
/// ```rust
/// use prism3_function::predicate::{Predicate, BoxPredicate};
///
/// let pred = BoxPredicate::new(|x: &i32| *x > 0);
/// assert!(pred.test(&5));
///
/// // Chaining consumes the predicate
/// let combined = pred.and(BoxPredicate::new(|x| x % 2 == 0));
/// assert!(combined.test(&4));
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxPredicate<T> {
    function: Box<dyn Fn(&T) -> bool>,
    name: Option<String>,
}

impl<T: 'static> BoxPredicate<T> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(
        BoxPredicate<T>,
        (Fn(&T) -> bool + 'static),
        |f| Box::new(f)
    );

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_box_predicate_methods!(BoxPredicate<T>);
}

// Generates: impl Debug for BoxPredicate<T> and impl Display for BoxPredicate<T>
impl_predicate_debug_display!(BoxPredicate<T>);

// Implements Predicate trait for BoxPredicate<T>
impl<T: 'static> Predicate<T> for BoxPredicate<T> {
    fn test(&self, value: &T) -> bool {
        (self.function)(value)
    }

    // Generates: into_box(), into_rc(), into_fn()
    impl_box_into_conversions!(
        BoxPredicate<T>,
        RcPredicate,
        impl Fn(&T) -> bool
    );
}

/// An Rc-based predicate with single-threaded shared ownership.
///
/// This type is suitable for scenarios where the predicate needs to be
/// reused in a single-threaded context. Composition methods borrow `&self`,
/// allowing the original predicate to remain usable after composition.
///
/// # Examples
///
/// ```rust
/// use prism3_function::predicate::{Predicate, RcPredicate};
///
/// let pred = RcPredicate::new(|x: &i32| *x > 0);
/// assert!(pred.test(&5));
///
/// // Original predicate remains usable after composition
/// let combined = pred.and(RcPredicate::new(|x| x % 2 == 0));
/// assert!(pred.test(&5));  // Still works
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcPredicate<T> {
    function: Rc<dyn Fn(&T) -> bool>,
    name: Option<String>,
}

impl<T: 'static> RcPredicate<T> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(RcPredicate<T>, (Fn(&T) -> bool + 'static), |f| Rc::new(f));

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_shared_predicate_methods!(RcPredicate<T>, 'static);
}

// Generates: impl Clone for RcPredicate<T>
impl_predicate_clone!(RcPredicate<T>);

// Generates: impl Debug for RcPredicate<T> and impl Display for RcPredicate<T>
impl_predicate_debug_display!(RcPredicate<T>);

// Implements Predicate trait for RcPredicate<T>
impl<T: 'static> Predicate<T> for RcPredicate<T> {
    fn test(&self, value: &T) -> bool {
        (self.function)(value)
    }

    fn into_box(self) -> BoxPredicate<T> {
        let self_fn = self.function;
        BoxPredicate {
            function: Box::new(move |value: &T| self_fn(value)),
            name: self.name,
        }
    }

    fn into_rc(self) -> RcPredicate<T> {
        self
    }

    // do NOT override Predicate::into_arc() because RcPredicate is not Send + Sync
    // and calling RcPredicate::into_arc() will cause a compile error

    fn into_fn(self) -> impl Fn(&T) -> bool {
        let self_fn = self.function;
        move |value: &T| self_fn(value)
    }

    fn to_box(&self) -> BoxPredicate<T> {
        let self_fn = self.function.clone();
        BoxPredicate {
            function: Box::new(move |value: &T| self_fn(value)),
            name: self.name.clone(),
        }
    }

    fn to_rc(&self) -> RcPredicate<T> {
        self.clone()
    }

    // do NOT override Predicate::to_arc() because RcPredicate is not Send + Sync
    // and calling RcPredicate::to_arc() will cause a compile error

    fn to_fn(&self) -> impl Fn(&T) -> bool {
        let self_fn = self.function.clone();
        move |value: &T| self_fn(value)
    }
}

/// An Arc-based predicate with thread-safe shared ownership.
///
/// This type is suitable for scenarios where the predicate needs to be
/// shared across threads. Composition methods borrow `&self`, allowing the
/// original predicate to remain usable after composition.
///
/// # Examples
///
/// ```rust
/// use prism3_function::predicate::{Predicate, ArcPredicate};
///
/// let pred = ArcPredicate::new(|x: &i32| *x > 0);
/// assert!(pred.test(&5));
///
/// // Original predicate remains usable after composition
/// let combined = pred.and(ArcPredicate::new(|x| x % 2 == 0));
/// assert!(pred.test(&5));  // Still works
///
/// // Can be cloned and sent across threads
/// let pred_clone = pred.clone();
/// std::thread::spawn(move || {
///     assert!(pred_clone.test(&10));
/// }).join().unwrap();
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcPredicate<T> {
    function: Arc<dyn Fn(&T) -> bool + Send + Sync>,
    name: Option<String>,
}

impl<T: 'static> ArcPredicate<T> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(
        ArcPredicate<T>,
        (Fn(&T) -> bool + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_shared_predicate_methods!(ArcPredicate<T>, Send + Sync + 'static);
}

// Generates: impl Clone for ArcPredicate<T>
impl_predicate_clone!(ArcPredicate<T>);

// Generates: impl Debug for ArcPredicate<T> and impl Display for ArcPredicate<T>
impl_predicate_debug_display!(ArcPredicate<T>);

// Implements Predicate trait for ArcPredicate<T>
impl<T: 'static> Predicate<T> for ArcPredicate<T> {
    fn test(&self, value: &T) -> bool {
        (self.function)(value)
    }

    fn into_box(self) -> BoxPredicate<T> {
        BoxPredicate {
            function: Box::new(move |value: &T| (self.function)(value)),
            name: self.name,
        }
    }

    fn into_rc(self) -> RcPredicate<T> {
        RcPredicate {
            function: Rc::new(move |value: &T| (self.function)(value)),
            name: self.name,
        }
    }

    fn into_arc(self) -> ArcPredicate<T> {
        self
    }

    fn into_fn(self) -> impl Fn(&T) -> bool {
        move |value: &T| (self.function)(value)
    }

    fn to_box(&self) -> BoxPredicate<T> {
        let self_fn = self.function.clone();
        BoxPredicate {
            function: Box::new(move |value: &T| self_fn(value)),
            name: self.name.clone(),
        }
    }

    fn to_rc(&self) -> RcPredicate<T> {
        let self_fn = self.function.clone();
        RcPredicate {
            function: Rc::new(move |value: &T| self_fn(value)),
            name: self.name.clone(),
        }
    }

    fn to_arc(&self) -> ArcPredicate<T> {
        self.clone()
    }

    fn to_fn(&self) -> impl Fn(&T) -> bool {
        let self_fn = self.function.clone();
        move |value: &T| self_fn(value)
    }
}

// Blanket implementation for all closures that match Fn(&T) -> bool
impl<T: 'static, F> Predicate<T> for F
where
    F: Fn(&T) -> bool + 'static,
{
    fn test(&self, value: &T) -> bool {
        self(value)
    }

    fn into_box(self) -> BoxPredicate<T> {
        BoxPredicate::new(self)
    }

    fn into_rc(self) -> RcPredicate<T> {
        RcPredicate::new(self)
    }

    fn into_arc(self) -> ArcPredicate<T>
    where
        Self: Send + Sync,
    {
        ArcPredicate::new(self)
    }

    fn into_fn(self) -> impl Fn(&T) -> bool {
        self
    }

    fn to_box(&self) -> BoxPredicate<T>
    where
        Self: Clone + 'static,
    {
        let self_fn = self.clone();
        BoxPredicate::new(self_fn)
    }

    fn to_rc(&self) -> RcPredicate<T>
    where
        Self: Clone + 'static,
    {
        let self_fn = self.clone();
        RcPredicate::new(self_fn)
    }

    fn to_arc(&self) -> ArcPredicate<T>
    where
        Self: Clone + Send + Sync + 'static,
        T: 'static,
    {
        let self_fn = self.clone();
        ArcPredicate::new(self_fn)
    }

    fn to_fn(&self) -> impl Fn(&T) -> bool
    where
        Self: Clone + 'static,
    {
        self.clone()
    }
}

/// Extension trait providing logical composition methods for closures.
///
/// This trait is automatically implemented for all closures and function
/// pointers that match `Fn(&T) -> bool`, enabling method chaining starting
/// from a closure.
///
/// # Examples
///
/// ```rust
/// use prism3_function::predicate::{Predicate, FnPredicateOps};
///
/// let is_positive = |x: &i32| *x > 0;
/// let is_even = |x: &i32| x % 2 == 0;
///
/// // Combine predicates using extension methods
/// let pred = is_positive.and(is_even);
/// assert!(pred.test(&4));
/// assert!(!pred.test(&3));
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnPredicateOps<T>: Fn(&T) -> bool + Sized + 'static {
    /// Returns a predicate that represents the logical AND of this predicate
    /// and another.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure
    ///   - A function pointer
    ///   - A `BoxPredicate<T>`, `RcPredicate<T>`, or `ArcPredicate<T>`
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical AND.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::predicate::{Predicate, FnPredicateOps};
    ///
    /// let is_positive = |x: &i32| *x > 0;
    /// let is_even = |x: &i32| x % 2 == 0;
    ///
    /// let combined = is_positive.and(is_even);
    /// assert!(combined.test(&4));
    /// assert!(!combined.test(&3));
    /// ```
    fn and<P>(self, other: P) -> BoxPredicate<T>
    where
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| self.test(value) && other.test(value))
    }

    /// Returns a predicate that represents the logical OR of this predicate
    /// and another.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure
    ///   - A function pointer
    ///   - A `BoxPredicate<T>`, `RcPredicate<T>`, or `ArcPredicate<T>`
    ///   - Any type implementing `Predicate<T>`
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical OR.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::predicate::{Predicate, FnPredicateOps};
    ///
    /// let is_negative = |x: &i32| *x < 0;
    /// let is_large = |x: &i32| *x > 100;
    ///
    /// let combined = is_negative.or(is_large);
    /// assert!(combined.test(&-5));
    /// assert!(combined.test(&150));
    /// assert!(!combined.test(&50));
    /// ```
    fn or<P>(self, other: P) -> BoxPredicate<T>
    where
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| self.test(value) || other.test(value))
    }

    /// Returns a predicate that represents the logical negation of this
    /// predicate.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical negation.
    fn not(self) -> BoxPredicate<T>
    where
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| !self.test(value))
    }

    /// Returns a predicate that represents the logical NAND (NOT AND) of this
    /// predicate and another.
    ///
    /// NAND returns `true` unless both predicates are `true`.
    /// Equivalent to `!(self AND other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Accepts closures, function pointers, or any
    ///   `Predicate<T>` implementation.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical NAND.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::predicate::{Predicate, FnPredicateOps};
    ///
    /// let is_positive = |x: &i32| *x > 0;
    /// let is_even = |x: &i32| x % 2 == 0;
    ///
    /// let nand = is_positive.nand(is_even);
    /// assert!(nand.test(&3));   // !(true && false) = true
    /// assert!(!nand.test(&4));  // !(true && true) = false
    /// ```
    fn nand<P>(self, other: P) -> BoxPredicate<T>
    where
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| !(self.test(value) && other.test(value)))
    }

    /// Returns a predicate that represents the logical XOR (exclusive OR) of
    /// this predicate and another.
    ///
    /// XOR returns `true` if exactly one of the predicates is `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Accepts closures, function pointers, or any
    ///   `Predicate<T>` implementation.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical XOR.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::predicate::{Predicate, FnPredicateOps};
    ///
    /// let is_positive = |x: &i32| *x > 0;
    /// let is_even = |x: &i32| x % 2 == 0;
    ///
    /// let xor = is_positive.xor(is_even);
    /// assert!(xor.test(&3));    // true ^ false = true
    /// assert!(!xor.test(&4));   // true ^ true = false
    /// assert!(!xor.test(&-1));  // false ^ false = false
    /// ```
    fn xor<P>(self, other: P) -> BoxPredicate<T>
    where
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| self.test(value) ^ other.test(value))
    }

    /// Returns a predicate that represents the logical NOR (NOT OR) of this
    /// predicate and another.
    ///
    /// NOR returns `true` only when both predicates are `false`. Equivalent
    /// to `!(self OR other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original predicate, clone it first (if it implements
    ///   `Clone`). Accepts closures, function pointers, or any
    ///   `Predicate<T>` implementation.
    ///
    /// # Returns
    ///
    /// A `BoxPredicate` representing the logical NOR.
    ///
    /// # Examples
    ///
    /// ```rust
    /// use prism3_function::predicate::{Predicate, FnPredicateOps};
    ///
    /// let is_positive = |x: &i32| *x > 0;
    /// let is_even = |x: &i32| x % 2 == 0;
    ///
    /// let nor = is_positive.nor(is_even);
    /// assert!(nor.test(&-3));   // !(false || false) = true
    /// assert!(!nor.test(&4));   // !(true || true) = false
    /// assert!(!nor.test(&3));   // !(true || false) = false
    /// ```
    fn nor<P>(self, other: P) -> BoxPredicate<T>
    where
        P: Predicate<T> + 'static,
        T: 'static,
    {
        BoxPredicate::new(move |value: &T| !(self.test(value) || other.test(value)))
    }
}

// Blanket implementation for all closures
impl<T, F> FnPredicateOps<T> for F where F: Fn(&T) -> bool + 'static {}
