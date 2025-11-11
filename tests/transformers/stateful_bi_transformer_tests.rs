/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use prism3_function::{
    ArcStatefulBiTransformer,
    ArcStatefulTransformer,
    BoxBiPredicate,
    BoxStatefulBiTransformer,
    BoxStatefulTransformer,
    FnStatefulBiTransformerOps,
    RcStatefulBiTransformer,
    RcStatefulTransformer,
    StatefulBiTransformer,
};

// ============================================================================
// BoxStatefulBiTransformer Tests
// ============================================================================

#[test]
fn test_box_stateful_bi_transformer_new() {
    // Test basic creation and usage with stateful transformation
    let mut counter = 0;
    let mut transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    assert_eq!(transformer.apply(10, 20), 31); // 10 + 20 + 1
    assert_eq!(transformer.apply(10, 20), 32); // 10 + 20 + 2
    assert_eq!(transformer.apply(10, 20), 33); // 10 + 20 + 3
}

#[test]
fn test_box_stateful_bi_transformer_constant() {
    // Test constant bi-transformer that ignores inputs
    let mut constant = BoxStatefulBiTransformer::constant("hello");
    assert_eq!(constant.apply(1, 2), "hello");
    assert_eq!(constant.apply(3, 4), "hello");
    assert_eq!(constant.apply(5, 6), "hello");
}

#[test]
fn test_box_stateful_bi_transformer_and_then() {
    // Test composition with and_then method
    let mut counter1 = 0;
    let bi_trans = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter1 += 1;
        x + y + counter1
    });

    let mut counter2 = 0;
    let trans = BoxStatefulTransformer::new(move |x: i32| {
        counter2 += 1;
        x * counter2
    });

    let mut composed = bi_trans.and_then(trans);
    assert_eq!(composed.apply(10, 20), 31); // (10 + 20 + 1) * 1
    assert_eq!(composed.apply(10, 20), 64); // (10 + 20 + 2) * 2
    assert_eq!(composed.apply(10, 20), 99); // (10 + 20 + 3) * 3
}

#[test]
fn test_box_stateful_bi_transformer_and_then_with_closure() {
    // Test and_then with a closure
    let mut counter = 0;
    let bi_trans = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut composed = bi_trans.and_then(|x: i32| x * 2);
    assert_eq!(composed.apply(5, 5), 22); // (5 + 5 + 1) * 2
    assert_eq!(composed.apply(5, 5), 24); // (5 + 5 + 2) * 2
}

#[test]
fn test_box_stateful_bi_transformer_when_or_else() {
    // Test conditional execution with when and or_else
    let mut then_count = 0;
    let mut else_count = 0;

    let mut transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        then_count += 1;
        format!("Then[{}]: {}", then_count, x + y)
    })
    .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
    .or_else(move |x, y| {
        else_count += 1;
        format!("Else[{}]: {}", else_count, x * y)
    });

    assert_eq!(transformer.apply(5, 3), "Then[1]: 8");
    assert_eq!(transformer.apply(-5, 3), "Else[1]: -15");
    assert_eq!(transformer.apply(10, 2), "Then[2]: 12");
    assert_eq!(transformer.apply(0, 5), "Else[2]: 0");
}

#[test]
fn test_box_stateful_bi_transformer_when_with_predicate() {
    // Test when with a predicate object
    let predicate = BoxBiPredicate::new(|x: &i32, y: &i32| *x >= 10 && *y >= 10);

    let mut transformer = BoxStatefulBiTransformer::new(|x: i32, y: i32| x + y)
        .when(predicate)
        .or_else(|x, y| x * y);

    assert_eq!(transformer.apply(15, 20), 35); // both >= 10, add
    assert_eq!(transformer.apply(5, 20), 100); // not both >= 10, multiply
}

#[test]
fn test_box_stateful_bi_transformer_into_box() {
    // Test into_box conversion (zero-cost)
    let mut counter = 0;
    let transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut boxed = StatefulBiTransformer::into_box(transformer);
    assert_eq!(boxed.apply(10, 20), 31);
    assert_eq!(boxed.apply(10, 20), 32);
}

#[test]
fn test_box_stateful_bi_transformer_into_rc() {
    // Test into_rc conversion
    let mut counter = 0;
    let transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut rc_transformer = transformer.into_rc();
    assert_eq!(rc_transformer.apply(10, 20), 31);
    assert_eq!(rc_transformer.apply(10, 20), 32);
}

#[test]
fn test_box_stateful_bi_transformer_into_fn() {
    // Test into_fn conversion
    let mut counter = 0;
    let transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x * y * counter
    });

    let mut closure = StatefulBiTransformer::into_fn(transformer);
    assert_eq!(closure(10, 2), 20); // 10 * 2 * 1
    assert_eq!(closure(10, 2), 40); // 10 * 2 * 2
    assert_eq!(closure(10, 2), 60); // 10 * 2 * 3
}

