# Qubit Function

[![CircleCI](https://img.shields.io/circleci/build/github/qubit-ltd/rust-function/main?style=shield&logo=circleci)](https://circleci.com/gh/qubit-ltd/rust-function)
[![Coverage Status](https://coveralls.io/repos/github/qubit-ltd/rust-function/badge.svg?branch=main)](https://coveralls.io/github/qubit-ltd/rust-function?branch=main)
[![Crates.io](https://img.shields.io/crates/v/qubit-function.svg?color=blue)](https://crates.io/crates/qubit-function)
[![Rust](https://img.shields.io/badge/rust-1.70+-blue.svg?logo=rust)](https://www.rust-lang.org)
[![License](https://img.shields.io/badge/license-Apache%202.0-blue.svg)](LICENSE)
[![English Document](https://img.shields.io/badge/Document-English-blue.svg)](README.md)

为 Rust 提供全面的函数式编程抽象,提供与 Java 风格相近的函数式接口,并适配 Rust 的所有权模型。

## 概述

本 crate 为 Rust 提供一套完整的函数式编程抽象,灵感来自 Java 的函数式接口,并精心适配 Rust 的所有权系统。它为每种抽象提供多种实现(Box/Arc/Rc),涵盖从简单的单线程场景到复杂的多线程应用的各种使用场景。

## 核心特性

- **完整的函数式接口套件**: 24 种核心函数式抽象及其多种变体
- **高性能并发**: 使用 parking_lot Mutex 提供卓越的线程同步性能
- **多种所有权模型**: 基于 Box 的单一所有权、基于 Arc 的线程安全共享、基于 Rc 的单线程共享
- **灵活的 API 设计**: 基于 trait 的统一接口,针对不同场景优化的具体实现
- **方法链式调用**: 所有类型都支持流畅 API(链式调用)和函数组合
- **线程安全选项**: 在线程安全(Arc)和高效单线程(Rc)实现之间选择
- **零成本抽象**: 高效的实现,最小的运行时开销

## 安装

在 `Cargo.toml` 中添加:

```toml
[dependencies]
qubit-function = "0.7.1"
```

## 核心抽象

本 crate 提供 24 种核心函数式抽象,每种都有多个实现:

### 1. Predicate - 单参数谓词

判断一个值是否满足条件,返回 `bool`。

**Trait**: `Predicate<T>`
**核心方法**: `test(&self, value: &T) -> bool`
**等价闭包**: `Fn(&T) -> bool`

**实现类型**:
- `BoxPredicate<T>` - 单一所有权,不可克隆
- `ArcPredicate<T>` - 线程安全,可克隆
- `RcPredicate<T>` - 单线程,可克隆

**示例**:
```rust
use qubit_function::{Predicate, ArcPredicate};

let is_even = ArcPredicate::new(|x: &i32| x % 2 == 0);
let is_positive = ArcPredicate::new(|x: &i32| *x > 0);

let combined = is_even.and(is_positive.clone());
assert!(combined.test(&4));
assert!(!combined.test(&-2));
```

### 2. BiPredicate - 双参数谓词

判断两个值是否满足条件,返回 `bool`。

**Trait**: `BiPredicate<T, U>`
**核心方法**: `test(&self, first: &T, second: &U) -> bool`
**等价闭包**: `Fn(&T, &U) -> bool`

**实现类型**:
- `BoxBiPredicate<T, U>` - 单一所有权
- `ArcBiPredicate<T, U>` - 线程安全
- `RcBiPredicate<T, U>` - 单线程

**示例**:
```rust
use qubit_function::{BiPredicate, BoxBiPredicate};

let sum_positive = BoxBiPredicate::new(|x: &i32, y: &i32| x + y > 0);
assert!(sum_positive.test(&3, &4));
assert!(!sum_positive.test(&-5, &2));
```

### 3. Consumer - 只读消费者

接受值引用并执行带副作用的操作,不返回结果。

**Trait**: `Consumer<T>`
**核心方法**: `accept(&self, value: &T)`
**等价闭包**: `Fn(&T)`

**实现类型**:
- `BoxConsumer<T>` - 单一所有权
- `ArcConsumer<T>` - 线程安全
- `RcConsumer<T>` - 单线程

**示例**:
```rust
use qubit_function::{Consumer, BoxConsumer};

let logger = BoxConsumer::new(|x: &i32| {
    println!("值: {}", x);
});
logger.accept(&42);
```

### 4. ConsumerOnce - 一次性只读消费者

接受值引用并执行一次带副作用的操作。

**Trait**: `ConsumerOnce<T>`
**核心方法**: `accept(self, value: &T)`
**等价闭包**: `FnOnce(&T)`

**实现类型**:
- `BoxConsumerOnce<T>` - 单一所有权,一次性使用

### 5. BiConsumer - 双参数只读消费者

接受两个值引用并执行带副作用的操作,不返回结果。

**Trait**: `BiConsumer<T, U>`
**核心方法**: `accept(&self, first: &T, second: &U)`
**等价闭包**: `Fn(&T, &U)`

**实现类型**:
- `BoxBiConsumer<T, U>` - 单一所有权
- `ArcBiConsumer<T, U>` - 线程安全
- `RcBiConsumer<T, U>` - 单线程

**示例**:
```rust
use qubit_function::{BiConsumer, BoxBiConsumer};

let sum_logger = BoxBiConsumer::new(|x: &i32, y: &i32| {
    println!("和: {}", x + y);
});
sum_logger.accept(&10, &20);
```

### 6. BiConsumerOnce - 一次性双参数只读消费者

接受两个值引用并执行一次带副作用的操作。

**Trait**: `BiConsumerOnce<T, U>`
**核心方法**: `accept(self, first: &T, second: &U)`
**等价闭包**: `FnOnce(&T, &U)`

**实现类型**:
- `BoxBiConsumerOnce<T, U>` - 单一所有权,一次性使用

### 7. Mutator - 无状态原地修改器

通过可变引用**原地**修改目标值,无返回值; 修改器自身无状态,
以 `&self` 调用(对应 `Fn(&mut T)`)。

**Trait**: `Mutator<T>`
**核心方法**: `apply(&self, value: &mut T)`
**等价闭包**: `Fn(&mut T)`

**实现类型**:
- `BoxMutator<T>` - 单一所有权
- `ArcMutator<T>` - 线程安全
- `RcMutator<T>` - 单线程

**示例**:
```rust
use qubit_function::{Mutator, BoxMutator};

let mut doubler = BoxMutator::new(|x: &mut i32| *x *= 2);
let mut value = 10;
doubler.apply(&mut value);
assert_eq!(value, 20);
```

### 8. MutatorOnce - 一次性原地修改器

仅可调用一次,通过可变引用原地修改目标值(对应 `FnOnce(&mut T)`)。

**Trait**: `MutatorOnce<T>`
**核心方法**: `apply(self, value: &mut T)`
**等价闭包**: `FnOnce(&mut T)`

**实现类型**:
- `BoxMutatorOnce<T>` - 单一所有权,一次性使用

### 9. Supplier - 无状态值提供者

无参数,每次调用 `get` 都返回一个 `T`; 值提供者自身无状态,以
`&self` 调用(对应 `Fn() -> T`)。

**Trait**: `Supplier<T>`
**核心方法**: `get(&self) -> T`
**等价闭包**: `Fn() -> T`

**实现类型**:
- `BoxSupplier<T>` - 单一所有权,无锁
- `ArcSupplier<T>` - 线程安全,无锁
- `RcSupplier<T>` - 单线程

**示例**:
```rust
use qubit_function::{Supplier, BoxSupplier};

let factory = BoxSupplier::new(|| String::from("你好"));
assert_eq!(factory.get(), "你好");
```

### 10. SupplierOnce - 一次性值提供者

无参数,仅能调用一次 `get` 以返回一个 `T`(对应 `FnOnce() -> T`)。

**Trait**: `SupplierOnce<T>`
**核心方法**: `get(self) -> T`
**等价闭包**: `FnOnce() -> T`

**实现类型**:
- `BoxSupplierOnce<T>` - 单一所有权,一次性使用

### 11. StatefulSupplier - 有状态值提供者

在可变内部状态下返回 `T`; 多次 `get` 的结果可以不同(对应
`FnMut() -> T`)。

**Trait**: `StatefulSupplier<T>`
**核心方法**: `get(&mut self) -> T`
**等价闭包**: `FnMut() -> T`

**实现类型**:
- `BoxStatefulSupplier<T>` - 单一所有权
- `ArcStatefulSupplier<T>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulSupplier<T>` - 单线程(使用 RefCell)

**示例**:
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

### 12. Function - 借用输入函数

基于借用输入计算结果,不消耗输入。

**Trait**: `Function<T, R>`
**核心方法**: `apply(&self, input: &T) -> R`
**等价闭包**: `Fn(&T) -> R`

**实现类型**:
- `BoxFunction<T, R>` - 单一所有权
- `ArcFunction<T, R>` - 线程安全
- `RcFunction<T, R>` - 单线程

**示例**:
```rust
use qubit_function::{Function, BoxFunction};

let to_string = BoxFunction::new(|x: &i32| format!("值: {}", x));
assert_eq!(to_string.apply(&42), "值: 42");
```

### 13. FunctionOnce - 一次性借用输入函数

基于借用输入计算一次结果。

**Trait**: `FunctionOnce<T, R>`
**核心方法**: `apply(self, input: &T) -> R`
**等价闭包**: `FnOnce(&T) -> R`

**实现类型**:
- `BoxFunctionOnce<T, R>` - 单一所有权,一次性使用

### 14. StatefulFunction - 有状态借用输入函数

基于借用输入计算结果,并允许修改内部状态。

**Trait**: `StatefulFunction<T, R>`
**核心方法**: `apply(&mut self, input: &T) -> R`
**等价闭包**: `FnMut(&T) -> R`

**实现类型**:
- `BoxStatefulFunction<T, R>` - 单一所有权
- `ArcStatefulFunction<T, R>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulFunction<T, R>` - 单线程(使用 RefCell)

### 15. Transformer - 值转换器

取得输入值的所有权,并将类型 `T` 的值转换为类型 `R` 的值。

**Trait**: `Transformer<T, R>`
**核心方法**: `apply(&self, input: T) -> R`
**等价闭包**: `Fn(T) -> R`

**实现类型**:
- `BoxTransformer<T, R>` - 单一所有权
- `ArcTransformer<T, R>` - 线程安全
- `RcTransformer<T, R>` - 单线程

**类型别名**: `UnaryOperator<T>` = `Transformer<T, T>`

**示例**:
```rust
use qubit_function::{Transformer, BoxTransformer};

let parse = BoxTransformer::new(|s: String| s.parse::<i32>().unwrap_or(0));
assert_eq!(parse.apply("42".to_string()), 42);
```

### 16. TransformerOnce - 一次性值转换器

一次性取得输入值的所有权,并将其转换为类型 `R` 的值。

**Trait**: `TransformerOnce<T, R>`
**核心方法**: `apply(self, input: T) -> R`
**等价闭包**: `FnOnce(T) -> R`

**实现类型**:
- `BoxTransformerOnce<T, R>` - 单一所有权,一次性使用

**类型别名**: `UnaryOperatorOnce<T>` = `TransformerOnce<T, T>`

### 17. StatefulTransformer - 有状态值转换器

取得输入值的所有权并完成转换,同时允许修改内部状态。

**Trait**: `StatefulTransformer<T, R>`
**核心方法**: `apply(&mut self, input: T) -> R`
**等价闭包**: `FnMut(T) -> R`

**实现类型**:
- `BoxStatefulTransformer<T, R>` - 单一所有权
- `ArcStatefulTransformer<T, R>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulTransformer<T, R>` - 单线程(使用 RefCell)

### 18. BiTransformer - 双参数值转换器

取得两个输入值的所有权,并将其转换为结果。

**Trait**: `BiTransformer<T, U, R>`
**核心方法**: `apply(&self, first: T, second: U) -> R`
**等价闭包**: `Fn(T, U) -> R`

**实现类型**:
- `BoxBiTransformer<T, U, R>` - 单一所有权
- `ArcBiTransformer<T, U, R>` - 线程安全
- `RcBiTransformer<T, U, R>` - 单线程

**类型别名**: `BinaryOperator<T>` = `BiTransformer<T, T, T>`

**示例**:
```rust
use qubit_function::{BiTransformer, BoxBiTransformer};

let add = BoxBiTransformer::new(|x: i32, y: i32| x + y);
assert_eq!(add.apply(10, 20), 30);
```

### 19. StatefulBiTransformer - 有状态双参数值转换器

取得两个输入值的所有权并完成转换,同时允许修改内部状态。

**Trait**: `StatefulBiTransformer<T, U, R>`
**核心方法**: `apply(&mut self, first: T, second: U) -> R`
**等价闭包**: `FnMut(T, U) -> R`

**实现类型**:
- `BoxStatefulBiTransformer<T, U, R>` - 单一所有权
- `ArcStatefulBiTransformer<T, U, R>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulBiTransformer<T, U, R>` - 单线程(使用 RefCell)

### 20. BiTransformerOnce - 一次性双参数值转换器

一次性取得两个输入值的所有权,并将其转换为结果。

**Trait**: `BiTransformerOnce<T, U, R>`
**核心方法**: `apply(self, first: T, second: U) -> R`
**等价闭包**: `FnOnce(T, U) -> R`

**实现类型**:
- `BoxBiTransformerOnce<T, U, R>` - 单一所有权,一次性使用

**类型别名**: `BinaryOperatorOnce<T>` = `BiTransformerOnce<T, T, T>`

### 21. StatefulConsumer - 有状态消费者

接受值引用并执行带副作用的操作,同时允许修改内部状态。

**Trait**: `StatefulConsumer<T>`
**核心方法**: `accept(&mut self, value: &T)`
**等价闭包**: `FnMut(&T)`

**实现类型**:
- `BoxStatefulConsumer<T>` - 单一所有权
- `ArcStatefulConsumer<T>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulConsumer<T>` - 单线程(使用 RefCell)

### 22. StatefulBiConsumer - 有状态双参数消费者

接受两个值引用并执行带副作用的操作,同时允许修改内部状态。

**Trait**: `StatefulBiConsumer<T, U>`
**核心方法**: `accept(&mut self, first: &T, second: &U)`
**等价闭包**: `FnMut(&T, &U)`

**实现类型**:
- `BoxStatefulBiConsumer<T, U>` - 单一所有权
- `ArcStatefulBiConsumer<T, U>` - 线程安全(使用 parking_lot::Mutex)
- `RcStatefulBiConsumer<T, U>` - 单线程(使用 RefCell)

### 23. Comparator - 排序比较器

比较两个值并返回 `Ordering`。

**Trait**: `Comparator<T>`
**核心方法**: `compare(&self, a: &T, b: &T) -> Ordering`
**等价闭包**: `Fn(&T, &T) -> Ordering`

**实现类型**:
- `BoxComparator<T>` - 单一所有权
- `ArcComparator<T>` - 线程安全
- `RcComparator<T>` - 单线程

**示例**:
```rust
use qubit_function::{Comparator, BoxComparator};
use std::cmp::Ordering;

let cmp = BoxComparator::new(|a: &i32, b: &i32| a.cmp(b));
assert_eq!(cmp.compare(&5, &3), Ordering::Greater);
```

### 24. Tester - 无参条件判定器

在不接收参数的前提下,判断某一状态或条件是否成立。

**Trait**: `Tester`
**核心方法**: `test(&self) -> bool`
**等价闭包**: `Fn() -> bool`

**实现类型**:
- `BoxTester` - 单一所有权
- `ArcTester` - 线程安全
- `RcTester` - 单线程

**示例**:
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

## Trait 与闭包对应表

| Trait | 核心方法签名 | 等价闭包类型 |
|-------|------------|-------------|
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

## 实现类型对比

每个 trait 基于所有权模型都有多种实现:

| Trait | Box(单一所有权) | Arc(线程安全) | Rc(单线程) |
|-------|----------------|--------------|-----------|
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

**图例**:
- **Box**: 单一所有权,不可克隆,消耗 self
- **Arc**: 共享所有权,线程安全,可克隆
- **Rc**: 共享所有权,单线程,可克隆
- **-**: 不适用(Once 类型不需要共享)

## 设计理念

本 crate 采用 **Trait + 多实现** 模式:

1. **统一接口**: 每个函数式类型都有一个定义核心行为的 trait
2. **专门实现**: 针对不同场景优化的多个具体类型
3. **类型保持**: 组合方法返回相同的具体类型
4. **所有权灵活性**: 在单一所有权、线程安全共享或单线程共享之间选择
5. **高性能并发**: 使用 parking_lot Mutex 提供卓越的同步性能
6. **易用 API**: 自然的方法链式调用和函数组合

## 示例

`examples/` 目录包含每种类型的全面演示。运行示例:

```bash
cargo run --example predicate_demo
cargo run --example consumer_demo
cargo run --example transformer_demo
```

## 文档

`doc/` 目录中提供了每个主要抽象的详细设计文档。

## 许可证

采用 Apache License, Version 2.0 许可证。

## 作者

胡海星 <starfish.hu@gmail.com>
