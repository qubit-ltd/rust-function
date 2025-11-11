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
    // Core traits
    Consumer,
    ConsumerOnce,
    StatefulConsumer,

    // Box-based (single ownership)
    BoxConsumer,
    BoxConsumerOnce,
    BoxStatefulConsumer,

    // Rc-based (shared single-threaded ownership)
    RcConsumer,
    RcStatefulConsumer,

    // Arc-based (shared multi-threaded ownership)
    ArcConsumer,
    ArcStatefulConsumer,

    // Extension traits
    FnConsumerOps,
    FnConsumerOnceOps,
    FnStatefulConsumerOps,
};

// ---- BiConsumer Types (Fn(&T, &U)) ----
pub use consumers::{
    // Core traits
    BiConsumer,
    BiConsumerOnce,
    StatefulBiConsumer,

    // Box-based (single ownership)
    BoxBiConsumer,
    BoxBiConsumerOnce,
    BoxStatefulBiConsumer,

    // Rc-based (shared single-threaded ownership)
    RcBiConsumer,
    RcStatefulBiConsumer,

    // Arc-based (shared multi-threaded ownership)
    ArcBiConsumer,
    ArcStatefulBiConsumer,

    // Extension traits
    FnBiConsumerOps,
    FnBiConsumerOnceOps,
    FnStatefulBiConsumerOps,
};

// ---- Function Types (Fn(&T) -> R) ----
pub use functions::{
    // Core traits
    Function,
    FunctionOnce,
    StatefulFunction,
    MutatingFunction,
    MutatingFunctionOnce,
    StatefulMutatingFunction,

    // Box-based (single ownership)
    BoxFunction,
    BoxFunctionOnce,
    BoxStatefulFunction,
    BoxMutatingFunction,
    BoxMutatingFunctionOnce,
    BoxStatefulMutatingFunction,

    // Rc-based (shared single-threaded ownership)
    RcFunction,
    RcStatefulFunction,
    RcMutatingFunction,
    RcStatefulMutatingFunction,

    // Arc-based (shared multi-threaded ownership)
    ArcFunction,
    ArcStatefulFunction,
    ArcMutatingFunction,
    ArcStatefulMutatingFunction,

    // Extension traits
    FnFunctionOps,
    FnFunctionOnceOps,
    FnStatefulFunctionOps,
    FnMutatingFunctionOps,
    FnMutatingFunctionOnceOps,
    FnStatefulMutatingFunctionOps,
};

// ---- BiFunction Types (Fn(&T, &U) -> R) ----
pub use functions::{
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

    // Rc-based (shared single-threaded ownership)
    RcBiFunction,
    RcBiMutatingFunction,

    // Arc-based (shared multi-threaded ownership)
    ArcBiFunction,
    ArcBiMutatingFunction,

    // Extension traits
    FnBiFunctionOps,
    FnBiFunctionOnceOps,
    FnBiMutatingFunctionOps,
    FnBiMutatingFunctionOnceOps,
};

// ---- Binary Function Types (Fn(&T, &T) -> R) ----
pub use functions::{
    // Box-based (single ownership)
    BoxBinaryFunction,
    BoxBinaryMutatingFunction,

    // Rc-based (shared single-threaded ownership)
    RcBinaryFunction,
    RcBinaryMutatingFunction,

    // Arc-based (shared multi-threaded ownership)
    ArcBinaryFunction,
    ArcBinaryMutatingFunction,
};

// ---- Conditional Function Types ----
pub use functions::{
    // Box-based (single ownership)
    BoxConditionalFunction,
    BoxConditionalBiFunction,
    BoxConditionalStatefulFunction,
    BoxConditionalBiMutatingFunction,
    BoxConditionalBiMutatingFunctionOnce,

    // Rc-based (shared single-threaded ownership)
    RcConditionalFunction,
    RcConditionalBiFunction,
    RcConditionalStatefulFunction,
    RcConditionalBiMutatingFunction,

    // Arc-based (shared multi-threaded ownership)
    ArcConditionalFunction,
    ArcConditionalBiFunction,
    ArcConditionalStatefulFunction,
    ArcConditionalBiMutatingFunction,
};

// =============================================================================
// Data Processing Types
// =============================================================================

// ---- Transformer Types (Fn(T) -> R) ----
pub use transformers::{
    // Core traits
    Transformer,
    TransformerOnce,
    StatefulTransformer,
    StatefulBiTransformer,

    // Box-based (single ownership)
    BoxTransformer,
    BoxTransformerOnce,
    BoxStatefulTransformer,
    BoxStatefulBiTransformer,

    // Rc-based (shared single-threaded ownership)
    RcTransformer,
    RcStatefulTransformer,
    RcStatefulBiTransformer,

    // Arc-based (shared multi-threaded ownership)
    ArcTransformer,
    ArcStatefulTransformer,
    ArcStatefulBiTransformer,

    // Extension traits
    FnTransformerOps,
    FnTransformerOnceOps,
    FnStatefulTransformerOps,
    FnStatefulBiTransformerOps,
};