#[test]
fn test_box_stateful_bi_transformer_with_string_types() {
    // Test with string input and output types
    let mut count = 0;
    let mut transformer = BoxStatefulBiTransformer::new(move |s1: String, s2: String| {
        count += 1;
        format!("[{}] {}{}", count, s1, s2)
    });

    assert_eq!(
        transformer.apply("hello".to_string(), "world".to_string()),
        "[1] helloworld"
    );
    assert_eq!(
        transformer.apply("foo".to_string(), "bar".to_string()),
        "[2] foobar"
    );
}

#[test]
fn test_box_stateful_bi_transformer_different_types() {
    // Test with different input and output types
    let mut counter = 0;
    let mut transformer = BoxStatefulBiTransformer::new(move |name: String, age: i32| {
        counter += 1;
        format!("#{} {} is {}", counter, name, age)
    });

    assert_eq!(transformer.apply("Alice".to_string(), 30), "#1 Alice is 30");
    assert_eq!(transformer.apply("Bob".to_string(), 25), "#2 Bob is 25");
}

#[test]
fn test_box_stateful_bi_transformer_accumulation() {
    // Test stateful accumulation
    let mut sum = 0;
    let mut transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        sum += x + y;
        sum
    });

    assert_eq!(transformer.apply(10, 20), 30);
    assert_eq!(transformer.apply(5, 5), 40);
    assert_eq!(transformer.apply(3, 7), 50);
}

#[test]
fn test_box_stateful_bi_transformer_complex_state() {
    // Test with complex internal state
    let mut history = Vec::new();
    let mut transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        let sum = x + y;
        history.push(sum);
        (sum, history.len())
    });

    assert_eq!(transformer.apply(10, 20), (30, 1));
    assert_eq!(transformer.apply(5, 5), (10, 2));
    assert_eq!(transformer.apply(3, 7), (10, 3));
}

// ============================================================================
// ArcStatefulBiTransformer Tests
// ============================================================================

#[test]
fn test_arc_stateful_bi_transformer_new() {
    // Test basic creation and usage
    let mut counter = 0;
    let mut transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    assert_eq!(transformer.apply(10, 20), 31);
    assert_eq!(transformer.apply(10, 20), 32);
    assert_eq!(transformer.apply(10, 20), 33);
}

#[test]
fn test_arc_stateful_bi_transformer_constant() {
    // Test constant bi-transformer
    let mut constant = ArcStatefulBiTransformer::constant("hello");
    assert_eq!(constant.apply(1, 2), "hello");
    assert_eq!(constant.apply(3, 4), "hello");
}

#[test]
fn test_arc_stateful_bi_transformer_clone() {
    // Test cloning and shared state
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut t1 = transformer.clone();
    let mut t2 = transformer.clone();

    assert_eq!(t1.apply(10, 20), 31); // counter = 1
    assert_eq!(t2.apply(10, 20), 32); // counter = 2 (shared state)
    assert_eq!(t1.apply(10, 20), 33); // counter = 3 (shared state)
}

#[test]
fn test_arc_stateful_bi_transformer_and_then() {
    // Test composition with and_then
    let mut counter1 = 0;
    let bi_trans = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter1 += 1;
        x + y + counter1
    });

    let mut counter2 = 0;
    let trans = ArcStatefulTransformer::new(move |x: i32| {
        counter2 += 1;
        x * counter2
    });

    let mut composed = bi_trans.and_then(trans);
    assert_eq!(composed.apply(10, 20), 31); // (10 + 20 + 1) * 1
    assert_eq!(composed.apply(10, 20), 64); // (10 + 20 + 2) * 2
}

#[test]
fn test_arc_stateful_bi_transformer_and_then_preserves_original() {
    // Test that and_then uses &self and preserves original
    let mut counter = 0;
    let bi_trans = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let trans = ArcStatefulTransformer::new(|x: i32| x * 2);
    let mut composed = bi_trans.and_then(trans);

    assert_eq!(composed.apply(5, 5), 22); // (5 + 5 + 1) * 2

    // Original bi_trans still usable
    let mut original = bi_trans.clone();
    assert_eq!(original.apply(10, 20), 32); // 10 + 20 + 2 (state continues)
}

#[test]
fn test_arc_stateful_bi_transformer_when_or_else() {
    // Test conditional execution
    let mut then_count = 0;
    let mut else_count = 0;

    let mut transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        then_count += 1;
        format!("Then[{}]: {}", then_count, x + y)
    })
    .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
    .or_else(move |x, y| {
        else_count += 1;
        format!("Else[{}]: {}", else_count, x * y)
    });

    assert_eq!(transformer.apply(5, 3), "Then[1]: 8");
    assert_eq!(transformer.apply(-5, 3), "Else[1]: -15");
}

