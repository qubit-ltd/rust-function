/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # BiPredicate Abstraction
//!
//! Provides a Rust implementation similar to Java's `BiPredicate`
//! interface for testing whether two values satisfy a condition.
//!
//! ## Core Semantics
//!
//! A **BiPredicate** is fundamentally a pure judgment operation that
//! tests whether two values satisfy a specific condition. It should
//! be:
//!
//! - **Read-only**: Does not modify the tested values
//! - **Side-effect free**: Does not change external state (from the
//!   user's perspective)
//! - **Repeatable**: Same inputs should produce the same result
//! - **Deterministic**: Judgment logic should be predictable
//!
//! It is similar to the `Fn(&T, &U) -> bool` trait in the standard library.
//!
//! ## Design Philosophy
//!
//! This module follows the same principles as the `Predicate` module:
//!
//! 1. **Single Trait**: Only one `BiPredicate<T, U>` trait with
//!    `&self`, keeping the API simple and semantically clear
//! 2. **No BiPredicateMut**: All stateful scenarios use interior
//!    mutability (`RefCell`, `Cell`, `Mutex`) instead of `&mut self`
//! 3. **No BiPredicateOnce**: Violates bi-predicate semantics -
//!    judgments should be repeatable
//! 4. **Three Implementations**: `BoxBiPredicate`, `RcBiPredicate`,
//!    and `ArcBiPredicate` cover all ownership scenarios
//!
//! ## Type Selection Guide
//!
//! | Scenario | Recommended Type | Reason |
//! |----------|------------------|--------|
//! | One-time use | `BoxBiPredicate` | Single ownership, no overhead |
//! | Multi-threaded | `ArcBiPredicate` | Thread-safe, clonable |
//! | Single-threaded reuse | `RcBiPredicate` | Better performance |
//! | Stateful predicate | Any type + `RefCell`/`Cell`/`Mutex` | Interior mutability |
//!
//! ## Examples
//!
//! ### Basic Usage with Closures
//!
//! ```rust
//! use prism3_function::bi_predicate::BiPredicate;
//!
//! let is_sum_positive = |x: &i32, y: &i32| x + y > 0;
//! assert!(is_sum_positive.test(&5, &3));
//! assert!(!is_sum_positive.test(&-3, &-7));
//! ```
//!
//! ### BoxBiPredicate - Single Ownership
//!
//! ```rust
//! use prism3_function::bi_predicate::{BiPredicate, BoxBiPredicate};
//!
//! let pred = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0)
//!     .and(BoxBiPredicate::new(|x, y| x > y));
//! assert!(pred.test(&10, &5));
//! ```
//!
//! ### Closure Composition with Extension Methods
//!
//! Closures automatically gain `and`, `or`, `not` methods through the
//! `FnBiPredicateOps` extension trait, returning `BoxBiPredicate`:
//!
//! ```rust
//! use prism3_function::bi_predicate::{BiPredicate,
//!     FnBiPredicateOps};
//!
//! // Compose closures directly - result is BoxBiPredicate
//! let is_sum_positive = |x: &i32, y: &i32| x + y > 0;
//! let first_larger = |x: &i32, y: &i32| x > y;
//!
//! let combined = is_sum_positive.and(first_larger);
//! assert!(combined.test(&10, &5));
//! assert!(!combined.test(&3, &8));
//!
//! // Use `or` for disjunction
//! let negative_sum = |x: &i32, y: &i32| x + y < 0;
//! let both_large = |x: &i32, y: &i32| *x > 100 && *y > 100;
//! let either = negative_sum.or(both_large);
//! assert!(either.test(&-10, &5));
//! assert!(either.test(&200, &150));
//! ```
//!
//! ### RcBiPredicate - Single-threaded Reuse
//!
//! ```rust
//! use prism3_function::bi_predicate::{BiPredicate, RcBiPredicate};
//!
//! let pred = RcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
//! let combined1 = pred.and(RcBiPredicate::new(|x, y| x > y));
//! let combined2 = pred.or(RcBiPredicate::new(|x, y| *x > 100));
//!
//! // Original predicate is still usable
//! assert!(pred.test(&5, &3));
//! ```
//!
//! ### ArcBiPredicate - Thread-safe Sharing
//!
//! ```rust
//! use prism3_function::bi_predicate::{BiPredicate, ArcBiPredicate};
//! use std::thread;
//!
//! let pred = ArcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
//! let pred_clone = pred.clone();
//!
//! let handle = thread::spawn(move || {
//!     pred_clone.test(&10, &5)
//! });
//!
//! assert!(handle.join().unwrap());
//! assert!(pred.test(&3, &7));  // Original still usable
//! ```
//!
//! ### Stateful BiPredicates with Interior Mutability
//!
//! ```rust
//! use prism3_function::bi_predicate::{BiPredicate, BoxBiPredicate};
//! use std::cell::Cell;
//!
//! let count = Cell::new(0);
//! let pred = BoxBiPredicate::new(move |x: &i32, y: &i32| {
//!     count.set(count.get() + 1);
//!     x + y > 0
//! });
//!
//! // No need for `mut` - interior mutability handles state
//! assert!(pred.test(&5, &3));
//! assert!(!pred.test(&-8, &-3));
//! ```
//!
//! ## Author
//!
//! Haixing Hu

