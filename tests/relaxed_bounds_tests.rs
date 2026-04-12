use std::cell::RefCell;
use std::marker::PhantomData;
use std::rc::Rc;
use std::sync::{
    Arc,
    Mutex,
};

use qubit_function::{
    ArcConsumer,
    ArcMutator,
    ArcPredicate,
    ArcSupplier,
    ArcTransformer,
    BinaryOperator,
    BinaryOperatorOnce,
    BiConsumer,
    BiConsumerOnce,
    BiTransformer,
    BiTransformerOnce,
    BoxBiConsumer,
    BoxBiConsumerOnce,
    BoxConsumer,
    BoxConsumerOnce,
    BoxMutator,
    BoxMutatorOnce,
    BoxPredicate,
    BoxSupplier,
    BoxSupplierOnce,
    BoxTransformer,
    BoxTransformerOnce,
    Consumer,
    ConsumerOnce,
    FnBiFunctionOps,
    FnBiFunctionOnceOps,
    FnBiMutatingFunctionOps,
    FnBiMutatingFunctionOnceOps,
    FnBiPredicateOps,
    FnBiTransformerOnceOps,
    FnBiTransformerOps,
    FnPredicateOps,
    FnStatefulBiTransformerOps,
    FnStatefulSupplierOps,
    FnStatefulTransformerOps,
    FnTransformerOnceOps,
    FnTransformerOps,
    FnTesterOps,
    Mutator,
    MutatorOnce,
    Predicate,
    RcConsumer,
    StatefulFunction,
    Supplier,
    SupplierOnce,
    Tester,
    Transformer,
    TransformerOnce,
    UnaryOperator,
    UnaryOperatorOnce,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct Borrowed<'a> {
    value: &'a i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BorrowedUnaryOp;

impl<'a> Transformer<Borrowed<'a>, Borrowed<'a>> for BorrowedUnaryOp {
    fn apply(&self, input: Borrowed<'a>) -> Borrowed<'a> {
        input
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BorrowedBinaryOp;

impl<'a> BiTransformer<Borrowed<'a>, Borrowed<'a>, Borrowed<'a>> for BorrowedBinaryOp {
    fn apply(&self, first: Borrowed<'a>, _second: Borrowed<'a>) -> Borrowed<'a> {
        first
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BorrowedUnaryOpOnce;

impl<'a> TransformerOnce<Borrowed<'a>, Borrowed<'a>> for BorrowedUnaryOpOnce {
    fn apply(self, input: Borrowed<'a>) -> Borrowed<'a> {
        input
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
struct BorrowedBinaryOpOnce;

impl<'a> BiTransformerOnce<Borrowed<'a>, Borrowed<'a>, Borrowed<'a>> for BorrowedBinaryOpOnce {
    fn apply(self, first: Borrowed<'a>, _second: Borrowed<'a>) -> Borrowed<'a> {
        first
    }
}

#[test]
fn test_consumers_allow_non_static_generic_on_new() {
    let n = 7;
    let input = Borrowed { value: &n };

    let box_sum = Rc::new(RefCell::new(0));
    let rc_sum = Rc::new(RefCell::new(0));
    let arc_sum = Arc::new(Mutex::new(0));

    let box_consumer = BoxConsumer::new({
        let box_sum = Rc::clone(&box_sum);
        move |item: &Borrowed<'_>| {
            *box_sum.borrow_mut() += *item.value;
        }
    });
    box_consumer.accept(&input);

    let rc_consumer = RcConsumer::new({
        let rc_sum = Rc::clone(&rc_sum);
        move |item: &Borrowed<'_>| {
            *rc_sum.borrow_mut() += *item.value;
        }
    });
    rc_consumer.accept(&input);

    let arc_consumer = ArcConsumer::new({
        let arc_sum = Arc::clone(&arc_sum);
        move |item: &Borrowed<'_>| {
            *arc_sum.lock().expect("lock should succeed") += *item.value;
        }
    });
    arc_consumer.accept(&input);

    assert_eq!(*box_sum.borrow(), 7);
    assert_eq!(*rc_sum.borrow(), 7);
    assert_eq!(*arc_sum.lock().expect("lock should succeed"), 7);
}

#[test]
fn test_bi_consumer_allow_non_static_generic_on_new() {
    let n = 5;
    let input = Borrowed { value: &n };
    let sink = Rc::new(RefCell::new(String::new()));

    let bi_consumer = BoxBiConsumer::new({
        let sink = Rc::clone(&sink);
        move |prefix: &&str, item: &Borrowed<'_>| {
            *sink.borrow_mut() = format!("{}-{}", *prefix, item.value);
        }
    });

    bi_consumer.accept(&"ok", &input);
    assert_eq!(&*sink.borrow(), "ok-5");
}

#[test]
fn test_consumer_once_allow_non_static_generic_on_new() {
    let n = 3;
    let input = Borrowed { value: &n };
    let sink = Rc::new(RefCell::new(0));

    let consumer_once = BoxConsumerOnce::new({
        let sink = Rc::clone(&sink);
        move |item: &Borrowed<'_>| {
            *sink.borrow_mut() = *item.value;
        }
    });
    consumer_once.accept(&input);

    let bi_sink = Rc::new(RefCell::new(0));
    let bi_consumer_once = BoxBiConsumerOnce::new({
        let bi_sink = Rc::clone(&bi_sink);
        move |left: &Borrowed<'_>, right: &Borrowed<'_>| {
            *bi_sink.borrow_mut() = *left.value + *right.value;
        }
    });
    bi_consumer_once.accept(&input, &input);

    assert_eq!(*sink.borrow(), 3);
    assert_eq!(*bi_sink.borrow(), 6);
}