#[test]
fn test_arc_stateful_bi_transformer_when_preserves_original() {
    // Test that when uses &self and preserves original
    let transformer = ArcStatefulBiTransformer::new(|x: i32, y: i32| x + y);
    let conditional = transformer.when(|x: &i32, _y: &i32| *x > 0);

    let mut result = conditional.or_else(|x, y| x * y);
    assert_eq!(result.apply(5, 3), 8);

    // Original transformer still usable
    let mut original = transformer.clone();
    assert_eq!(original.apply(10, 20), 30);
}

#[test]
fn test_arc_stateful_bi_transformer_into_box() {
    // Test into_box conversion
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut boxed = StatefulBiTransformer::into_box(transformer);
    assert_eq!(boxed.apply(10, 20), 31);
    assert_eq!(boxed.apply(10, 20), 32);
}

#[test]
fn test_arc_stateful_bi_transformer_into_rc() {
    // Test into_rc conversion
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut rc_transformer = transformer.into_rc();
    assert_eq!(rc_transformer.apply(10, 20), 31);
    assert_eq!(rc_transformer.apply(10, 20), 32);
}

#[test]
fn test_arc_stateful_bi_transformer_into_arc() {
    // Test into_arc conversion (zero-cost)
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut arc_transformer = transformer.into_arc();
    assert_eq!(arc_transformer.apply(10, 20), 31);
    assert_eq!(arc_transformer.apply(10, 20), 32);
}

#[test]
fn test_arc_stateful_bi_transformer_into_fn() {
    // Test into_fn conversion
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x * y * counter
    });

    let mut closure = StatefulBiTransformer::into_fn(transformer);
    assert_eq!(closure(10, 2), 20); // 10 * 2 * 1
    assert_eq!(closure(10, 2), 40); // 10 * 2 * 2
}

#[test]
fn test_arc_stateful_bi_transformer_to_arc() {
    // Test non-consuming to_arc conversion
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let arc1 = transformer.to_arc();
    let mut a1 = arc1.clone();
    assert_eq!(a1.apply(10, 20), 31);

    let arc2 = transformer.to_arc();
    let mut a2 = arc2.clone();
    assert_eq!(a2.apply(10, 20), 32); // shared state

    // Original still usable
    let mut original = transformer.clone();
    assert_eq!(original.apply(10, 20), 33);
}

#[test]
fn test_arc_stateful_bi_transformer_thread_safe() {
    // Test thread safety
    use std::thread;

    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let t1 = transformer.clone();
    let t2 = transformer.clone();

    let handle1 = thread::spawn(move || {
        let mut t = t1.clone();
        t.apply(10, 20)
    });

    let handle2 = thread::spawn(move || {
        let mut t = t2.clone();
        t.apply(5, 5)
    });

    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();

    // Results depend on execution order, but both should be valid
    assert!((31..=32).contains(&result1));
    assert!((11..=12).contains(&result2));
}

// ============================================================================
// RcStatefulBiTransformer Tests
// ============================================================================

#[test]
fn test_rc_stateful_bi_transformer_new() {
    // Test basic creation and usage
    let mut counter = 0;
    let mut transformer = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    assert_eq!(transformer.apply(10, 20), 31);
    assert_eq!(transformer.apply(10, 20), 32);
    assert_eq!(transformer.apply(10, 20), 33);
}

#[test]
fn test_rc_stateful_bi_transformer_constant() {
    // Test constant bi-transformer
    let mut constant = RcStatefulBiTransformer::constant("hello");
    assert_eq!(constant.apply(1, 2), "hello");
    assert_eq!(constant.apply(3, 4), "hello");
}

#[test]
fn test_rc_stateful_bi_transformer_clone() {
    // Test cloning and shared state
    let mut counter = 0;
    let transformer = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut t1 = transformer.clone();
    let mut t2 = transformer.clone();

    assert_eq!(t1.apply(10, 20), 31); // counter = 1
    assert_eq!(t2.apply(10, 20), 32); // counter = 2 (shared state)
    assert_eq!(t1.apply(10, 20), 33); // counter = 3 (shared state)
}

#[test]
fn test_rc_stateful_bi_transformer_and_then() {
    // Test composition with and_then
    let mut counter1 = 0;
    let bi_trans = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter1 += 1;
        x + y + counter1
    });

    let mut counter2 = 0;
    let trans = RcStatefulTransformer::new(move |x: i32| {
        counter2 += 1;
        x * counter2
    });

    let mut composed = bi_trans.and_then(trans);
    assert_eq!(composed.apply(10, 20), 31); // (10 + 20 + 1) * 1
    assert_eq!(composed.apply(10, 20), 64); // (10 + 20 + 2) * 2
}

#[test]
fn test_rc_stateful_bi_transformer_and_then_preserves_original() {
    // Test that and_then uses &self and preserves original
    let mut counter = 0;
    let bi_trans = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let trans = RcStatefulTransformer::new(|x: i32| x * 2);
    let mut composed = bi_trans.and_then(trans);

    assert_eq!(composed.apply(5, 5), 22); // (5 + 5 + 1) * 2

    // Original bi_trans still usable
    let mut original = bi_trans.clone();
    assert_eq!(original.apply(10, 20), 32); // 10 + 20 + 2 (state continues)
}