use std::rc::Rc;
use std::sync::Arc;

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

/// Type alias for bi-predicate function to simplify complex types.
///
/// This type alias represents a function that takes two references and returns a boolean.
/// It is used to reduce type complexity in struct definitions.
type BiPredicateFn<T, U> = dyn Fn(&T, &U) -> bool;

/// Type alias for thread-safe bi-predicate function to simplify complex types.
///
/// This type alias represents a function that takes two references and returns a boolean,
/// with Send + Sync bounds for thread-safe usage. It is used to reduce type complexity
/// in Arc-based struct definitions.
type SendSyncBiPredicateFn<T, U> = dyn Fn(&T, &U) -> bool + Send + Sync;

/// A bi-predicate trait for testing whether two values satisfy a
/// condition.
///
/// This trait represents a **pure judgment operation** - it tests
/// whether two given values meet certain criteria without modifying
/// either the values or the bi-predicate itself (from the user's
/// perspective). This semantic clarity distinguishes bi-predicates
/// from consumers or transformers.
///
/// ## Design Rationale
///
/// This is a **minimal trait** that only defines:
/// - The core `test` method using `&self` (immutable borrow)
/// - Type conversion methods (`into_box`, `into_rc`, `into_arc`)
/// - Closure conversion method (`into_fn`)
///
/// Logical composition methods (`and`, `or`, `not`, `xor`, `nand`,
/// `nor`) are intentionally **not** part of the trait. Instead, they
/// are implemented on concrete types (`BoxBiPredicate`,
/// `RcBiPredicate`, `ArcBiPredicate`), allowing each implementation
/// to maintain its specific ownership characteristics:
///
/// - `BoxBiPredicate`: Methods consume `self` (single ownership)
/// - `RcBiPredicate`: Methods borrow `&self` (shared ownership)
/// - `ArcBiPredicate`: Methods borrow `&self` (thread-safe shared
///   ownership)
///
/// ## Why `&self` Instead of `&mut self`?
///
/// Bi-predicates use `&self` because:
///
/// 1. **Semantic Clarity**: A bi-predicate is a judgment, not a
///    mutation
/// 2. **Flexibility**: Can be used in immutable contexts
/// 3. **Simplicity**: No need for `mut` in user code
/// 4. **Interior Mutability**: State (if needed) can be managed with
///    `RefCell`, `Cell`, or `Mutex`
///
/// ## Automatic Implementation for Closures
///
/// Any closure matching `Fn(&T, &U) -> bool` automatically implements
/// this trait, providing seamless integration with Rust's closure
/// system.
///
/// ## Examples
///
/// ### Basic Usage
///
/// ```rust
/// use prism3_function::bi_predicate::BiPredicate;
///
/// let is_sum_positive = |x: &i32, y: &i32| x + y > 0;
/// assert!(is_sum_positive.test(&5, &3));
/// assert!(!is_sum_positive.test(&-5, &-3));
/// ```
///
/// ### Type Conversion
///
/// ```rust
/// use prism3_function::bi_predicate::{BiPredicate,
///     BoxBiPredicate};
///
/// let closure = |x: &i32, y: &i32| x + y > 0;
/// let boxed: BoxBiPredicate<i32, i32> = closure.into_box();
/// assert!(boxed.test(&5, &3));
/// ```
///
/// ### Stateful BiPredicate with Interior Mutability
///
/// ```rust
/// use prism3_function::bi_predicate::{BiPredicate,
///     BoxBiPredicate};
/// use std::cell::Cell;
///
/// let count = Cell::new(0);
/// let counting_pred = BoxBiPredicate::new(move |x: &i32, y: &i32| {
///     count.set(count.get() + 1);
///     x + y > 0
/// });
///
/// // Note: No `mut` needed - interior mutability handles state
/// assert!(counting_pred.test(&5, &3));
/// assert!(!counting_pred.test(&-5, &-3));
/// ```
///
/// ## Author
///
/// Haixing Hu
pub trait BiPredicate<T, U> {
    /// Tests whether the given values satisfy this bi-predicate.
    ///
    /// # Parameters
    ///
    /// * `first` - The first value to test.
    /// * `second` - The second value to test.
    ///
    /// # Returns
    ///
    /// `true` if the values satisfy this bi-predicate, `false`
    /// otherwise.
    fn test(&self, first: &T, second: &U) -> bool;

