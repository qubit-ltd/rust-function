/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # Functions Module
//!
//! This module provides function-related functional programming abstractions
//! for transforming values from one type to another with reference semantics.
//!
//! # Author
//!
//! Haixing Hu

pub mod bi_function;
pub mod bi_function_once;
pub mod bi_mutating_function;
pub mod bi_mutating_function_once;
pub mod function;
pub mod function_once;
pub mod macros;
pub mod mutating_function;
pub mod mutating_function_once;
pub mod stateful_function;
pub mod stateful_mutating_function;

pub use bi_function::{
    ArcBiFunction,
    ArcBinaryFunction,
    ArcConditionalBiFunction,
    BiFunction,
    BoxBiFunction,
    BoxBinaryFunction,
    BoxConditionalBiFunction,
    FnBiFunctionOps,
    RcBiFunction,
    RcBinaryFunction,
    RcConditionalBiFunction,
};
pub use bi_function_once::{
    BiFunctionOnce,
    BoxBiFunctionOnce,
    FnBiFunctionOnceOps,
};
pub use bi_mutating_function::{
    ArcBiMutatingFunction,
    ArcBinaryMutatingFunction,
    ArcConditionalBiMutatingFunction,
    BiMutatingFunction,
    BoxBiMutatingFunction,
    BoxBinaryMutatingFunction,
    BoxConditionalBiMutatingFunction,
    FnBiMutatingFunctionOps,
    RcBiMutatingFunction,
    RcBinaryMutatingFunction,
    RcConditionalBiMutatingFunction,
};
pub use bi_mutating_function_once::{
    BiMutatingFunctionOnce,
    BoxBiMutatingFunctionOnce,
    BoxConditionalBiMutatingFunctionOnce,
    FnBiMutatingFunctionOnceOps,
};
pub use function::{
    ArcConditionalFunction,
    ArcFunction,
    BoxConditionalFunction,
    BoxFunction,
    FnFunctionOps,
    Function,
    RcConditionalFunction,
    RcFunction,
};
pub use function_once::{
    BoxFunctionOnce,
    FnFunctionOnceOps,
    FunctionOnce,
};
pub use mutating_function::{
    ArcMutatingFunction,
    BoxMutatingFunction,
    FnMutatingFunctionOps,
    MutatingFunction,
    RcMutatingFunction,
};
pub use mutating_function_once::{
    BoxMutatingFunctionOnce,
    FnMutatingFunctionOnceOps,
    MutatingFunctionOnce,
};
pub use stateful_function::{
    ArcConditionalStatefulFunction,
    ArcStatefulFunction,
    BoxConditionalStatefulFunction,
    BoxStatefulFunction,
    FnStatefulFunctionOps,
    RcConditionalStatefulFunction,
    RcStatefulFunction,
    StatefulFunction,
};
pub use stateful_mutating_function::{
    ArcStatefulMutatingFunction,
    BoxStatefulMutatingFunction,
    FnStatefulMutatingFunctionOps,
    RcStatefulMutatingFunction,
    StatefulMutatingFunction,
};