// ---- BiTransformer Types (Fn(T, U) -> R) ----
pub use transformers::{
    // Core traits
    BiTransformer,
    BiTransformerOnce,

    // Box-based (single ownership)
    BoxBiTransformer,
    BoxBiTransformerOnce,

    // Rc-based (shared single-threaded ownership)
    RcBiTransformer,

    // Arc-based (shared multi-threaded ownership)
    ArcBiTransformer,

    // Extension traits
    FnBiTransformerOps,
    FnBiTransformerOnceOps,
};

// ---- Operator Types ----
pub use transformers::{
    // Unary operators (Fn(T) -> T)
    UnaryOperator,
    UnaryOperatorOnce,
    BoxUnaryOperator,
    BoxUnaryOperatorOnce,
    RcUnaryOperator,
    ArcUnaryOperator,

    // Binary operators (Fn(T, T) -> T)
    BinaryOperator,
    BinaryOperatorOnce,
    BoxBinaryOperator,
    BoxBinaryOperatorOnce,
    RcBinaryOperator,
    ArcBinaryOperator,
};

// ---- Conditional Transformer Types ----
pub use transformers::{
    // Box-based (single ownership)
    BoxConditionalTransformer,
    BoxConditionalTransformerOnce,
    BoxConditionalStatefulTransformer,
    BoxConditionalStatefulBiTransformer,

    // Rc-based (shared single-threaded ownership)
    RcConditionalTransformer,
    RcConditionalStatefulTransformer,
    RcConditionalStatefulBiTransformer,

    // Arc-based (shared multi-threaded ownership)
    ArcConditionalTransformer,
    ArcConditionalStatefulTransformer,
    ArcConditionalStatefulBiTransformer,
};

// =============================================================================
// Utility Types
// =============================================================================

// ---- Mutator Types (Fn(&mut T)) ----
pub use mutators::{
    // Core traits
    Mutator,
    MutatorOnce,
    StatefulMutator,

    // Box-based (single ownership)
    BoxMutator,
    BoxMutatorOnce,
    BoxStatefulMutator,

    // Rc-based (shared single-threaded ownership)
    RcMutator,
    RcStatefulMutator,

    // Arc-based (shared multi-threaded ownership)
    ArcMutator,
    ArcStatefulMutator,

    // Conditional types
    BoxConditionalMutator,
    BoxConditionalMutatorOnce,
    BoxConditionalStatefulMutator,
    RcConditionalMutator,
    RcConditionalStatefulMutator,
    ArcConditionalMutator,
    ArcConditionalStatefulMutator,

    // Extension traits
    FnMutatorOps,
    FnMutatorOnceOps,
    FnMutStatefulMutatorOps,
};

// ---- Predicate Types (Fn(&T) -> bool) ----
pub use predicates::{
    // Core traits
    Predicate,

    // Box-based (single ownership)
    BoxPredicate,

    // Rc-based (shared single-threaded ownership)
    RcPredicate,

    // Arc-based (shared multi-threaded ownership)
    ArcPredicate,

    // Extension traits
    FnPredicateOps,
};

// ---- BiPredicate Types (Fn(&T, &U) -> bool) ----
pub use predicates::{
    // Core traits
    BiPredicate,

    // Box-based (single ownership)
    BoxBiPredicate,

    // Rc-based (shared single-threaded ownership)
    RcBiPredicate,

    // Arc-based (shared multi-threaded ownership)
    ArcBiPredicate,

    // Extension traits
    FnBiPredicateOps,
};

// ---- Supplier Types (Fn() -> R) ----
pub use suppliers::{
    // Core traits
    Supplier,
    SupplierOnce,
    StatefulSupplier,

    // Box-based (single ownership)
    BoxSupplier,
    BoxSupplierOnce,
    BoxStatefulSupplier,

    // Rc-based (shared single-threaded ownership)
    RcSupplier,
    RcStatefulSupplier,

    // Arc-based (shared multi-threaded ownership)
    ArcSupplier,
    ArcStatefulSupplier,

    // Extension traits
    FnStatefulSupplierOps,
};

// ---- Comparator Types (Fn(&T, &T) -> Ordering) ----
pub use comparator::{
    // Core traits
    Comparator,

    // Box-based (single ownership)
    BoxComparator,

    // Rc-based (shared single-threaded ownership)
    RcComparator,

    // Arc-based (shared multi-threaded ownership)
    ArcComparator,

    // Extension traits
    FnComparatorOps,
};

// ---- Tester Types (FnMut() -> bool) ----
pub use tester::{
    // Core traits
    Tester,

    // Box-based (single ownership)
    BoxTester,

    // Rc-based (shared single-threaded ownership)
    RcTester,

    // Arc-based (shared multi-threaded ownership)
    ArcTester,

    // Extension traits
    FnTesterOps,
};