    /// Converts this bi-predicate into a `BoxBiPredicate`.
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` wrapping this bi-predicate.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps the bi-predicate in a
    /// closure that calls `test`, providing automatic conversion
    /// for custom types that only implement the core `test`
    /// method.
    fn into_box(self) -> BoxBiPredicate<T, U>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| self.test(first, second))
    }

    /// Converts this bi-predicate into an `RcBiPredicate`.
    ///
    /// # Returns
    ///
    /// An `RcBiPredicate` wrapping this bi-predicate.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps the bi-predicate in a
    /// closure that calls `test`, providing automatic conversion
    /// for custom types that only implement the core `test`
    /// method.
    fn into_rc(self) -> RcBiPredicate<T, U>
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
    {
        RcBiPredicate::new(move |first, second| self.test(first, second))
    }

    /// Converts this bi-predicate into an `ArcBiPredicate`.
    ///
    /// # Returns
    ///
    /// An `ArcBiPredicate` wrapping this bi-predicate.
    ///
    /// # Default Implementation
    ///
    /// The default implementation wraps the bi-predicate in a
    /// closure that calls `test`, providing automatic conversion
    /// for custom types that only implement the core `test`
    /// method. Note that this requires `Send + Sync` bounds for
    /// thread-safe sharing.
    fn into_arc(self) -> ArcBiPredicate<T, U>
    where
        Self: Sized + Send + Sync + 'static,
        T: 'static,
        U: 'static,
    {
        ArcBiPredicate::new(move |first, second| self.test(first, second))
    }

    /// Converts this bi-predicate into a closure that can be used
    /// directly with standard library methods.
    ///
    /// This method consumes the bi-predicate and returns a closure
    /// with signature `Fn(&T, &U) -> bool`. Since `Fn` is a subtrait
    /// of `FnMut`, the returned closure can be used in any context
    /// that requires either `Fn(&T, &U) -> bool` or
    /// `FnMut(&T, &U) -> bool`.
    ///
    /// # Returns
    ///
    /// A closure implementing `Fn(&T, &U) -> bool` (also usable as
    /// `FnMut(&T, &U) -> bool`).
    ///
    /// # Default Implementation
    ///
    /// The default implementation returns a closure that calls the
    /// `test` method, providing automatic conversion for custom
    /// types.
    ///
    /// # Examples
    ///
    /// ## Using with Iterator Methods
    ///
    /// ```rust
    /// use prism3_function::bi_predicate::{BiPredicate,
    ///     BoxBiPredicate};
    ///
    /// let pred = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
    ///
    /// let pairs = vec![(1, 2), (-1, 3), (5, -6)];
    /// let mut closure = pred.into_fn();
    /// let positives: Vec<_> = pairs.iter()
    ///     .filter(|(x, y)| closure(x, y))
    ///     .collect();
    /// assert_eq!(positives, vec![&(1, 2), &(-1, 3)]);
    /// ```
    fn into_fn(self) -> impl Fn(&T, &U) -> bool
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
    {
        move |first, second| self.test(first, second)
    }

    fn to_box(&self) -> BoxBiPredicate<T, U>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
    {
        self.clone().into_box()
    }

    fn to_rc(&self) -> RcBiPredicate<T, U>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
    {
        self.clone().into_rc()
    }

    fn to_arc(&self) -> ArcBiPredicate<T, U>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        T: 'static,
        U: 'static,
    {
        self.clone().into_arc()
    }

    fn to_fn(&self) -> impl Fn(&T, &U) -> bool
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
    {
        self.clone().into_fn()
    }
}

