/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Prism3 Function
//!
//! Provides functional programming abstractions for Rust, including:
//!
//! - **Transformer types**: Transform values from type T to type R
//! - **UnaryOperator types**: Transform values of type T to the same type T
//! - **BiTransformer types**: Transform two values to produce a result
//! - **BinaryOperator types**: Transform two values of type T to produce a T
//! - **Consumer types**: Functions that consume values without returning
//! - **BiConsumer types**: Functions that consume two values without returning
//! - **Predicate types**: Functions that test values and return boolean
//! - **BiPredicate types**: Functions that test two values and return boolean
//! - **Supplier types**: Functions that produce values without input
//! - **Mapper types**: Stateful transformations from type T to type R
//! - **Tester types**: Functions that test conditions without input
//! - **Comparator types**: Functions that compare values and return ordering
//!
//! # Author
//!
//! Haixing Hu

// Module declarations
pub mod comparator;
pub mod consumers;
pub mod functions;
pub mod macros;
pub mod mutators;
pub mod predicates;
pub mod suppliers;
pub mod tester;
pub mod transformers;

// Re-export all types from submodules for backward compatibility
// Types are organized by functionality and ownership model for better readability

// =============================================================================
// Core Functional Types
// =============================================================================

// ---- Consumer Types (Fn(&T)) ----
pub use consumers::{
    // Arc-based (shared multi-threaded ownership)
    ArcConsumer,
    ArcStatefulConsumer,

    // Box-based (single ownership)
    BoxConsumer,
    BoxConsumerOnce,
    BoxStatefulConsumer,

    // Core traits
    Consumer,
    ConsumerOnce,
    FnConsumerOnceOps,
    // Extension traits
    FnConsumerOps,
    FnStatefulConsumerOps,
    // Rc-based (shared single-threaded ownership)
    RcConsumer,
    RcStatefulConsumer,

    StatefulConsumer,
};

// ---- BiConsumer Types (Fn(&T, &U)) ----
pub use consumers::{
    // Arc-based (shared multi-threaded ownership)
    ArcBiConsumer,
    ArcStatefulBiConsumer,

    // Core traits
    BiConsumer,
    BiConsumerOnce,
    // Box-based (single ownership)
    BoxBiConsumer,
    BoxBiConsumerOnce,
    BoxStatefulBiConsumer,

    FnBiConsumerOnceOps,
    // Extension traits
    FnBiConsumerOps,
    FnStatefulBiConsumerOps,
    // Rc-based (shared single-threaded ownership)
    RcBiConsumer,
    RcStatefulBiConsumer,

    StatefulBiConsumer,
};

// ---- Function Types (Fn(&T) -> R) ----
pub use functions::{
    // Arc-based (shared multi-threaded ownership)
    ArcFunction,
    ArcMutatingFunction,
    ArcStatefulFunction,
    ArcStatefulMutatingFunction,

    // Box-based (single ownership)
    BoxFunction,
    BoxFunctionOnce,
    BoxMutatingFunction,
    BoxMutatingFunctionOnce,
    BoxStatefulFunction,
    BoxStatefulMutatingFunction,

    FnFunctionOnceOps,
    // Extension traits
    FnFunctionOps,
    FnMutatingFunctionOnceOps,
    FnMutatingFunctionOps,
    FnStatefulFunctionOps,
    FnStatefulMutatingFunctionOps,
    // Core traits
    Function,
    FunctionOnce,
    MutatingFunction,
    MutatingFunctionOnce,
    // Rc-based (shared single-threaded ownership)
    RcFunction,
    RcMutatingFunction,
    RcStatefulFunction,
    RcStatefulMutatingFunction,

    StatefulFunction,
    StatefulMutatingFunction,
};

// ---- BiFunction Types (Fn(&T, &U) -> R) ----
pub use functions::{
    // Arc-based (shared multi-threaded ownership)
    ArcBiFunction,
    ArcBiMutatingFunction,

    // Core traits
    BiFunction,
    BiFunctionOnce,
    BiMutatingFunction,
    BiMutatingFunctionOnce,

    // Box-based (single ownership)
    BoxBiFunction,
    BoxBiFunctionOnce,
    BoxBiMutatingFunction,
    BoxBiMutatingFunctionOnce,

    FnBiFunctionOnceOps,
    // Extension traits
    FnBiFunctionOps,
    FnBiMutatingFunctionOnceOps,
    FnBiMutatingFunctionOps,
    // Rc-based (shared single-threaded ownership)
    RcBiFunction,
    RcBiMutatingFunction,
};

// ---- Binary Function Types (Fn(&T, &T) -> R) ----
pub use functions::{
    // Arc-based (shared multi-threaded ownership)
    ArcBinaryFunction,
    ArcBinaryMutatingFunction,
    // Box-based (single ownership)
    BoxBinaryFunction,
    BoxBinaryMutatingFunction,

    // Rc-based (shared single-threaded ownership)
    RcBinaryFunction,
    RcBinaryMutatingFunction,
};

// ---- Conditional Function Types ----
pub use functions::{
    ArcConditionalBiFunction,
    ArcConditionalBiMutatingFunction,
    // Arc-based (shared multi-threaded ownership)
    ArcConditionalFunction,
    ArcConditionalStatefulFunction,
    BoxConditionalBiFunction,
    BoxConditionalBiMutatingFunction,
    BoxConditionalBiMutatingFunctionOnce,

    // Box-based (single ownership)
    BoxConditionalFunction,
    BoxConditionalStatefulFunction,
    RcConditionalBiFunction,
    RcConditionalBiMutatingFunction,

    // Rc-based (shared single-threaded ownership)
    RcConditionalFunction,
    RcConditionalStatefulFunction,
};