#[test]
fn test_rc_stateful_bi_transformer_when_or_else() {
    // Test conditional execution
    let mut then_count = 0;
    let mut else_count = 0;

    let mut transformer = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        then_count += 1;
        format!("Then[{}]: {}", then_count, x + y)
    })
    .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
    .or_else(move |x, y| {
        else_count += 1;
        format!("Else[{}]: {}", else_count, x * y)
    });

    assert_eq!(transformer.apply(5, 3), "Then[1]: 8");
    assert_eq!(transformer.apply(-5, 3), "Else[1]: -15");
}

#[test]
fn test_rc_stateful_bi_transformer_when_preserves_original() {
    // Test that when uses &self and preserves original
    let transformer = RcStatefulBiTransformer::new(|x: i32, y: i32| x + y);
    let conditional = transformer.when(|x: &i32, _y: &i32| *x > 0);

    let mut result = conditional.or_else(|x, y| x * y);
    assert_eq!(result.apply(5, 3), 8);

    // Original transformer still usable
    let mut original = transformer.clone();
    assert_eq!(original.apply(10, 20), 30);
}

#[test]
fn test_rc_stateful_bi_transformer_into_box() {
    // Test into_box conversion
    let mut counter = 0;
    let transformer = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut boxed = StatefulBiTransformer::into_box(transformer);
    assert_eq!(boxed.apply(10, 20), 31);
    assert_eq!(boxed.apply(10, 20), 32);
}

#[test]
fn test_rc_stateful_bi_transformer_into_rc() {
    // Test into_rc conversion (zero-cost)
    let mut counter = 0;
    let transformer = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let mut rc_transformer = transformer.into_rc();
    assert_eq!(rc_transformer.apply(10, 20), 31);
    assert_eq!(rc_transformer.apply(10, 20), 32);
}

#[test]
fn test_rc_stateful_bi_transformer_into_fn() {
    // Test into_fn conversion
    let mut counter = 0;
    let transformer = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x * y * counter
    });

    let mut closure = StatefulBiTransformer::into_fn(transformer);
    assert_eq!(closure(10, 2), 20); // 10 * 2 * 1
    assert_eq!(closure(10, 2), 40); // 10 * 2 * 2
}

#[test]
fn test_rc_stateful_bi_transformer_to_rc() {
    // Test non-consuming to_rc conversion
    let mut counter = 0;
    let transformer = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    let rc1 = transformer.to_rc();
    let mut r1 = rc1.clone();
    assert_eq!(r1.apply(10, 20), 31);

    let rc2 = transformer.to_rc();
    let mut r2 = rc2.clone();
    assert_eq!(r2.apply(10, 20), 32); // shared state

    // Original still usable
    let mut original = transformer.clone();
    assert_eq!(original.apply(10, 20), 33);
}

// ============================================================================
// Closure StatefulBiTransformer Tests
// ============================================================================

#[test]
fn test_closure_as_stateful_bi_transformer() {
    // Test that closures implement StatefulBiTransformer
    let mut counter = 0;
    let mut transformer = |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    };

    // Use StatefulBiTransformer::apply which takes &mut self
    assert_eq!(transformer.apply(10, 20), 31);
}

#[test]
fn test_closure_into_box() {
    // Test closure conversion to BoxStatefulBiTransformer
    let mut counter = 0;
    let transformer = move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    };

    let mut boxed = StatefulBiTransformer::into_box(transformer);
    assert_eq!(boxed.apply(10, 20), 31);
    assert_eq!(boxed.apply(10, 20), 32);
}

#[test]
fn test_closure_into_rc() {
    // Test closure conversion to RcStatefulBiTransformer
    let mut counter = 0;
    let transformer = move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    };

    let mut rc_transformer = transformer.into_rc();
    assert_eq!(rc_transformer.apply(10, 20), 31);
    assert_eq!(rc_transformer.apply(10, 20), 32);
}

#[test]
fn test_closure_into_arc() {
    // Test closure conversion to ArcStatefulBiTransformer
    let mut counter = 0;
    let transformer = move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    };

    let mut arc_transformer = transformer.into_arc();
    assert_eq!(arc_transformer.apply(10, 20), 31);
    assert_eq!(arc_transformer.apply(10, 20), 32);
}

#[test]
fn test_closure_into_fn() {
    // Test closure into_fn conversion
    let mut counter = 0;
    let transformer = move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    };

    let mut closure = StatefulBiTransformer::into_fn(transformer);
    assert_eq!(closure(10, 20), 31);
    assert_eq!(closure(10, 20), 32);
}

// ============================================================================
// FnStatefulBiTransformerOps Tests
// ============================================================================