/// A Box-based bi-predicate with single ownership.
///
/// This type is suitable for one-time use scenarios where the
/// bi-predicate does not need to be cloned or shared. Composition
/// methods consume `self`, reflecting the single-ownership model.
///
/// # Examples
///
/// ```rust
/// use prism3_function::bi_predicate::{BiPredicate, BoxBiPredicate};
///
/// let pred = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
/// assert!(pred.test(&5, &3));
///
/// // Chaining consumes the bi-predicate
/// let combined = pred.and(BoxBiPredicate::new(|x, y| x > y));
/// assert!(combined.test(&10, &5));
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct BoxBiPredicate<T, U> {
    function: Box<dyn Fn(&T, &U) -> bool>,
    name: Option<String>,
}

impl<T, U> BoxBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(
        BoxBiPredicate<T, U>,
        (Fn(&T, &U) -> bool + 'static),
        |f| Box::new(f)
    );

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_box_predicate_methods!(BoxBiPredicate<T, U>);
}

// Generates: impl Debug for BoxBiPredicate<T, U> and impl Display for BoxBiPredicate<T, U>
impl_predicate_debug_display!(BoxBiPredicate<T, U>);

impl<T, U> BiPredicate<T, U> for BoxBiPredicate<T, U> {
    fn test(&self, first: &T, second: &U) -> bool {
        (self.function)(first, second)
    }

    // Use optimized zero-cost conversion for into_box
    fn into_box(self) -> BoxBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        self
    }

    fn into_rc(self) -> RcBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        RcBiPredicate {
            function: Rc::new(move |first, second| (self.function)(first, second)),
            name: self.name.clone(),
        }
    }

    // do NOT override BoxBiPredicate::into_arc() because BoxBiPredicate is not Send + Sync
    // and calling BoxBiPredicate::into_arc() will cause a compile error

    fn into_fn(self) -> impl Fn(&T, &U) -> bool
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
    {
        move |first, second| (self.function)(first, second)
    }

    // do NOT override BoxBiPredicate::to_xxx() because BoxBiPredicate is not Clone
    // and calling BoxBiPredicate::to_xxx() will cause a compile error
}

/// An Rc-based bi-predicate with single-threaded shared ownership.
///
/// This type is suitable for scenarios where the bi-predicate needs
/// to be reused in a single-threaded context. Composition methods
/// borrow `&self`, allowing the original bi-predicate to remain
/// usable after composition.
///
/// # Examples
///
/// ```rust
/// use prism3_function::bi_predicate::{BiPredicate, RcBiPredicate};
///
/// let pred = RcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
/// assert!(pred.test(&5, &3));
///
/// // Original bi-predicate remains usable after composition
/// let combined = pred.and(RcBiPredicate::new(|x, y| x > y));
/// assert!(pred.test(&5, &3));  // Still works
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct RcBiPredicate<T, U> {
    function: Rc<BiPredicateFn<T, U>>,
    name: Option<String>,
}

impl<T: 'static, U: 'static> RcBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(
        RcBiPredicate<T, U>,
        (Fn(&T, &U) -> bool + 'static),
        |f| Rc::new(f)
    );

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_shared_predicate_methods!(RcBiPredicate<T, U>, 'static);
}

// Generates: impl Clone for RcBiPredicate<T, U>
impl_predicate_clone!(RcBiPredicate<T, U>);

// Generates: impl Debug for RcBiPredicate<T, U> and impl Display for RcBiPredicate<T, U>
impl_predicate_debug_display!(RcBiPredicate<T, U>);

// Implements BiPredicate trait for RcBiPredicate<T, U>
impl<T, U> BiPredicate<T, U> for RcBiPredicate<T, U> {
    fn test(&self, first: &T, second: &U) -> bool {
        (self.function)(first, second)
    }

    // Use optimized conversion for into_box that preserves the
    // existing Rc
    fn into_box(self) -> BoxBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate {
            function: Box::new(move |first, second| (self.function)(first, second)),
            name: self.name,
        }
    }

    // Use optimized zero-cost conversion for into_rc
    fn into_rc(self) -> RcBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        self
    }

    // do NOT override RcBiPredicate::into_arc() because RcBiPredicate is not Send + Sync
    // and calling RcBiPredicate::into_arc() will cause a compile error

    fn into_fn(self) -> impl Fn(&T, &U) -> bool
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
    {
        move |first, second| (self.function)(first, second)
    }

    fn to_box(&self) -> BoxBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        BoxBiPredicate {
            function: Box::new(move |first, second| self_fn(first, second)),
            name: self.name.clone(),
        }
    }

    fn to_rc(&self) -> RcBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        self.clone()
    }

    fn to_fn(&self) -> impl Fn(&T, &U) -> bool
    where
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        move |first, second| self_fn(first, second)
    }
}