// =============================================================================
// Data Processing Types
// =============================================================================

// ---- Transformer Types (Fn(T) -> R) ----
pub use transformers::{
    ArcStatefulBiTransformer,

    ArcStatefulTransformer,
    // Arc-based (shared multi-threaded ownership)
    ArcTransformer,
    BoxStatefulBiTransformer,

    BoxStatefulTransformer,
    // Box-based (single ownership)
    BoxTransformer,
    BoxTransformerOnce,
    FnStatefulBiTransformerOps,
    FnStatefulTransformerOps,
    FnTransformerOnceOps,
    // Extension traits
    FnTransformerOps,
    RcStatefulBiTransformer,

    RcStatefulTransformer,
    // Rc-based (shared single-threaded ownership)
    RcTransformer,
    StatefulBiTransformer,

    StatefulTransformer,
    // Core traits
    Transformer,
    TransformerOnce,
};

// ---- BiTransformer Types (Fn(T, U) -> R) ----
pub use transformers::{
    // Arc-based (shared multi-threaded ownership)
    ArcBiTransformer,

    // Core traits
    BiTransformer,
    BiTransformerOnce,

    // Box-based (single ownership)
    BoxBiTransformer,
    BoxBiTransformerOnce,

    FnBiTransformerOnceOps,
    // Extension traits
    FnBiTransformerOps,
    // Rc-based (shared single-threaded ownership)
    RcBiTransformer,
};

// ---- Operator Types ----
pub use transformers::{
    ArcBinaryOperator,
    ArcUnaryOperator,

    // Binary operators (Fn(T, T) -> T)
    BinaryOperator,
    BinaryOperatorOnce,
    BoxBinaryOperator,
    BoxBinaryOperatorOnce,
    BoxUnaryOperator,
    BoxUnaryOperatorOnce,
    RcBinaryOperator,
    RcUnaryOperator,
    // Unary operators (Fn(T) -> T)
    UnaryOperator,
    UnaryOperatorOnce,
};

// ---- Conditional Transformer Types ----
pub use transformers::{
    ArcConditionalStatefulBiTransformer,
    ArcConditionalStatefulTransformer,
    // Arc-based (shared multi-threaded ownership)
    ArcConditionalTransformer,
    BoxConditionalStatefulBiTransformer,

    BoxConditionalStatefulTransformer,
    // Box-based (single ownership)
    BoxConditionalTransformer,
    BoxConditionalTransformerOnce,
    RcConditionalStatefulBiTransformer,

    RcConditionalStatefulTransformer,
    // Rc-based (shared single-threaded ownership)
    RcConditionalTransformer,
};

// =============================================================================
// Utility Types
// =============================================================================

// ---- Mutator Types (Fn(&mut T)) ----
pub use mutators::{
    ArcConditionalMutator,
    ArcConditionalStatefulMutator,

    // Arc-based (shared multi-threaded ownership)
    ArcMutator,
    ArcStatefulMutator,

    // Conditional types
    BoxConditionalMutator,
    BoxConditionalMutatorOnce,
    BoxConditionalStatefulMutator,
    // Box-based (single ownership)
    BoxMutator,
    BoxMutatorOnce,
    BoxStatefulMutator,

    FnMutStatefulMutatorOps,
    FnMutatorOnceOps,
    // Extension traits
    FnMutatorOps,
    // Core traits
    Mutator,
    MutatorOnce,
    RcConditionalMutator,
    RcConditionalStatefulMutator,
    // Rc-based (shared single-threaded ownership)
    RcMutator,
    RcStatefulMutator,

    StatefulMutator,
};

// ---- Predicate Types (Fn(&T) -> bool) ----
pub use predicates::{
    // Arc-based (shared multi-threaded ownership)
    ArcPredicate,

    // Box-based (single ownership)
    BoxPredicate,

    // Extension traits
    FnPredicateOps,
    // Core traits
    Predicate,

    // Rc-based (shared single-threaded ownership)
    RcPredicate,
};

// ---- BiPredicate Types (Fn(&T, &U) -> bool) ----
pub use predicates::{
    // Arc-based (shared multi-threaded ownership)
    ArcBiPredicate,

    // Core traits
    BiPredicate,

    // Box-based (single ownership)
    BoxBiPredicate,

    // Extension traits
    FnBiPredicateOps,
    // Rc-based (shared single-threaded ownership)
    RcBiPredicate,
};

// ---- Supplier Types (Fn() -> R) ----
pub use suppliers::{
    ArcStatefulSupplier,

    // Arc-based (shared multi-threaded ownership)
    ArcSupplier,
    BoxStatefulSupplier,

    // Box-based (single ownership)
    BoxSupplier,
    BoxSupplierOnce,
    // Extension traits
    FnStatefulSupplierOps,
    RcStatefulSupplier,

    // Rc-based (shared single-threaded ownership)
    RcSupplier,
    StatefulSupplier,

    // Core traits
    Supplier,
    SupplierOnce,
};

// ---- Comparator Types (Fn(&T, &T) -> Ordering) ----
pub use comparator::{
    // Arc-based (shared multi-threaded ownership)
    ArcComparator,

    // Box-based (single ownership)
    BoxComparator,

    // Core traits
    Comparator,

    // Extension traits
    FnComparatorOps,
    // Rc-based (shared single-threaded ownership)
    RcComparator,
};

// ---- Tester Types (FnMut() -> bool) ----
pub use tester::{
    // Arc-based (shared multi-threaded ownership)
    ArcTester,

    // Box-based (single ownership)
    BoxTester,

    // Extension traits
    FnTesterOps,
    // Rc-based (shared single-threaded ownership)
    RcTester,

    // Core traits
    Tester,
};