#[test]
fn test_fn_stateful_bi_transformer_ops_and_then() {
    // Test and_then extension method for closures
    let mut counter1 = 0;
    let bi_trans = move |x: i32, y: i32| {
        counter1 += 1;
        x + y + counter1
    };

    let mut counter2 = 0;
    let trans = move |x: i32| {
        counter2 += 1;
        x * counter2
    };

    let mut composed = bi_trans.and_then(trans);
    assert_eq!(composed.apply(10, 20), 31); // (10 + 20 + 1) * 1
    assert_eq!(composed.apply(10, 20), 64); // (10 + 20 + 2) * 2
}

#[test]
fn test_fn_stateful_bi_transformer_ops_and_then_with_transformer() {
    // Test and_then with a stateful transformer object
    let mut counter = 0;
    let bi_trans = move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    };

    let trans = BoxStatefulTransformer::new(|x: i32| x * 2);
    let mut composed = bi_trans.and_then(trans);

    assert_eq!(composed.apply(5, 5), 22); // (5 + 5 + 1) * 2
    assert_eq!(composed.apply(5, 5), 24); // (5 + 5 + 2) * 2
}

#[test]
fn test_fn_stateful_bi_transformer_ops_when() {
    // Test when extension method for closures
    let mut transformer = (|x: i32, y: i32| x + y)
        .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(|x, y| x * y);

    assert_eq!(transformer.apply(5, 3), 8);
    assert_eq!(transformer.apply(-5, 3), -15);
    assert_eq!(transformer.apply(0, 5), 0);
}

#[test]
fn test_fn_stateful_bi_transformer_ops_when_with_predicate() {
    // Test when with a predicate object
    let predicate = BoxBiPredicate::new(|x: &i32, _y: &i32| *x >= 10);

    let mut transformer = (|x: i32, y: i32| x + y)
        .when(predicate)
        .or_else(|x, y| x * y);

    assert_eq!(transformer.apply(15, 5), 20); // x >= 10, add
    assert_eq!(transformer.apply(5, 10), 50); // x < 10, multiply
}

#[test]
fn test_fn_stateful_bi_transformer_ops_to_fn() {
    // Test to_fn extension method for closures
    // Use a closure that doesn't capture mutable references so it can be cloned
    let transformer = |x: i32, y: i32| x + y;

    let mut fn_transformer = FnStatefulBiTransformerOps::to_fn(&transformer);
    assert_eq!(fn_transformer(10, 20), 30);
    assert_eq!(fn_transformer(5, 15), 20);

    // Original transformer still usable
    let mut original = transformer;
    assert_eq!(original.apply(1, 2), 3);
}

#[test]
fn test_closure_to_fn_method_call() {
    // Test closure's to_fn method via direct method call on FnStatefulBiTransformerOps trait
    // Use a closure that doesn't capture mutable references so it can be cloned
    let transformer = |x: i32, y: i32| x + y;

    // Test calling to_fn directly on the FnStatefulBiTransformerOps trait
    // This specifically tests the FnStatefulBiTransformerOps::to_fn implementation for closures
    let mut fn_transformer = FnStatefulBiTransformerOps::to_fn(&transformer);
    assert_eq!(fn_transformer(10, 20), 30);
    assert_eq!(fn_transformer(5, 15), 20);

    // Original transformer still usable
    let mut original = transformer;
    assert_eq!(original.apply(1, 2), 3);
}

#[test]
fn test_closure_as_stateful_bi_transformer_to_fn() {
    // Test closure used as StatefulBiTransformer calling to_fn method
    // This tests the blanket implementation of StatefulBiTransformer for FnMut closures
    let transformer = |x: i32, y: i32| x + y;

    // Test calling to_fn via StatefulBiTransformer trait (blanket implementation for closures)
    let mut fn_transformer = StatefulBiTransformer::to_fn(&transformer);
    assert_eq!(fn_transformer(10, 20), 30);
    assert_eq!(fn_transformer(5, 15), 20);

    // Original transformer still usable
    let mut original = transformer;
    assert_eq!(original.apply(1, 2), 3);
}

// ============================================================================
// BoxConditionalStatefulBiTransformer Tests
// ============================================================================

#[test]
fn test_box_conditional_or_else_basic() {
    // Test basic or_else functionality
    let add = BoxStatefulBiTransformer::new(|x: i32, y: i32| x + y);
    let multiply = BoxStatefulBiTransformer::new(|x: i32, y: i32| x * y);

    let mut conditional = add
        .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(multiply);

    assert_eq!(conditional.apply(5, 3), 8); // both positive, add
    assert_eq!(conditional.apply(-5, 3), -15); // not both positive, multiply
    assert_eq!(conditional.apply(0, 5), 0); // zero, multiply
}