/// An Arc-based bi-predicate with thread-safe shared ownership.
///
/// This type is suitable for scenarios where the bi-predicate needs
/// to be shared across threads. Composition methods borrow `&self`,
/// allowing the original bi-predicate to remain usable after
/// composition.
///
/// # Examples
///
/// ```rust
/// use prism3_function::bi_predicate::{BiPredicate, ArcBiPredicate};
///
/// let pred = ArcBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
/// assert!(pred.test(&5, &3));
///
/// // Original bi-predicate remains usable after composition
/// let combined = pred.and(ArcBiPredicate::new(|x, y| x > y));
/// assert!(pred.test(&5, &3));  // Still works
///
/// // Can be cloned and sent across threads
/// let pred_clone = pred.clone();
/// std::thread::spawn(move || {
///     assert!(pred_clone.test(&10, &5));
/// }).join().unwrap();
/// ```
///
/// # Author
///
/// Haixing Hu
pub struct ArcBiPredicate<T, U> {
    function: Arc<SendSyncBiPredicateFn<T, U>>,
    name: Option<String>,
}

impl<T: 'static, U: 'static> ArcBiPredicate<T, U> {
    // Generates: new(), new_with_name(), name(), set_name(), always_true(), always_false()
    impl_predicate_common_methods!(
        ArcBiPredicate<T, U>,
        (Fn(&T, &U) -> bool + Send + Sync + 'static),
        |f| Arc::new(f)
    );

    // Generates: and(), or(), not(), nand(), xor(), nor()
    impl_shared_predicate_methods!(ArcBiPredicate<T, U>, Send + Sync + 'static);
}

// Generates: impl Clone for ArcBiPredicate<T, U>
impl_predicate_clone!(ArcBiPredicate<T, U>);

// Generates: impl Debug for ArcBiPredicate<T, U> and impl Display for ArcBiPredicate<T, U>
impl_predicate_debug_display!(ArcBiPredicate<T, U>);

// Implements BiPredicate trait for ArcBiPredicate<T, U>
impl<T: 'static, U: 'static> BiPredicate<T, U> for ArcBiPredicate<T, U> {
    fn test(&self, first: &T, second: &U) -> bool {
        (self.function)(first, second)
    }

    // Use optimized conversion for into_box that preserves the
    // existing Arc
    fn into_box(self) -> BoxBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate {
            function: Box::new(move |first, second| (self.function)(first, second)),
            name: self.name,
        }
    }

    // Use optimized conversion for into_rc that preserves the
    // existing Arc
    fn into_rc(self) -> RcBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        RcBiPredicate {
            function: Rc::new(move |first, second| (self.function)(first, second)),
            name: self.name,
        }
    }

    // Use optimized zero-cost conversion for into_arc
    fn into_arc(self) -> ArcBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        self
    }

    // Use optimized conversion for into_fn that preserves the
    // existing Arc
    fn into_fn(self) -> impl Fn(&T, &U) -> bool
    where
        Self: Sized + 'static,
        T: 'static,
        U: 'static,
    {
        move |first, second| (self.function)(first, second)
    }

    fn to_box(&self) -> BoxBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        BoxBiPredicate {
            function: Box::new(move |first, second| self_fn(first, second)),
            name: self.name.clone(),
        }
    }

    fn to_rc(&self) -> RcBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        RcBiPredicate {
            function: Rc::new(move |first, second| self_fn(first, second)),
            name: self.name.clone(),
        }
    }

    fn to_arc(&self) -> ArcBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        self.clone()
    }

    fn to_fn(&self) -> impl Fn(&T, &U) -> bool
    where
        T: 'static,
        U: 'static,
    {
        let self_fn = self.function.clone();
        move |first, second| self_fn(first, second)
    }
}