#[test]
fn test_mutators_allow_non_static_generic_on_new() {
    let n = 11;
    let mut slot = Some(&n);

    let box_mutator = BoxMutator::new(|value: &mut Option<&i32>| {
        *value = None;
    });
    box_mutator.apply(&mut slot);
    assert_eq!(slot, None);

    let arc_mutator = ArcMutator::new(|value: &mut Option<&i32>| {
        if value.is_none() {
            *value = Some(&42);
        }
    });
    arc_mutator.apply(&mut slot);
    assert_eq!(slot, Some(&42));
}

#[test]
fn test_mutator_once_allow_non_static_generic_on_new() {
    let n = 9;
    let mut slot = Some(&n);

    let mutator_once = BoxMutatorOnce::new(|value: &mut Option<&i32>| {
        *value = None;
    });
    mutator_once.apply(&mut slot);

    assert_eq!(slot, None);
}

#[test]
fn test_predicate_and_transformer_allow_non_static_generic_on_new() {
    let n = 13;
    let value = Borrowed { value: &n };

    let predicate = BoxPredicate::new(|item: &Borrowed<'_>| *item.value > 10);
    assert!(predicate.test(&value));

    let arc_predicate = ArcPredicate::new(|item: &Borrowed<'_>| *item.value % 2 == 1);
    assert!(arc_predicate.test(&value));

    let transformer = BoxTransformer::new(|item: Borrowed<'_>| *item.value + 1);
    assert_eq!(transformer.apply(value), 14);

    let arc_transformer = ArcTransformer::new(|item: Borrowed<'_>| *item.value - 1);
    assert_eq!(arc_transformer.apply(value), 12);
}

#[test]
fn test_transformer_once_allow_non_static_generic_on_new() {
    let n = 8;
    let value = Borrowed { value: &n };

    let transformer_once = BoxTransformerOnce::new(|item: Borrowed<'_>| *item.value * 2);
    assert_eq!(transformer_once.apply(value), 16);
}

#[test]
fn test_suppliers_allow_non_static_generic_on_new() {
    let n = 21;

    let box_supplier: BoxSupplier<PhantomData<&i32>> = make_box_supplier_with_lifetime(&n);
    let box_supplier_once: BoxSupplierOnce<PhantomData<&i32>> =
        make_box_supplier_once_with_lifetime(&n);
    let arc_supplier: ArcSupplier<PhantomData<&i32>> = make_arc_supplier_with_lifetime(&n);

    assert_eq!(box_supplier.get(), PhantomData);
    assert_eq!(box_supplier_once.get(), PhantomData);
    assert_eq!(arc_supplier.get(), PhantomData);
}

fn make_box_supplier_with_lifetime(_: &i32) -> BoxSupplier<PhantomData<&i32>> {
    BoxSupplier::new(|| PhantomData)
}

fn make_box_supplier_once_with_lifetime(_: &i32) -> BoxSupplierOnce<PhantomData<&i32>> {
    BoxSupplierOnce::new(|| PhantomData)
}

fn make_arc_supplier_with_lifetime(_: &i32) -> ArcSupplier<PhantomData<&i32>> {
    ArcSupplier::new(|| PhantomData)
}

#[test]
fn test_fn_ops_traits_allow_non_static_closure_implementations() {
    let a = 3;
    let b = 5;
    let a_ref = &a;
    let b_ref = &b;

    let bi_function = |x: &i32, y: &i32| x + y + a_ref;
    assert_fn_bi_function_ops_impl::<i32, i32, i32, _>(bi_function);

    let bi_mutating_function = |x: &mut i32, y: &mut i32| {
        *x += *a_ref;
        *y += *b_ref;
        *x + *y
    };
    assert_fn_bi_mutating_function_ops_impl::<i32, i32, i32, _>(bi_mutating_function);

    let predicate = |x: &i32| *x > *a_ref;
    assert_fn_predicate_ops_impl::<i32, _>(predicate);

    let bi_predicate = |x: &i32, y: &i32| *x + *y > *b_ref;
    assert_fn_bi_predicate_ops_impl::<i32, i32, _>(bi_predicate);

    let mut counter = 0;
    let stateful_supplier = || {
        counter += 1;
        counter + *a_ref
    };
    assert_fn_stateful_supplier_ops_impl::<i32, _>(stateful_supplier);

    let transformer = |x: i32| x + *a_ref;
    assert_fn_transformer_ops_impl::<i32, i32, _>(transformer);

    let bi_transformer = |x: i32, y: i32| x + y + *a_ref;
    assert_fn_bi_transformer_ops_impl::<i32, i32, i32, _>(bi_transformer);

    let mut offset = 0;
    let stateful_transformer = move |x: i32| {
        offset += 1;
        x + offset + *a_ref
    };
    assert_fn_stateful_transformer_ops_impl::<i32, i32, _>(stateful_transformer);

    let mut delta = 0;
    let stateful_bi_transformer = move |x: i32, y: i32| {
        delta += 1;
        x + y + delta + *b_ref
    };
    assert_fn_stateful_bi_transformer_ops_impl::<i32, i32, i32, _>(stateful_bi_transformer);

    let transformer_once = move |x: i32| x + *b_ref;
    assert_fn_transformer_once_ops_impl::<i32, i32, _>(transformer_once);

    let bi_transformer_once = move |x: i32, y: i32| x * y + *a_ref;
    assert_fn_bi_transformer_once_ops_impl::<i32, i32, i32, _>(bi_transformer_once);

    let bi_function_once = move |x: &i32, y: &i32| *x + *y + *a_ref;
    assert_fn_bi_function_once_ops_impl::<i32, i32, i32, _>(bi_function_once);

    let bi_mutating_function_once = move |x: &mut i32, y: &mut i32| {
        *x += *a_ref;
        *y += *b_ref;
        *x + *y
    };
    assert_fn_bi_mutating_function_once_ops_impl::<i32, i32, i32, _>(bi_mutating_function_once);

    let tester = || *a_ref < *b_ref;
    assert_fn_tester_ops_impl::<_>(tester);

    let stateful_function = |value: &Borrowed<'_>| *value.value;
    assert_stateful_function_impl(&a, stateful_function);

    let bi_transformer_with_borrow = |left: Borrowed<'_>, right: Borrowed<'_>| *left.value + *right.value;
    assert_bi_transformer_impl(&a, bi_transformer_with_borrow);

    assert_unary_operator_impl(&a, BorrowedUnaryOp);
    assert_binary_operator_impl(&a, BorrowedBinaryOp);
    assert_unary_operator_once_impl(&a, BorrowedUnaryOpOnce);
    assert_binary_operator_once_impl(&a, BorrowedBinaryOpOnce);
}

fn assert_fn_bi_function_ops_impl<T, U, R, F>(f: F)
where
    F: FnBiFunctionOps<T, U, R>,
{
    let _ = f;
}

fn assert_fn_bi_mutating_function_ops_impl<T, U, R, F>(f: F)
where
    F: FnBiMutatingFunctionOps<T, U, R>,
{
    let _ = f;
}

fn assert_fn_bi_function_once_ops_impl<T, U, R, F>(f: F)
where
    F: FnBiFunctionOnceOps<T, U, R>,
{
    let _ = f;
}

fn assert_fn_bi_mutating_function_once_ops_impl<T, U, R, F>(f: F)
where
    F: FnBiMutatingFunctionOnceOps<T, U, R>,
{
    let _ = f;
}

fn assert_fn_predicate_ops_impl<T, F>(f: F)
where
    F: FnPredicateOps<T>,
{
    let _ = f;
}

fn assert_fn_bi_predicate_ops_impl<T, U, F>(f: F)
where
    F: FnBiPredicateOps<T, U>,
{
    let _ = f;
}

fn assert_fn_stateful_supplier_ops_impl<T, F>(f: F)
where
    F: FnStatefulSupplierOps<T>,
{
    let _ = f;
}

fn assert_fn_transformer_ops_impl<T, R, F>(f: F)
where
    F: FnTransformerOps<T, R>,
{
    let _ = f;
}

fn assert_fn_bi_transformer_ops_impl<T, U, R, F>(f: F)
where
    F: FnBiTransformerOps<T, U, R>,
{
    let _ = f;
}

fn assert_fn_stateful_transformer_ops_impl<T, R, F>(f: F)
where
    F: FnStatefulTransformerOps<T, R>,
{
    let _ = f;
}

fn assert_fn_stateful_bi_transformer_ops_impl<T, U, R, F>(f: F)
where
    F: FnStatefulBiTransformerOps<T, U, R>,
{
    let _ = f;
}

fn assert_fn_transformer_once_ops_impl<T, R, F>(f: F)
where
    F: FnTransformerOnceOps<T, R>,
{
    let _ = f;
}

fn assert_fn_bi_transformer_once_ops_impl<T, U, R, F>(f: F)
where
    F: FnBiTransformerOnceOps<T, U, R>,
{
    let _ = f;
}

fn assert_fn_tester_ops_impl<F>(f: F)
where
    F: FnTesterOps + Tester,
{
    let _ = f;
}

fn assert_stateful_function_impl<'a, F>(_: &'a i32, f: F)
where
    F: StatefulFunction<Borrowed<'a>, i32>,
{
    let _ = f;
}

fn assert_bi_transformer_impl<'a, F>(_: &'a i32, f: F)
where
    F: BiTransformer<Borrowed<'a>, Borrowed<'a>, i32>,
{
    let _ = f;
}

fn assert_unary_operator_impl<'a, F>(_: &'a i32, f: F)
where
    F: UnaryOperator<Borrowed<'a>>,
{
    let _ = f;
}

fn assert_binary_operator_impl<'a, F>(_: &'a i32, f: F)
where
    F: BinaryOperator<Borrowed<'a>>,
{
    let _ = f;
}

fn assert_unary_operator_once_impl<'a, F>(_: &'a i32, f: F)
where
    F: UnaryOperatorOnce<Borrowed<'a>>,
{
    let _ = f;
}

fn assert_binary_operator_once_impl<'a, F>(_: &'a i32, f: F)
where
    F: BinaryOperatorOnce<Borrowed<'a>>,
{
    let _ = f;
}