#[test]
fn test_box_conditional_or_else_with_closure() {
    // Test or_else with closure
    let add = BoxStatefulBiTransformer::new(|x: i32, y: i32| x + y);

    let mut conditional = add.when(|x: &i32, _y: &i32| *x > 10).or_else(|x, y| x - y);

    assert_eq!(conditional.apply(15, 5), 20); // x > 10, add
    assert_eq!(conditional.apply(5, 3), 2); // x <= 10, subtract
}

#[test]
fn test_box_conditional_stateful_transformers() {
    // Test conditional with stateful transformers
    let mut then_count = 0;
    let mut else_count = 0;

    let then_trans = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        then_count += 1;
        format!("Then[{}]: {}", then_count, x + y)
    });

    let else_trans = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        else_count += 1;
        format!("Else[{}]: {}", else_count, x * y)
    });

    let mut conditional = then_trans
        .when(|x: &i32, _y: &i32| *x > 0)
        .or_else(else_trans);

    assert_eq!(conditional.apply(5, 3), "Then[1]: 8");
    assert_eq!(conditional.apply(-5, 3), "Else[1]: -15");
    assert_eq!(conditional.apply(10, 2), "Then[2]: 12");
}

// ============================================================================
// ArcConditionalStatefulBiTransformer Tests
// ============================================================================

#[test]
fn test_arc_conditional_or_else_basic() {
    // Test basic or_else functionality
    let add = ArcStatefulBiTransformer::new(|x: i32, y: i32| x + y);
    let multiply = ArcStatefulBiTransformer::new(|x: i32, y: i32| x * y);

    let mut conditional = add
        .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(multiply);

    assert_eq!(conditional.apply(5, 3), 8);
    assert_eq!(conditional.apply(-5, 3), -15);
}

#[test]
fn test_arc_conditional_clone() {
    // Test cloning of conditional transformer
    let add = ArcStatefulBiTransformer::new(|x: i32, y: i32| x + y);
    let conditional = add.when(|x: &i32, _y: &i32| *x > 0);

    let conditional_clone = conditional.clone();

    let mut result1 = conditional.or_else(|x, y| x * y);
    let mut result2 = conditional_clone.or_else(|x, y| x * y);

    assert_eq!(result1.apply(5, 3), 8);
    assert_eq!(result2.apply(5, 3), 8);
    assert_eq!(result1.apply(-5, 3), -15);
    assert_eq!(result2.apply(-5, 3), -15);
}

#[test]
fn test_arc_conditional_stateful_transformers() {
    // Test conditional with stateful transformers
    let mut then_count = 0;
    let mut else_count = 0;

    let then_trans = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        then_count += 1;
        format!("Then[{}]: {}", then_count, x + y)
    });

    let else_trans = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        else_count += 1;
        format!("Else[{}]: {}", else_count, x * y)
    });

    let mut conditional = then_trans
        .when(|x: &i32, _y: &i32| *x > 0)
        .or_else(else_trans);

    assert_eq!(conditional.apply(5, 3), "Then[1]: 8");
    assert_eq!(conditional.apply(-5, 3), "Else[1]: -15");
}

// ============================================================================
// RcConditionalStatefulBiTransformer Tests
// ============================================================================

#[test]
fn test_rc_conditional_or_else_basic() {
    // Test basic or_else functionality
    let add = RcStatefulBiTransformer::new(|x: i32, y: i32| x + y);
    let multiply = RcStatefulBiTransformer::new(|x: i32, y: i32| x * y);

    let mut conditional = add
        .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(multiply);

    assert_eq!(conditional.apply(5, 3), 8);
    assert_eq!(conditional.apply(-5, 3), -15);
}

#[test]
fn test_rc_conditional_clone() {
    // Test cloning of conditional transformer
    let add = RcStatefulBiTransformer::new(|x: i32, y: i32| x + y);
    let conditional = add.when(|x: &i32, _y: &i32| *x > 0);

    let conditional_clone = conditional.clone();

    let mut result1 = conditional.or_else(|x, y| x * y);
    let mut result2 = conditional_clone.or_else(|x, y| x * y);

    assert_eq!(result1.apply(5, 3), 8);
    assert_eq!(result2.apply(5, 3), 8);
    assert_eq!(result1.apply(-5, 3), -15);
    assert_eq!(result2.apply(-5, 3), -15);
}

#[test]
fn test_rc_conditional_stateful_transformers() {
    // Test conditional with stateful transformers
    let mut then_count = 0;
    let mut else_count = 0;

    let then_trans = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        then_count += 1;
        format!("Then[{}]: {}", then_count, x + y)
    });

    let else_trans = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        else_count += 1;
        format!("Else[{}]: {}", else_count, x * y)
    });

    let mut conditional = then_trans
        .when(|x: &i32, _y: &i32| *x > 0)
        .or_else(else_trans);

    assert_eq!(conditional.apply(5, 3), "Then[1]: 8");
    assert_eq!(conditional.apply(-5, 3), "Else[1]: -15");
}

// ============================================================================
// BiTransformerOnce Implementation Tests
// ============================================================================