// Blanket implementation for all closures that match
// Fn(&T, &U) -> bool. This provides optimal implementations for
// closures by wrapping them directly into the target type.
impl<T: 'static, U: 'static, F> BiPredicate<T, U> for F
where
    F: Fn(&T, &U) -> bool + 'static,
{
    fn test(&self, first: &T, second: &U) -> bool {
        self(first, second)
    }

    // Optimal implementation for closures: wrap directly in Box
    fn into_box(self) -> BoxBiPredicate<T, U> {
        BoxBiPredicate::new(self)
    }

    // Optimal implementation for closures: wrap directly in Rc
    fn into_rc(self) -> RcBiPredicate<T, U> {
        RcBiPredicate::new(self)
    }

    // Optimal implementation for closures: wrap directly in Arc
    fn into_arc(self) -> ArcBiPredicate<T, U>
    where
        Self: Send + Sync,
    {
        ArcBiPredicate::new(self)
    }

    // Optimal implementation for closures: return self (zero-cost)
    fn into_fn(self) -> impl Fn(&T, &U) -> bool {
        self
    }

    fn to_box(&self) -> BoxBiPredicate<T, U>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(self.clone())
    }

    fn to_rc(&self) -> RcBiPredicate<T, U>
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
    {
        RcBiPredicate::new(self.clone())
    }

    fn to_arc(&self) -> ArcBiPredicate<T, U>
    where
        Self: Sized + Clone + Send + Sync + 'static,
        T: 'static,
        U: 'static,
    {
        ArcBiPredicate::new(self.clone())
    }

    fn to_fn(&self) -> impl Fn(&T, &U) -> bool
    where
        Self: Sized + Clone + 'static,
        T: 'static,
        U: 'static,
    {
        self.clone()
    }
}

/// Extension trait providing logical composition methods for closures.
///
/// This trait is automatically implemented for all closures and
/// function pointers that match `Fn(&T, &U) -> bool`, enabling method
/// chaining starting from a closure.
///
/// # Examples
///
/// ```rust
/// use prism3_function::bi_predicate::{BiPredicate, FnBiPredicateOps};
///
/// let is_sum_positive = |x: &i32, y: &i32| x + y > 0;
/// let first_larger = |x: &i32, y: &i32| x > y;
///
/// // Combine bi-predicates using extension methods
/// let pred = is_sum_positive.and(first_larger);
/// assert!(pred.test(&10, &5));
/// assert!(!pred.test(&3, &8));
/// ```
///
/// # Author
///
/// Haixing Hu
pub trait FnBiPredicateOps<T, U>: Fn(&T, &U) -> bool + Sized + 'static {
    /// Returns a bi-predicate that represents the logical AND of this
    /// bi-predicate and another.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical AND.
    fn and<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| self(first, second) && other.test(first, second))
    }

    /// Returns a bi-predicate that represents the logical OR of this
    /// bi-predicate and another.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical OR.
    fn or<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| self(first, second) || other.test(first, second))
    }

    /// Returns a bi-predicate that represents the logical negation of
    /// this bi-predicate.
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical negation.
    fn not(self) -> BoxBiPredicate<T, U>
    where
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| !self(first, second))
    }

    /// Returns a bi-predicate that represents the logical NAND (NOT
    /// AND) of this bi-predicate and another.
    ///
    /// NAND returns `true` unless both bi-predicates are `true`.
    /// Equivalent to `!(self AND other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical NAND.
    fn nand<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| {
            !(self(first, second) && other.test(first, second))
        })
    }

    /// Returns a bi-predicate that represents the logical XOR
    /// (exclusive OR) of this bi-predicate and another.
    ///
    /// XOR returns `true` if exactly one of the bi-predicates is
    /// `true`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical XOR.
    fn xor<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| self(first, second) ^ other.test(first, second))
    }

    /// Returns a bi-predicate that represents the logical NOR (NOT
    /// OR) of this bi-predicate and another.
    ///
    /// NOR returns `true` only if both bi-predicates are `false`.
    /// Equivalent to `!(self OR other)`.
    ///
    /// # Parameters
    ///
    /// * `other` - The other bi-predicate to combine with. **Note: This parameter
    ///   is passed by value and will transfer ownership.** If you need to
    ///   preserve the original bi-predicate, clone it first (if it implements
    ///   `Clone`). Can be:
    ///   - Another closure: `|x: &T, y: &U| -> bool`
    ///   - A function pointer: `fn(&T, &U) -> bool`
    ///   - A `BoxBiPredicate<T, U>`
    ///   - An `RcBiPredicate<T, U>`
    ///   - An `ArcBiPredicate<T, U>`
    ///   - Any type implementing `BiPredicate<T, U>`
    ///
    /// # Returns
    ///
    /// A `BoxBiPredicate` representing the logical NOR.
    fn nor<P>(self, other: P) -> BoxBiPredicate<T, U>
    where
        P: BiPredicate<T, U> + 'static,
        T: 'static,
        U: 'static,
    {
        BoxBiPredicate::new(move |first, second| {
            !(self(first, second) || other.test(first, second))
        })
    }
}

// Blanket implementation for all closures
impl<T, U, F> FnBiPredicateOps<T, U> for F where F: Fn(&T, &U) -> bool + 'static {}
