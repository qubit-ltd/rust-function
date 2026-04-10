# Qubit Function

[![CircleCI](https://img.shields.io/circleci/build/github/qubit-ltd/rust-function/main?style=shield&logo=circleci)](https://circleci.com/gh/qubit-ltd/rust-function)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rust-function/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rust-function?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-function.svg?color=blue)](https://crates.io/crates/qubit-function)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![中文文档](https://img.shields.io/badge/文档-中文版-blue.svg)](README.zh_CN.md)

Comprehensive functional programming abstractions for Rust, providing Java-style functional interfaces adapted to Rust's ownership model.

## Overview

This crate provides a complete set of functional programming abstractions inspired by Java's functional interfaces, carefully adapted to Rust's ownership system. It offers multiple implementations for each abstraction (Box/Arc/Rc) to cover various use cases from simple single-threaded scenarios to complex multi-threaded applications.

## Key Features

- **Complete Functional Interface Suite**: 24 core functional abstractions with multiple variants
- **High-Performance Concurrency**: Uses parking_lot Mutex for superior thread synchronization performance
- **Multiple Ownership Models**: Box-based single ownership, Arc-based thread-safe sharing, and Rc-based single-threaded sharing
- **Flexible API Design**: Trait-based unified interface with concrete implementations optimized for different scenarios
- **Method Chaining**: All types support fluent API and functional composition
- **Thread-Safety Options**: Choose between thread-safe (Arc) and efficient single-threaded (Rc) implementations
- **Zero-Cost Abstractions**: Efficient implementations with minimal runtime overhead

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
qubit-function = "0.7.1"
```

## Core Abstractions

This crate provides 24 core functional abstractions, each with multiple implementations:

### 1. Predicate - Single-Argument Predicate

Tests whether a value satisfies a condition, returning `bool`.

**Trait**: `Predicate<T>`
**Core Method**: `test(&self, value: &T) -> bool`
**Closure Equivalent**: `Fn(&T) -> bool`

**Implementations**:
- `BoxPredicate<T>` - Single ownership, non-cloneable
- `ArcPredicate<T>` - Thread-safe, cloneable
- `RcPredicate<T>` - Single-threaded, cloneable

**Example**:
```rust
use qubit_function::{Predicate, ArcPredicate};

let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);
let is_positive = ArcPredicate::new(|x: &i32| *x > 0);

let combined = is_even.and(is_positive.clone());
assert!(combined.test(&4));
assert!(!combined.test(&-2));
```

### 2. BiPredicate - Two-Argument Predicate

Tests whether two values satisfy a condition, returning `bool`.

**Trait**: `BiPredicate<T, U>`
**Core Method**: `test(&self, first: &T, second: &U) -> bool`
**Closure Equivalent**: `Fn(&T, &U) -> bool`

**Implementations**:
- `BoxBiPredicate<T, U>` - Single ownership
- `ArcBiPredicate<T, U>` - Thread-safe
- `RcBiPredicate<T, U>` - Single-threaded

**Example**:
```rust
use qubit_function::{BiPredicate, BoxBiPredicate};

let sum_positive = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
assert!(sum_positive.test(&3, &4));
assert!(!sum_positive.test(&-5, &2));
```

### 3. Consumer - Read-Only Consumer

Accepts a value reference and performs side effects without returning a
result.

**Trait**: `Consumer<T>`
**Core Method**: `accept(&self, value: &T)`
**Closure Equivalent**: `Fn(&T)`

**Implementations**:
- `BoxConsumer<T>` - Single ownership
- `ArcConsumer<T>` - Thread-safe
- `RcConsumer<T>` - Single-threaded

**Example**:
```rust
use qubit_function::{Consumer, BoxConsumer};

let logger = BoxConsumer::new(|x: &i32| {
    println!("Value: {}", x);
});
logger.accept(&42);
```

### 4. ConsumerOnce - Single-Use Read-Only Consumer

Accepts a value reference and performs side effects once.

**Trait**: `ConsumerOnce<T>`
**Core Method**: `accept(self, value: &T)`
**Closure Equivalent**: `FnOnce(&T)`

**Implementations**:
- `BoxConsumerOnce<T>` - Single ownership, one-time use

### 5. BiConsumer - Two-Argument Read-Only Consumer

Accepts two value references and performs side effects without returning a
result.

**Trait**: `BiConsumer<T, U>`
**Core Method**: `accept(&self, first: &T, second: &U)`
**Closure Equivalent**: `Fn(&T, &U)`

**Implementations**:
- `BoxBiConsumer<T, U>` - Single ownership
- `ArcBiConsumer<T, U>` - Thread-safe
- `RcBiConsumer<T, U>` - Single-threaded

**Example**:
```rust
use qubit_function::{BiConsumer, BoxBiConsumer};

let sum_logger = BoxBiConsumer::new(|x: &i32, y: &i32| {
    println!("Sum: {}", x + y);
});
sum_logger.accept(&10, &20);
```

### 6. BiConsumerOnce - Single-Use Two-Argument Read-Only Consumer

Accepts two value references and performs side effects once.

**Trait**: `BiConsumerOnce<T, U>`
**Core Method**: `accept(self, first: &T, second: &U)`
**Closure Equivalent**: `FnOnce(&T, &U)`

**Implementations**:
- `BoxBiConsumerOnce<T, U>` - Single ownership, one-time use

### 7. Mutator - Stateless In-Place Mutator

Modifies the target value in place via `&mut T` with no return value. The mutator itself is **stateless** and is invoked with `&self` (equivalent to `Fn(&mut T)`).

**Trait**: `Mutator<T>`
**Core Method**: `apply(&self, value: &mut T)`
**Closure Equivalent**: `Fn(&mut T)`

**Implementations**:
- `BoxMutator<T>` - Single ownership
- `ArcMutator<T>` - Thread-safe
- `RcMutator<T>` - Single-threaded

**Example**:
```rust
use qubit_function::{Mutator, BoxMutator};

let mut doubler = BoxMutator::new(|x: &mut i32| *x *= 2);
let mut value = 10;
doubler.apply(&mut value);
assert_eq!(value, 20);
```

### 8. MutatorOnce - Single-Use In-Place Mutator

May be invoked once to mutate the target in place via `&mut T` (equivalent to `FnOnce(&mut T)`).

**Trait**: `MutatorOnce<T>`
**Core Method**: `apply(self, value: &mut T)`
**Closure Equivalent**: `FnOnce(&mut T)`

**Implementations**:
- `BoxMutatorOnce<T>` - Single ownership, one-time use

### 9. Supplier - Stateless Value Supplier

Returns a value of type `T` on each `get` call with no input. The
supplier itself is **stateless** and uses `&self` (equivalent to
`Fn() -> T`).

**Trait**: `Supplier<T>`
**Core Method**: `get(&self) -> T`
**Closure Equivalent**: `Fn() -> T`

**Implementations**:
- `BoxSupplier<T>` - Single ownership, lock-free
- `ArcSupplier<T>` - Thread-safe, lock-free
- `RcSupplier<T>` - Single-threaded

**Example**:
```rust
use qubit_function::{Supplier, BoxSupplier};

let factory = BoxSupplier::new(|| String::from("Hello"));
assert_eq!(factory.get(), "Hello");
```

### 10. SupplierOnce - Single-Use Value Supplier

May invoke `get` only once to return a single `T` (equivalent to
`FnOnce() -> T`).

**Trait**: `SupplierOnce<T>`
**Core Method**: `get(self) -> T`
**Closure Equivalent**: `FnOnce() -> T`

**Implementations**:
- `BoxSupplierOnce<T>` - Single ownership, one-time use

### 11. StatefulSupplier - Stateful Value Supplier

Supplies a `T` using mutable internal state; successive `get` calls may differ (equivalent to `FnMut() -> T`).

**Trait**: `StatefulSupplier<T>`
**Core Method**: `get(&mut self) -> T`
**Closure Equivalent**: `FnMut() -> T`

**Implementations**:
- `BoxStatefulSupplier<T>` - Single ownership
- `ArcStatefulSupplier<T>` - Thread-safe with parking_lot::Mutex
- `RcStatefulSupplier<T>` - Single-threaded with RefCell

**Example**:
```rust
use qubit_function::{StatefulSupplier, BoxStatefulSupplier};

let mut counter = {
    let mut count = 0;
    BoxStatefulSupplier::new(move || {
        count += 1;
        count
    })
};

assert_eq!(counter.get(), 1);
assert_eq!(counter.get(), 2);
```

### 12. Function - Borrowed-Input Function

Computes a result from a borrowed input without consuming the input.

**Trait**: `Function<T, R>`
**Core Method**: `apply(&self, input: &T) -> R`
**Closure Equivalent**: `Fn(&T) -> R`

**Implementations**:
- `BoxFunction<T, R>` - Single ownership
- `ArcFunction<T, R>` - Thread-safe
- `RcFunction<T, R>` - Single-threaded

**Example**:
```rust
use qubit_function::{Function, BoxFunction};

let to_string = BoxFunction::new(|x: &i32| format!("Value: {}", x));
assert_eq!(to_string.apply(&42), "Value: 42");
```

### 13. FunctionOnce - Single-Use Borrowed-Input Function

Computes a result from a borrowed input once.

**Trait**: `FunctionOnce<T, R>`
**Core Method**: `apply(self, input: &T) -> R`
**Closure Equivalent**: `FnOnce(&T) -> R`

**Implementations**:
- `BoxFunctionOnce<T, R>` - Single ownership, one-time use

### 14. StatefulFunction - Stateful Borrowed-Input Function

Computes a result from a borrowed input while allowing mutable internal
state.

**Trait**: `StatefulFunction<T, R>`
**Core Method**: `apply(&mut self, input: &T) -> R`
**Closure Equivalent**: `FnMut(&T) -> R`

**Implementations**:
- `BoxStatefulFunction<T, R>` - Single ownership
- `ArcStatefulFunction<T, R>` - Thread-safe with parking_lot::Mutex
- `RcStatefulFunction<T, R>` - Single-threaded with RefCell

### 15. Transformer - Value Transformer

Consumes an input value of type `T` and transforms it into a value of
type `R`.

**Trait**: `Transformer<T, R>`
**Core Method**: `apply(&self, input: T) -> R`
**Closure Equivalent**: `Fn(T) -> R`

**Implementations**:
- `BoxTransformer<T, R>` - Single ownership
- `ArcTransformer<T, R>` - Thread-safe
- `RcTransformer<T, R>` - Single-threaded

**Type Alias**: `UnaryOperator<T>` = `Transformer<T, T>`

**Example**:
```rust
use qubit_function::{Transformer, BoxTransformer};

let parse = BoxTransformer::new(|s: String| s.parse::<i32>().unwrap_or(0));
assert_eq!(parse.apply("42".to_string()), 42);
```

### 16. TransformerOnce - Single-Use Value Transformer

Consumes an input value once and transforms it into a value of type `R`.

**Trait**: `TransformerOnce<T, R>`
**Core Method**: `apply(self, input: T) -> R`
**Closure Equivalent**: `FnOnce(T) -> R`

**Implementations**:
- `BoxTransformerOnce<T, R>` - Single ownership, one-time use

**Type Alias**: `UnaryOperatorOnce<T>` = `TransformerOnce<T, T>`

### 17. StatefulTransformer - Stateful Value Transformer

Consumes an input value and transforms it into a value of type `R`
while allowing mutable internal state.

**Trait**: `StatefulTransformer<T, R>`
**Core Method**: `apply(&mut self, input: T) -> R`
**Closure Equivalent**: `FnMut(T) -> R`

**Implementations**:
- `BoxStatefulTransformer<T, R>` - Single ownership
- `ArcStatefulTransformer<T, R>` - Thread-safe with parking_lot::Mutex
- `RcStatefulTransformer<T, R>` - Single-threaded with RefCell

### 18. BiTransformer - Two-Argument Value Transformer

Consumes two input values and transforms them into a result.

**Trait**: `BiTransformer<T, U, R>`
**Core Method**: `apply(&self, first: T, second: U) -> R`
**Closure Equivalent**: `Fn(T, U) -> R`

**Implementations**:
- `BoxBiTransformer<T, U, R>` - Single ownership
- `ArcBiTransformer<T, U, R>` - Thread-safe
- `RcBiTransformer<T, U, R>` - Single-threaded

**Type Alias**: `BinaryOperator<T>` = `BiTransformer<T, T, T>`

**Example**:
```rust
use qubit_function::{BiTransformer, BoxBiTransformer};

let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
assert_eq!(add.apply(10, 20), 30);
```

### 19. StatefulBiTransformer - Stateful Two-Argument Value Transformer

Consumes two input values and transforms them into a result while
allowing mutable internal state.

**Trait**: `StatefulBiTransformer<T, U, R>`
**Core Method**: `apply(&mut self, first: T, second: U) -> R`
**Closure Equivalent**: `FnMut(T, U) -> R`

**Implementations**:
- `BoxStatefulBiTransformer<T, U, R>` - Single ownership
- `ArcStatefulBiTransformer<T, U, R>` - Thread-safe with parking_lot::Mutex
- `RcStatefulBiTransformer<T, U, R>` - Single-threaded with RefCell

### 20. BiTransformerOnce - Single-Use Two-Argument Value Transformer

Consumes two input values once and transforms them into a result.

**Trait**: `BiTransformerOnce<T, U, R>`
**Core Method**: `apply(self, first: T, second: U) -> R`
**Closure Equivalent**: `FnOnce(T, U) -> R`

**Implementations**:
- `BoxBiTransformerOnce<T, U, R>` - Single ownership, one-time use

**Type Alias**: `BinaryOperatorOnce<T>` = `BiTransformerOnce<T, T, T>`

### 21. StatefulConsumer - Stateful Consumer

Accepts a value reference and performs side effects while allowing
mutable internal state.

**Trait**: `StatefulConsumer<T>`
**Core Method**: `accept(&mut self, value: &T)`
**Closure Equivalent**: `FnMut(&T)`

**Implementations**:
- `BoxStatefulConsumer<T>` - Single ownership
- `ArcStatefulConsumer<T>` - Thread-safe with parking_lot::Mutex
- `RcStatefulConsumer<T>` - Single-threaded with RefCell

### 22. StatefulBiConsumer - Stateful Two-Argument Consumer

Accepts two value references and performs side effects while allowing
mutable internal state.

**Trait**: `StatefulBiConsumer<T, U>`
**Core Method**: `accept(&mut self, first: &T, second: &U)`
**Closure Equivalent**: `FnMut(&T, &U)`

**Implementations**:
- `BoxStatefulBiConsumer<T, U>` - Single ownership
- `ArcStatefulBiConsumer<T, U>` - Thread-safe with parking_lot::Mutex
- `RcStatefulBiConsumer<T, U>` - Single-threaded with RefCell

### 23. Comparator - Ordering Comparator

Compares two values and returns an `Ordering`.

**Trait**: `Comparator<T>`
**Core Method**: `compare(&self, a: &T, b: &T) -> Ordering`
**Closure Equivalent**: `Fn(&T, &T) -> Ordering`

**Implementations**:
- `BoxComparator<T>` - Single ownership
- `ArcComparator<T>` - Thread-safe
- `RcComparator<T>` - Single-threaded

**Example**:
```rust
use qubit_function::{Comparator, BoxComparator};
use std::cmp::Ordering;

let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
```

### 24. Tester - Zero-Argument Condition Checker

Checks whether a condition or state holds without taking input.

**Trait**: `Tester`
**Core Method**: `test(&self) -> bool`
**Closure Equivalent**: `Fn() -> bool`

**Implementations**:
- `BoxTester` - Single ownership
- `ArcTester` - Thread-safe
- `RcTester` - Single-threaded

**Example**:
```rust
use qubit_function::{Tester, BoxTester};
use std::sync::{Arc, atomic::{AtomicBool, Ordering}};

let flag = Arc::new(AtomicBool::new(true));
let flag_clone = flag.clone();
let tester = BoxTester::new(move || flag_clone.load(Ordering::Relaxed));

assert!(tester.test());
flag.store(false, Ordering::Relaxed);
assert!(!tester.test());
```

## Trait and Closure Correspondence Table

| Trait | Core Method Signature | Equivalent Closure Type |
|-------|----------------------|------------------------|
| `Predicate<T>` | `test(&self, value: &T) -> bool` | `Fn(&T) -> bool` |
| `BiPredicate<T, U>` | `test(&self, first: &T, second: &U) -> bool` | `Fn(&T, &U) -> bool` |
| `Consumer<T>` | `accept(&self, value: &T)` | `Fn(&T)` |
| `ConsumerOnce<T>` | `accept(self, value: &T)` | `FnOnce(&T)` |
| `StatefulConsumer<T>` | `accept(&mut self, value: &T)` | `FnMut(&T)` |
| `BiConsumer<T, U>` | `accept(&self, first: &T, second: &U)` | `Fn(&T, &U)` |
| `BiConsumerOnce<T, U>` | `accept(self, first: &T, second: &U)` | `FnOnce(&T, &U)` |
| `StatefulBiConsumer<T, U>` | `accept(&mut self, first: &T, second: &U)` | `FnMut(&T, &U)` |
| `Mutator<T>` | `apply(&self, value: &mut T)` | `Fn(&mut T)` |
| `MutatorOnce<T>` | `apply(self, value: &mut T)` | `FnOnce(&mut T)` |
| `Supplier<T>` | `get(&self) -> T` | `Fn() -> T` |
| `SupplierOnce<T>` | `get(self) -> T` | `FnOnce() -> T` |
| `StatefulSupplier<T>` | `get(&mut self) -> T` | `FnMut() -> T` |
| `Function<T, R>` | `apply(&self, input: &T) -> R` | `Fn(&T) -> R` |
| `FunctionOnce<T, R>` | `apply(self, input: &T) -> R` | `FnOnce(&T) -> R` |
| `StatefulFunction<T, R>` | `apply(&mut self, input: &T) -> R` | `FnMut(&T) -> R` |
| `Transformer<T, R>` | `apply(&self, input: T) -> R` | `Fn(T) -> R` |
| `TransformerOnce<T, R>` | `apply(self, input: T) -> R` | `FnOnce(T) -> R` |
| `StatefulTransformer<T, R>` | `apply(&mut self, input: T) -> R` | `FnMut(T) -> R` |
| `BiTransformer<T, U, R>` | `apply(&self, first: T, second: U) -> R` | `Fn(T, U) -> R` |
| `StatefulBiTransformer<T, U, R>` | `apply(&mut self, first: T, second: U) -> R` | `FnMut(T, U) -> R` |
| `BiTransformerOnce<T, U, R>` | `apply(self, first: T, second: U) -> R` | `FnOnce(T, U) -> R` |
| `Comparator<T>` | `compare(&self, a: &T, b: &T) -> Ordering` | `Fn(&T, &T) -> Ordering` |
| `Tester` | `test(&self) -> bool` | `Fn() -> bool` |

## Implementation Types Comparison

Each trait has multiple implementations based on ownership model:

| Trait | Box (Single) | Arc (Thread-Safe) | Rc (Single-Thread) |
|-------|--------------|-------------------|-------------------|
| Predicate | BoxPredicate | ArcPredicate | RcPredicate |
| BiPredicate | BoxBiPredicate | ArcBiPredicate | RcBiPredicate |
| Consumer | BoxConsumer | ArcConsumer | RcConsumer |
| ConsumerOnce | BoxConsumerOnce | - | - |
| StatefulConsumer | BoxStatefulConsumer | ArcStatefulConsumer | RcStatefulConsumer |
| BiConsumer | BoxBiConsumer | ArcBiConsumer | RcBiConsumer |
| BiConsumerOnce | BoxBiConsumerOnce | - | - |
| StatefulBiConsumer | BoxStatefulBiConsumer | ArcStatefulBiConsumer | RcStatefulBiConsumer |
| Mutator | BoxMutator | ArcMutator | RcMutator |
| MutatorOnce | BoxMutatorOnce | - | - |
| Supplier | BoxSupplier | ArcSupplier | RcSupplier |
| SupplierOnce | BoxSupplierOnce | - | - |
| StatefulSupplier | BoxStatefulSupplier | ArcStatefulSupplier | RcStatefulSupplier |
| Function | BoxFunction | ArcFunction | RcFunction |
| FunctionOnce | BoxFunctionOnce | - | - |
| StatefulFunction | BoxStatefulFunction | ArcStatefulFunction | RcStatefulFunction |
| Transformer | BoxTransformer | ArcTransformer | RcTransformer |
| TransformerOnce | BoxTransformerOnce | - | - |
| StatefulTransformer | BoxStatefulTransformer | ArcStatefulTransformer | RcStatefulTransformer |
| BiTransformer | BoxBiTransformer | ArcBiTransformer | RcBiTransformer |
| StatefulBiTransformer | BoxStatefulBiTransformer | ArcStatefulBiTransformer | RcStatefulBiTransformer |
| BiTransformerOnce | BoxBiTransformerOnce | - | - |
| Comparator | BoxComparator | ArcComparator | RcComparator |
| Tester | BoxTester | ArcTester | RcTester |

**Legend**:
- **Box**: Single ownership, cannot be cloned, consumes self
- **Arc**: Shared ownership, thread-safe, cloneable
- **Rc**: Shared ownership, single-threaded, cloneable
- **-**: Not applicable (Once types don't need sharing)

## Design Philosophy

This crate adopts the **Trait + Multiple Implementations** pattern:

1. **Unified Interface**: Each functional type has a trait defining core behavior
2. **Specialized Implementations**: Multiple concrete types optimized for different scenarios
3. **Type Preservation**: Composition methods return the same concrete type
4. **Ownership Flexibility**: Choose between single ownership, thread-safe sharing, or single-threaded sharing
5. **High-Performance Concurrency**: Uses parking_lot Mutex for superior synchronization performance
6. **Ergonomic API**: Natural method chaining and functional composition

## Examples

The `examples/` directory contains comprehensive demonstrations for each type. Run examples with:

```bash
cargo run --example predicate_demo
cargo run --example consumer_demo
cargo run --example transformer_demo
```

## Documentation

Detailed design documents are available in the `doc/` directory for each major abstraction.

## License

Licensed under Apache License, Version 2.0.

## Author

Haixing Hu <starfish.hu@gmail.com>