#[test]
fn test_box_stateful_bi_transformer_apply() {
    // Test apply consuming the transformer
    let mut counter = 0;
    let mut transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    assert_eq!(transformer.apply(10, 20), 31);
    // transformer is now consumed
}

#[test]
fn test_arc_stateful_bi_transformer_apply() {
    // Test apply for ArcStatefulBiTransformer
    let mut counter = 0;
    let mut transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });

    assert_eq!(transformer.apply(10, 20), 31);
}

#[test]
fn test_arc_stateful_bi_transformer_to_box() {
    // Test non-consuming to_box conversion
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x * y * counter
    });

    let mut once_transformer = StatefulBiTransformer::to_box(&transformer);
    assert_eq!(once_transformer.apply(10, 2), 20); // 10 * 2 * 1

    // Original still usable
    let mut original = transformer.clone();
    assert_eq!(original.apply(10, 2), 40); // 10 * 2 * 2 (state continues)
}

#[test]
fn test_arc_stateful_bi_transformer_to_fn() {
    // Test non-consuming to_fn conversion
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x * y * counter
    });

    let mut fn_once = StatefulBiTransformer::to_fn(&transformer);
    assert_eq!(fn_once(10, 2), 20); // 10 * 2 * 1

    // Original still usable
    let mut original = transformer.clone();
    assert_eq!(original.apply(10, 2), 40); // 10 * 2 * 2 (state continues)
}

// ============================================================================
// Conditional StatefulBiTransformer Display/Debug Tests
// ============================================================================

#[cfg(test)]
mod conditional_stateful_bi_transformer_display_debug_tests {
    use super::*;

    #[test]
    fn test_box_conditional_stateful_bi_transformer_display() {
        let mut counter = 0;
        let add = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_box_conditional_stateful_bi_transformer_display_no_name() {
        let mut counter = 0;
        let add = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(display_str, "BoxConditionalStatefulBiTransformer(BoxStatefulBiTransformer, BoxBiPredicate(unnamed))");
    }

    #[test]
    fn test_box_conditional_stateful_bi_transformer_debug() {
        let mut counter = 0;
        let add = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_rc_conditional_stateful_bi_transformer_display() {
        let mut counter = 0;
        let add = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_rc_conditional_stateful_bi_transformer_display_no_name() {
        let mut counter = 0;
        let add = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(
            display_str,
            "RcConditionalStatefulBiTransformer(RcStatefulBiTransformer, RcBiPredicate(unnamed))"
        );
    }

    #[test]
    fn test_rc_conditional_stateful_bi_transformer_debug() {
        let mut counter = 0;
        let add = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_arc_conditional_stateful_bi_transformer_display() {
        let mut counter = 0;
        let add = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalStatefulBiTransformer"));
    }

    #[test]
    fn test_arc_conditional_stateful_bi_transformer_display_no_name() {
        let mut counter = 0;
        let add = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let display_str = format!("{}", conditional);
        assert_eq!(display_str, "ArcConditionalStatefulBiTransformer(ArcStatefulBiTransformer, ArcBiPredicate(unnamed))");
    }

    #[test]
    fn test_arc_conditional_stateful_bi_transformer_debug() {
        let mut counter = 0;
        let add = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
        let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0);
        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalStatefulBiTransformer"));
    }
}

// ============================================================================
// StatefulBiTransformer Trait Default Methods Tests
// ============================================================================

#[cfg(test)]
mod stateful_bi_transformer_trait_default_methods_tests {
    use super::*;
    use prism3_function::BiTransformerOnce;

    // Custom struct implementing StatefulBiTransformer to test default methods
    #[derive(Clone)]
    struct TestStatefulBiTransformer {
        state: i32,
    }

    impl TestStatefulBiTransformer {
        fn new(initial_state: i32) -> Self {
            Self {
                state: initial_state,
            }
        }
    }

    impl StatefulBiTransformer<i32, i32, i32> for TestStatefulBiTransformer {
        fn apply(&mut self, first: i32, second: i32) -> i32 {
            self.state += 1;
            first + second + self.state
        }
    }

    #[test]
    fn test_into_box() {
        let transformer = TestStatefulBiTransformer::new(10);
        let mut boxed = StatefulBiTransformer::into_box(transformer);

        assert_eq!(boxed.apply(5, 3), 19); // 5 + 3 + 11 (initial 10 + 1)
        assert_eq!(boxed.apply(5, 3), 20); // 5 + 3 + 12 (state continues)
    }

    #[test]
    fn test_into_rc() {
        let transformer = TestStatefulBiTransformer::new(20);
        let mut rc_transformer = StatefulBiTransformer::into_rc(transformer);

        assert_eq!(rc_transformer.apply(2, 4), 27); // 2 + 4 + 21
        assert_eq!(rc_transformer.apply(2, 4), 28); // 2 + 4 + 22
    }

    #[test]
    fn test_into_arc() {
        let transformer = TestStatefulBiTransformer::new(30);
        let mut arc_transformer = StatefulBiTransformer::into_arc(transformer);

        assert_eq!(arc_transformer.apply(1, 2), 34); // 1 + 2 + 31
        assert_eq!(arc_transformer.apply(1, 2), 35); // 1 + 2 + 32
    }

    #[test]
    fn test_into_fn() {
        let transformer = TestStatefulBiTransformer::new(40);
        let mut fn_transformer = StatefulBiTransformer::into_fn(transformer);

        assert_eq!(fn_transformer(3, 3), 47); // 3 + 3 + 41
        assert_eq!(fn_transformer(3, 3), 48); // 3 + 3 + 42
    }

    #[test]
    fn test_to_box() {
        let transformer = TestStatefulBiTransformer::new(50);
        let mut boxed = StatefulBiTransformer::to_box(&transformer);

        // Test that the boxed transformer works
        assert_eq!(boxed.apply(4, 2), 57); // 4 + 2 + 51

        // Test that original transformer is still usable
        let mut original = transformer.clone();
        assert_eq!(original.apply(4, 2), 57); // 4 + 2 + 51 (independent state)
    }

    #[test]
    fn test_to_rc() {
        let transformer = TestStatefulBiTransformer::new(60);
        let mut rc_transformer = StatefulBiTransformer::to_rc(&transformer);

        // Test that the Rc transformer works
        assert_eq!(rc_transformer.apply(6, 1), 68); // 6 + 1 + 61

        // Test that original transformer is still usable
        let mut original = transformer.clone();
        assert_eq!(original.apply(6, 1), 68); // 6 + 1 + 61 (independent state)
    }

    #[test]
    fn test_to_arc() {
        let transformer = TestStatefulBiTransformer::new(70);
        let mut arc_transformer = StatefulBiTransformer::to_arc(&transformer);

        // Test that the Arc transformer works
        assert_eq!(arc_transformer.apply(7, 2), 80); // 7 + 2 + 71

        // Test that original transformer is still usable
        let mut original = transformer.clone();
        assert_eq!(original.apply(7, 2), 80); // 7 + 2 + 71 (independent state)
    }

    #[test]
    fn test_to_fn() {
        let transformer = TestStatefulBiTransformer::new(80);
        let mut fn_transformer = StatefulBiTransformer::to_fn(&transformer);

        // Test that the fn transformer works
        assert_eq!(fn_transformer(8, 2), 91); // 8 + 2 + 81

        // Test that original transformer is still usable
        let mut original = transformer.clone();
        assert_eq!(original.apply(8, 2), 91); // 8 + 2 + 81 (independent state)
    }

    #[test]
    fn test_into_once() {
        let transformer = TestStatefulBiTransformer::new(90);
        let once_transformer = StatefulBiTransformer::into_once(transformer);

        // Test that the once transformer works
        assert_eq!(once_transformer.apply(9, 1), 101); // 9 + 1 + 91
    }

    #[test]
    fn test_to_once() {
        let transformer = TestStatefulBiTransformer::new(100);
        let once_transformer = StatefulBiTransformer::to_once(&transformer);

        // Test that the once transformer works
        assert_eq!(once_transformer.apply(10, 0), 111); // 10 + 0 + 101

        // Test that original transformer is still usable
        let mut original = transformer.clone();
        assert_eq!(original.apply(10, 0), 111); // 10 + 0 + 101 (independent state)
    }
}

// ============================================================================
// Basic StatefulBiTransformer Display Tests
// ============================================================================

#[test]
fn test_box_stateful_bi_transformer_display_with_name() {
    let mut counter = 0;
    let transformer =
        BoxStatefulBiTransformer::new_with_name("add_counter", move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "BoxStatefulBiTransformer(add_counter)");
}

#[test]
fn test_box_stateful_bi_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = BoxStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "BoxStatefulBiTransformer");
}

#[test]
fn test_rc_stateful_bi_transformer_display_with_name() {
    let mut counter = 0;
    let transformer =
        RcStatefulBiTransformer::new_with_name("add_counter", move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "RcStatefulBiTransformer(add_counter)");
}

#[test]
fn test_rc_stateful_bi_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = RcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "RcStatefulBiTransformer");
}

#[test]
fn test_arc_stateful_bi_transformer_display_with_name() {
    let mut counter = 0;
    let transformer =
        ArcStatefulBiTransformer::new_with_name("add_counter", move |x: i32, y: i32| {
            counter += 1;
            x + y + counter
        });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "ArcStatefulBiTransformer(add_counter)");
}

#[test]
fn test_arc_stateful_bi_transformer_display_without_name() {
    let mut counter = 0;
    let transformer = ArcStatefulBiTransformer::new(move |x: i32, y: i32| {
        counter += 1;
        x + y + counter
    });
    let display_str = format!("{}", transformer);
    assert_eq!(display_str, "ArcStatefulBiTransformer");
}
