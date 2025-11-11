/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Comprehensive tests for BiMutatingFunction trait and its implementations

use prism3_function::{
    ArcBiMutatingFunction,
    ArcBiPredicate,
    BiMutatingFunction,
    BiMutatingFunctionOnce,
    BoxBiMutatingFunction,
    FnBiMutatingFunctionOps,
    RcBiMutatingFunction,
    RcBiPredicate,
};

// ============================================================================
// Helper Functions and Data Structures
// ============================================================================


fn append_strings(x: &mut String, y: &mut String) -> usize {
    x.push_str("_modified");
    y.push_str("_changed");
    x.len() + y.len()
}

#[derive(Clone, Debug, PartialEq)]
struct TestStruct {
    value: i32,
}

impl TestStruct {
    fn new(value: i32) -> Self {
        Self { value }
    }

    fn modify(&mut self, other: &mut Self) -> i32 {
        self.value += other.value;
        other.value *= 2;
        self.value + other.value
    }
}

fn modify_structs(a: &mut TestStruct, b: &mut TestStruct) -> i32 {
    a.modify(b)
}

// ============================================================================
// BiMutatingFunction Trait Tests - Core Functionality
// ============================================================================

#[test]
fn test_bi_mutating_function_trait_apply() {
    // Test that BiMutatingFunction trait's apply method works correctly
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };

    let mut a = 20;
    let mut b = 22;
    assert_eq!(swap_sum.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);
}

#[test]
fn test_bi_mutating_function_trait_apply_with_complex_types() {
    let modify = |a: &mut TestStruct, b: &mut TestStruct| {
        a.modify(b)
    };

    let mut s1 = TestStruct::new(10);
    let mut s2 = TestStruct::new(5);
    let result = modify.apply(&mut s1, &mut s2);

    assert_eq!(result, 25); // (10+5) + (5*2) = 15 + 10 = 25
    assert_eq!(s1.value, 15);
    assert_eq!(s2.value, 10);
}

#[test]
fn test_bi_mutating_function_trait_into_box() {
    // Test conversion from closure to BoxBiMutatingFunction
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let boxed = BiMutatingFunction::into_box(swap_sum);

    let mut a = 20;
    let mut b = 22;
    assert_eq!(boxed.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);
}

#[test]
fn test_bi_mutating_function_trait_into_rc() {
    // Test conversion from closure to RcBiMutatingFunction
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let rc = swap_sum.into_rc();

    let mut a = 20;
    let mut b = 22;
    assert_eq!(rc.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);
}

#[test]
fn test_bi_mutating_function_trait_into_arc() {
    // Test conversion from closure to ArcBiMutatingFunction
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let arc = swap_sum.into_arc();

    let mut a = 20;
    let mut b = 22;
    assert_eq!(arc.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);
}

#[test]
fn test_bi_mutating_function_trait_into_fn() {
    // Test conversion to closure
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let func = BiMutatingFunction::into_fn(swap_sum);

    let mut a = 20;
    let mut b = 22;
    assert_eq!(func(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);
}

#[test]
fn test_bi_mutating_function_trait_into_once() {
    // Test conversion to BiMutatingFunctionOnce
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let once = swap_sum.into_once();

    let mut a = 20;
    let mut b = 22;
    assert_eq!(once.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);
}

#[test]
fn test_bi_mutating_function_trait_to_box() {
    // Test non-consuming conversion to BoxBiMutatingFunction
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let boxed = BiMutatingFunction::to_box(&swap_sum);

    let mut a = 20;
    let mut b = 22;
    assert_eq!(boxed.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);

    // Original function should still be usable
    let mut c = 30;
    let mut d = 32;
    assert_eq!(swap_sum.apply(&mut c, &mut d), 62);
}

#[test]
fn test_bi_mutating_function_trait_to_rc() {
    // Test non-consuming conversion to RcBiMutatingFunction
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let rc = swap_sum.to_rc();

    let mut a = 20;
    let mut b = 22;
    assert_eq!(rc.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);

    // Original function should still be usable
    let mut c = 30;
    let mut d = 32;
    assert_eq!(swap_sum.apply(&mut c, &mut d), 62);
}

#[test]
fn test_bi_mutating_function_trait_to_arc() {
    // Test non-consuming conversion to ArcBiMutatingFunction
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let arc = swap_sum.to_arc();

    let mut a = 20;
    let mut b = 22;
    assert_eq!(arc.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);

    // Original function should still be usable
    let mut c = 30;
    let mut d = 32;
    assert_eq!(swap_sum.apply(&mut c, &mut d), 62);
}

#[test]
fn test_bi_mutating_function_trait_to_fn() {
    // Test non-consuming conversion to closure
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let func = BiMutatingFunction::to_fn(&swap_sum);

    let mut a = 20;
    let mut b = 22;
    assert_eq!(func(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);

    // Original function should still be usable
    let mut c = 30;
    let mut d = 32;
    assert_eq!(swap_sum.apply(&mut c, &mut d), 62);
}

#[test]
fn test_bi_mutating_function_trait_to_once() {
    // Test non-consuming conversion to BiMutatingFunctionOnce
    let swap_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };
    let once = swap_sum.to_once();

    let mut a = 20;
    let mut b = 22;
    assert_eq!(once.apply(&mut a, &mut b), 42);
    assert_eq!(a, 22);
    assert_eq!(b, 20);

    // Original function should still be usable
    let mut c = 30;
    let mut d = 32;
    assert_eq!(swap_sum.apply(&mut c, &mut d), 62);
}

// ============================================================================
// BoxBiMutatingFunction Tests
// ============================================================================

#[test]
fn test_box_bi_mutating_function_new() {
    let swap_sum = BoxBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });
    let mut a = 10;
    let mut b = 15;
    assert_eq!(swap_sum.apply(&mut a, &mut b), 25);
    assert_eq!(a, 15);
    assert_eq!(b, 10);
}

#[test]
fn test_box_bi_mutating_function_new_with_name() {
    let swap_sum = BoxBiMutatingFunction::new_with_name(
        "swap_and_sum",
        |x: &mut i32, y: &mut i32| {
            let temp = *x;
            *x = *y;
            *y = temp;
            *x + *y
        },
    );
    assert_eq!(swap_sum.name(), Some("swap_and_sum"));
    let mut a = 10;
    let mut b = 15;
    assert_eq!(swap_sum.apply(&mut a, &mut b), 25);
}

#[test]
fn test_box_bi_mutating_function_new_with_optional_name() {
    let swap_sum = BoxBiMutatingFunction::new_with_optional_name(
        |x: &mut i32, y: &mut i32| {
            let temp = *x;
            *x = *y;
            *y = temp;
            *x + *y
        },
        Some("test_function".to_string()),
    );
    assert_eq!(swap_sum.name(), Some("test_function"));

    let no_name = BoxBiMutatingFunction::new_with_optional_name(
        |x: &mut i32, y: &mut i32| *x + *y,
        None,
    );
    assert_eq!(no_name.name(), None);
}

#[test]
fn test_box_bi_mutating_function_name_and_set_name() {
    let mut swap_sum = BoxBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    assert_eq!(swap_sum.name(), None);
    swap_sum.set_name("modified_name");
    assert_eq!(swap_sum.name(), Some("modified_name"));
    swap_sum.set_name("another_name");
    assert_eq!(swap_sum.name(), Some("another_name"));
}

#[test]
fn test_box_bi_mutating_function_constant() {
    let constant = BoxBiMutatingFunction::constant(42);
    let mut a = 1;
    let mut b = 2;
    assert_eq!(constant.apply(&mut a, &mut b), 42);

    let mut c = 100;
    let mut d = 200;
    assert_eq!(constant.apply(&mut c, &mut d), 42);
}

#[test]
fn test_box_bi_mutating_function_debug_display() {
    let swap_sum = BoxBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let debug_str = format!("{:?}", swap_sum);
    assert!(debug_str.contains("BoxBiMutatingFunction"));

    let display_str = format!("{}", swap_sum);
    assert!(display_str.contains("BoxBiMutatingFunction"));
}

#[test]
fn test_box_bi_mutating_function_with_strings() {
    let append = BoxBiMutatingFunction::new(append_strings);
    let mut s1 = "hello".to_string();
    let mut s2 = "world".to_string();

    let result = append.apply(&mut s1, &mut s2);
    assert_eq!(result, 14 + 13); // "hello_modified".len() + "world_changed".len()
    assert_eq!(s1, "hello_modified");
    assert_eq!(s2, "world_changed");
}

#[test]
fn test_box_bi_mutating_function_with_structs() {
    let modify = BoxBiMutatingFunction::new(modify_structs);
    let mut s1 = TestStruct::new(10);
    let mut s2 = TestStruct::new(5);

    let result = modify.apply(&mut s1, &mut s2);
    assert_eq!(result, 25); // (10+5) + (5*2) = 15 + 10 = 25
    assert_eq!(s1.value, 15);
    assert_eq!(s2.value, 10);
}

// ============================================================================
// RcBiMutatingFunction Tests
// ============================================================================

#[test]
fn test_rc_bi_mutating_function_new() {
    let swap_sum = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });
    let mut a = 10;
    let mut b = 15;
    assert_eq!(swap_sum.apply(&mut a, &mut b), 25);
    assert_eq!(a, 15);
    assert_eq!(b, 10);
}

#[test]
fn test_rc_bi_mutating_function_clone() {
    let original = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let cloned = original.clone();

    let mut a = 10;
    let mut b = 15;
    assert_eq!(original.apply(&mut a, &mut b), 25);
    assert_eq!(a, 15);
    assert_eq!(b, 10);

    let mut c = 20;
    let mut d = 25;
    assert_eq!(cloned.apply(&mut c, &mut d), 45);
    assert_eq!(c, 25);
    assert_eq!(d, 20);
}

#[test]
fn test_rc_bi_mutating_function_name_and_set_name() {
    let mut swap_sum = RcBiMutatingFunction::new_with_name(
        "rc_function",
        |x: &mut i32, y: &mut i32| {
            let temp = *x;
            *x = *y;
            *y = temp;
            *x + *y
        },
    );

    assert_eq!(swap_sum.name(), Some("rc_function"));
    swap_sum.set_name("modified_rc");
    assert_eq!(swap_sum.name(), Some("modified_rc"));
}

#[test]
fn test_rc_bi_mutating_function_constant() {
    let constant = RcBiMutatingFunction::constant(99);
    let mut a = 1;
    let mut b = 2;
    assert_eq!(constant.apply(&mut a, &mut b), 99);

    let cloned = constant.clone();
    let mut c = 10;
    let mut d = 20;
    assert_eq!(cloned.apply(&mut c, &mut d), 99);
}

#[test]
fn test_rc_bi_mutating_function_debug_display() {
    let swap_sum = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let debug_str = format!("{:?}", swap_sum);
    assert!(debug_str.contains("RcBiMutatingFunction"));

    let display_str = format!("{}", swap_sum);
    assert!(display_str.contains("RcBiMutatingFunction"));
}

// ============================================================================
// ArcBiMutatingFunction Tests
// ============================================================================

#[test]
fn test_arc_bi_mutating_function_new() {
    let swap_sum = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });
    let mut a = 10;
    let mut b = 15;
    assert_eq!(swap_sum.apply(&mut a, &mut b), 25);
    assert_eq!(a, 15);
    assert_eq!(b, 10);
}

#[test]
fn test_arc_bi_mutating_function_clone() {
    let original = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let cloned = original.clone();

    let mut a = 10;
    let mut b = 15;
    assert_eq!(original.apply(&mut a, &mut b), 25);
    assert_eq!(a, 15);
    assert_eq!(b, 10);

    let mut c = 20;
    let mut d = 25;
    assert_eq!(cloned.apply(&mut c, &mut d), 45);
    assert_eq!(c, 25);
    assert_eq!(d, 20);
}

#[test]
fn test_arc_bi_mutating_function_thread_safety() {
    use std::thread;

    let function = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        *x += 1;
        *y += 2;
        *x + *y
    });

    let func1 = function.clone();
    let func2 = function.clone();

    let handle1 = thread::spawn(move || {
        let mut a = 10;
        let mut b = 20;
        func1.apply(&mut a, &mut b)
    });

    let handle2 = thread::spawn(move || {
        let mut a = 30;
        let mut b = 40;
        func2.apply(&mut a, &mut b)
    });

    let result1 = handle1.join().unwrap();
    let result2 = handle2.join().unwrap();

    assert_eq!(result1, 33); // (10+1) + (20+2) = 11 + 22 = 33
    assert_eq!(result2, 73); // (30+1) + (40+2) = 31 + 42 = 73
}

#[test]
fn test_arc_bi_mutating_function_name_and_set_name() {
    let mut swap_sum = ArcBiMutatingFunction::new_with_name(
        "arc_function",
        |x: &mut i32, y: &mut i32| {
            let temp = *x;
            *x = *y;
            *y = temp;
            *x + *y
        },
    );

    assert_eq!(swap_sum.name(), Some("arc_function"));
    swap_sum.set_name("modified_arc");
    assert_eq!(swap_sum.name(), Some("modified_arc"));
}

#[test]
fn test_arc_bi_mutating_function_constant() {
    let constant = ArcBiMutatingFunction::constant(123);
    let mut a = 1;
    let mut b = 2;
    assert_eq!(constant.apply(&mut a, &mut b), 123);

    let cloned = constant.clone();
    let mut c = 10;
    let mut d = 20;
    assert_eq!(cloned.apply(&mut c, &mut d), 123);
}

#[test]
fn test_arc_bi_mutating_function_debug_display() {
    let swap_sum = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let debug_str = format!("{:?}", swap_sum);
    assert!(debug_str.contains("ArcBiMutatingFunction"));

    let display_str = format!("{}", swap_sum);
    assert!(display_str.contains("ArcBiMutatingFunction"));
}

// ============================================================================
// Function Composition Tests - and_then
// ============================================================================

#[test]
fn test_fn_bi_mutating_function_ops_and_then() {
    let swap_and_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };

    let double = |result: &i32| *result * 2;
    let composed = swap_and_sum.and_then(double);

    let mut a = 3;
    let mut b = 5;
    // swap_and_sum: a=5, b=3, result=5+3=8
    // double: 8*2=16
    assert_eq!(composed.apply(&mut a, &mut b), 16);
    assert_eq!(a, 5);
    assert_eq!(b, 3);
}

#[test]
fn test_fn_bi_mutating_function_ops_and_then_chain() {
    let add_and_modify = |x: &mut i32, y: &mut i32| {
        *x += 10;
        *y += 20;
        *x + *y
    };

    let to_string = |x: &i32| x.to_string();
    let add_prefix = |s: &mut String| { let result = format!("Result: {}", *s); *s = String::new(); result };

    let composed = add_and_modify.and_then(to_string).and_then(add_prefix);

    let mut a = 5;
    let mut b = 3;
    // add_and_modify: a=15, b=23, result=15+23=38
    // to_string: "38"
    // add_prefix: "Result: 38"
    let result = composed.apply(&mut a, &mut b);
    assert_eq!(result, "Result: 38");
    assert_eq!(a, 15);
    assert_eq!(b, 23);
}

// ============================================================================
// Conditional Function Tests - when/or_else
// ============================================================================

#[test]
fn test_fn_bi_mutating_function_ops_when_or_else() {
    let swap_and_sum = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };

    let multiply = |x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    };

    let conditional = swap_and_sum.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);

    // Test when condition is true
    let mut a = 5;
    let mut b = 3;
    assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum: (3+5) = 8
    assert_eq!(a, 3); // swapped from 5
    assert_eq!(b, 5); // swapped from 3

    // Test when condition is false (negative numbers)
    let conditional2 = swap_and_sum.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
    let mut c = -5;
    let mut d = 3;
    assert_eq!(conditional2.apply(&mut c, &mut d), -15); // multiply: (-5 * 3) = -15
    assert_eq!(c, -15);
    assert_eq!(d, 3);
}

#[test]
fn test_box_conditional_bi_mutating_function() {
    let swap_and_sum = BoxBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let multiply = BoxBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    });

    let conditional = swap_and_sum.when(|x: &i32, _y: &i32| *x > 0).or_else(multiply);

    // Test when condition is true
    let mut a = 5;
    let mut b = 3;
    assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum executed

    // Test when condition is false
    let mut c = -5;
    let mut d = 3;
    assert_eq!(conditional.apply(&mut c, &mut d), -15); // multiply executed
}

#[test]
fn test_rc_conditional_bi_mutating_function() {
    let swap_and_sum = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let multiply = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    });

    let conditional = swap_and_sum.when(|x: &i32, _y: &i32| *x > 0).or_else(multiply);
    let cloned = conditional.clone();

    // Test when condition is true
    let mut a = 5;
    let mut b = 3;
    assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum executed

    // Test cloned conditional
    let mut c = 10;
    let mut d = 2;
    assert_eq!(cloned.apply(&mut c, &mut d), 12); // swap_and_sum executed
}

#[test]
fn test_arc_conditional_bi_mutating_function() {
    let swap_and_sum = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let multiply = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    });

    let conditional = swap_and_sum.when(|x: &i32, _y: &i32| *x > 0).or_else(multiply);
    let cloned = conditional.clone();

    // Test when condition is true
    let mut a = 5;
    let mut b = 3;
    assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum executed

    // Test cloned conditional
    let mut c = 10;
    let mut d = 2;
    assert_eq!(cloned.apply(&mut c, &mut d), 12); // swap_and_sum executed
}

#[test]
fn test_rc_conditional_bi_mutating_function_clone() {
    let swap_and_sum = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let multiply = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    });

    let conditional = swap_and_sum.when(|x: &i32, _y: &i32| *x > 0).or_else(multiply);
    let cloned = conditional.clone();

    // Test original
    let mut a = 5;
    let mut b = 3;
    assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum executed

    // Test cloned (should behave identically)
    let mut c = 10;
    let mut d = 2;
    assert_eq!(cloned.apply(&mut c, &mut d), 12); // swap_and_sum executed
}

#[test]
fn test_arc_conditional_bi_mutating_function_clone() {
    let swap_and_sum = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });

    let multiply = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    });

    let conditional = swap_and_sum.when(|x: &i32, _y: &i32| *x > 0).or_else(multiply);
    let cloned = conditional.clone();

    // Test original
    let mut a = 5;
    let mut b = 3;
    assert_eq!(conditional.apply(&mut a, &mut b), 8); // swap_and_sum executed

    // Test cloned (should behave identically)
    let mut c = 10;
    let mut d = 2;
    assert_eq!(cloned.apply(&mut c, &mut d), 12); // swap_and_sum executed
}

#[test]
fn test_impl_conditional_function_clone_three_params_bi_mutating_macro_coverage() {
    println!("Starting test_impl_conditional_function_clone_three_params_bi_mutating_macro_coverage");

    // Test to ensure the three-parameter version of impl_conditional_function_clone macro is covered
    // for bi-mutating functions. This test verifies that the macro generates Clone implementations
    // for RcConditionalBiMutatingFunction<T, U, R> and ArcConditionalBiMutatingFunction<T, U, R>

    // Test RcConditionalBiMutatingFunction (three parameters: T, U, R)
    {
        println!("Testing RcConditionalBiMutatingFunction with macro-generated Clone (three parameters)");
        let swap = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
            let temp = *x;
            *x = *y;
            *y = temp;
            *x + *y
        });
        let pred = RcBiPredicate::new(|x: &i32, y: &i32| *x > *y);

        let conditional_rc = swap.when(pred);

        println!("Calling clone() on RcConditionalBiMutatingFunction - this should trigger macro-generated three-param code");
        let cloned_rc = conditional_rc.clone();
        println!("Clone completed for RcConditionalBiMutatingFunction");

        // Create or_else to test functionality
        let multiply = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
            *x *= *y;
            *x
        });
        let func = cloned_rc.or_else(multiply);

        // Verify functionality
        let mut a1 = 5;
        let mut b1 = 3;
        assert_eq!(func.apply(&mut a1, &mut b1), 8); // when branch: 5 > 3, swapped: 3 + 5 = 8

        let mut a2 = 2;
        let mut b2 = 7;
        assert_eq!(func.apply(&mut a2, &mut b2), 14); // or_else branch: 2 <= 7, multiplied: 2 * 7 = 14
        println!("RcConditionalBiMutatingFunction test passed");
    }

    // Test ArcConditionalBiMutatingFunction (three parameters: T, U, R)
    {
        println!("Testing ArcConditionalBiMutatingFunction with macro-generated Clone (three parameters)");
        let increment = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
            *x += *y;
            *x
        });
        let pred = ArcBiPredicate::new(|x: &i32, _y: &i32| *x >= 0);

        let conditional_arc = increment.when(pred);

        println!("Calling clone() on ArcConditionalBiMutatingFunction - this should trigger macro-generated three-param code");
        let cloned_arc = conditional_arc.clone();
        println!("Clone completed for ArcConditionalBiMutatingFunction");

        // Create or_else to test functionality
        let decrement = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
            *x -= *y;
            *x
        });
        let func = cloned_arc.or_else(decrement);

        // Verify functionality
        let mut c1 = 5;
        let mut d1 = 3;
        assert_eq!(func.apply(&mut c1, &mut d1), 8); // when branch: increment: 5 + 3 = 8

        let mut c2 = -2;
        let mut d2 = 3;
        assert_eq!(func.apply(&mut c2, &mut d2), -5); // or_else branch: decrement: -2 - 3 = -5
        println!("ArcConditionalBiMutatingFunction test passed");
    }

    println!("Three-parameter conditional clone macro test for bi-mutating functions passed!");
}

#[test]
fn test_conditional_bi_mutating_function_with_structs() {
    let modify = BoxBiMutatingFunction::new(modify_structs);
    let no_op = BoxBiMutatingFunction::new(|_a: &mut TestStruct, _b: &mut TestStruct| 0);

    let conditional = modify.when(|a: &TestStruct, b: &TestStruct| a.value > 0 && b.value > 0).or_else(no_op);

    // Test when condition is true
    let mut s1 = TestStruct::new(10);
    let mut s2 = TestStruct::new(5);
    let result = conditional.apply(&mut s1, &mut s2);
    assert_eq!(result, 25); // modify executed: (10+5) + (5*2) = 15 + 10 = 25
    assert_eq!(s1.value, 15);
    assert_eq!(s2.value, 10);

    // Test when condition is false
    let mut s3 = TestStruct::new(-10);
    let mut s4 = TestStruct::new(5);
    let result2 = conditional.apply(&mut s3, &mut s4);
    assert_eq!(result2, 0); // no_op executed
    assert_eq!(s3.value, -10); // unchanged
    assert_eq!(s4.value, 5); // unchanged
}

// ============================================================================
// Edge Cases and Boundary Tests
// ============================================================================

#[test]
fn test_bi_mutating_function_with_zero_values() {
    let add = |x: &mut i32, y: &mut i32| {
        *x += *y;
        *x
    };

    let mut a = 0;
    let mut b = 0;
    assert_eq!(add.apply(&mut a, &mut b), 0);
    assert_eq!(a, 0);
    assert_eq!(b, 0);

    let mut c = 0;
    let mut d = 5;
    assert_eq!(add.apply(&mut c, &mut d), 5);
    assert_eq!(c, 5);
    assert_eq!(d, 5);
}

#[test]
fn test_bi_mutating_function_with_negative_values() {
    let multiply = |x: &mut i32, y: &mut i32| {
        *x *= *y;
        *x
    };

    let mut a = -5;
    let mut b = 3;
    assert_eq!(multiply.apply(&mut a, &mut b), -15);
    assert_eq!(a, -15);
    assert_eq!(b, 3);

    let mut c = -2;
    let mut d = -4;
    assert_eq!(multiply.apply(&mut c, &mut d), 8);
    assert_eq!(c, 8);
    assert_eq!(d, -4);
}

#[test]
fn test_bi_mutating_function_with_large_values() {
    let add = |x: &mut i64, y: &mut i64| {
        *x += *y;
        *x
    };

    let mut a = i64::MAX - 10;
    let mut b = 5;
    assert_eq!(add.apply(&mut a, &mut b), i64::MAX - 5);
    assert_eq!(a, i64::MAX - 5);
    assert_eq!(b, 5);
}

#[test]
fn test_bi_mutating_function_with_empty_strings() {
    let concat = |x: &mut String, y: &mut String| {
        x.push_str(&y);
        x.len()
    };

    let mut s1 = String::new();
    let mut s2 = String::new();
    assert_eq!(concat.apply(&mut s1, &mut s2), 0);
    assert_eq!(s1, "");
    assert_eq!(s2, "");

    let mut s3 = "hello".to_string();
    let mut s4 = String::new();
    assert_eq!(concat.apply(&mut s3, &mut s4), 5);
    assert_eq!(s3, "hello");
    assert_eq!(s4, "");
}

#[test]
fn test_bi_mutating_function_with_unicode_strings() {
    let append = |x: &mut String, y: &mut String| {
        x.push_str("🌟");
        y.push_str("⭐");
        x.len() + y.len()
    };

    let mut s1 = "Hello".to_string();
    let mut s2 = "World".to_string();
    let result = append.apply(&mut s1, &mut s2);
    assert_eq!(s1, "Hello🌟");
    assert_eq!(s2, "World⭐");
    assert_eq!(result, 9 + 8); // "Hello🌟".len() + "World⭐".len()
}

#[test]
fn test_bi_mutating_function_identity_operations() {
    // Test functions that don't modify inputs
    let sum = |x: &mut i32, y: &mut i32| *x + *y;

    let mut a = 10;
    let mut b = 20;
    assert_eq!(sum.apply(&mut a, &mut b), 30);
    assert_eq!(a, 10); // unchanged
    assert_eq!(b, 20); // unchanged
}

#[test]
fn test_bi_mutating_function_chained_modifications() {
    let complex_op = |x: &mut i32, y: &mut i32| {
        *x = *x * 2 + *y;
        *y = *y * 3 - *x;
        *x + *y
    };

    let mut a = 3;
    let mut b = 5;
    let result = complex_op.apply(&mut a, &mut b);
    // a = 3*2 + 5 = 11
    // y = 5*3 - 11 = 15 - 11 = 4
    // result = 11 + 4 = 15
    assert_eq!(result, 15);
    assert_eq!(a, 11);
    assert_eq!(b, 4);
}

// ============================================================================
// Type Conversion Tests
// ============================================================================

#[test]
fn test_conversion_between_different_wrappers() {
    let original = |x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    };

    // Box -> Rc
    let boxed = BoxBiMutatingFunction::new(original);
    let rc = boxed.into_rc();
    let mut a = 10;
    let mut b = 20;
    assert_eq!(rc.apply(&mut a, &mut b), 30);
    assert_eq!(a, 20);
    assert_eq!(b, 10);

    // Test separate Rc and Arc functions
    let rc_func = RcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });
    let mut c = 30;
    let mut d = 40;
    assert_eq!(rc_func.apply(&mut c, &mut d), 70);
    assert_eq!(c, 40);
    assert_eq!(d, 30);

    // Test Arc function separately
    let arc_func = ArcBiMutatingFunction::new(|x: &mut i32, y: &mut i32| {
        let temp = *x;
        *x = *y;
        *y = temp;
        *x + *y
    });
    let mut e = 50;
    let mut f = 60;
    assert_eq!(arc_func.apply(&mut e, &mut f), 110);
    assert_eq!(e, 60);
    assert_eq!(f, 50);
}


// ============================================================================
// Error and Panic Tests
// ============================================================================

#[test]
#[should_panic]
fn test_bi_mutating_function_panic_in_closure() {
    let panic_func = |x: &mut i32, y: &mut i32| {
        if *x < 0 {
            panic!("Negative value not allowed");
        }
        *x + *y
    };

    let mut a = -5;
    let mut b = 10;
    let _ = panic_func.apply(&mut a, &mut b);
}

#[test]
fn test_bi_mutating_function_with_option_modification() {
    let modify_option = |x: &mut Option<i32>, y: &mut Option<i32>| {
        if let (Some(val1), Some(val2)) = (*x, *y) {
            *x = Some(val1 + val2);
            *y = Some(val1 * val2);
            val1 + val2
        } else {
            0
        }
    };

    let mut a = Some(10);
    let mut b = Some(5);
    let result = modify_option.apply(&mut a, &mut b);
    assert_eq!(result, 15);
    assert_eq!(a, Some(15));
    assert_eq!(b, Some(50));

    let mut c = None;
    let mut d = Some(5);
    let result2 = modify_option.apply(&mut c, &mut d);
    assert_eq!(result2, 0);
    assert_eq!(c, None);
    assert_eq!(d, Some(5));
}

// ============================================================================
// Integration with Other Function Types
// ============================================================================

#[test]
fn test_bi_mutating_function_with_function_composition() {
    let add = |x: &mut i32, y: &mut i32| {
        *x += *y;
        *x
    };

    let double = |x: &i32| *x * 2;

    // First apply bi-mutating function, then regular function
    let composed = add.and_then(double);

    let mut a = 10;
    let mut b = 5;
    // add: a = 10+5=15, return 15
    // double: 15*2=30
    assert_eq!(composed.apply(&mut a, &mut b), 30);
    assert_eq!(a, 15);
    assert_eq!(b, 5);
}

// ============================================================================
// Custom BiMutatingFunction Implementation Tests - Test Trait Default Methods
// ============================================================================

#[test]
fn test_custom_bi_mutating_function_default_methods() {
    // Test BiMutatingFunction trait default methods on custom implementation
    #[derive(Debug, Clone)]
    struct CustomBiMutatingFunction {
        multiplier: i32,
    }

    impl BiMutatingFunction<i32, i32, i32> for CustomBiMutatingFunction {
        fn apply(&self, first: &mut i32, second: &mut i32) -> i32 {
            *first *= self.multiplier;
            *second += self.multiplier;
            *first + *second
        }
    }

    let custom_func = CustomBiMutatingFunction { multiplier: 3 };
    let mut a = 2;
    let mut b = 4;

    // Test the apply method
    let result = custom_func.apply(&mut a, &mut b);
    assert_eq!(result, 13); // (2*3) + (4+3) = 6 + 7 = 13
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Reset values
    a = 2;
    b = 4;

    // Test default into_box method
    let boxed = CustomBiMutatingFunction { multiplier: 3 }.into_box();
    let result = boxed.apply(&mut a, &mut b);
    assert_eq!(result, 13); // (2*3) + (4+3) = 6 + 7 = 13
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Reset values
    a = 2;
    b = 4;

    // Test default into_rc method
    let rc = CustomBiMutatingFunction { multiplier: 3 }.into_rc();
    let result = rc.apply(&mut a, &mut b);
    assert_eq!(result, 13);
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Reset values
    a = 2;
    b = 4;

    // Test default into_arc method (requires Send + Sync)
    let arc = CustomBiMutatingFunction { multiplier: 3 }.into_arc();
    let result = arc.apply(&mut a, &mut b);
    assert_eq!(result, 13);
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Reset values
    a = 2;
    b = 4;

    // Test default into_fn method
    let func = CustomBiMutatingFunction { multiplier: 3 }.into_fn();
    let result = func(&mut a, &mut b);
    assert_eq!(result, 13);
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Reset values
    a = 2;
    b = 4;

    // Test default into_once method
    let once = CustomBiMutatingFunction { multiplier: 3 }.into_once();
    let result = once.apply(&mut a, &mut b);
    assert_eq!(result, 13);
    assert_eq!(a, 6);
    assert_eq!(b, 7);
}

#[test]
fn test_custom_bi_mutating_function_clone_default_methods() {
    // Test BiMutatingFunction trait default methods that require Clone
    #[derive(Debug, Clone)]
    struct CloneableBiMutatingFunction {
        factor: i32,
    }

    impl BiMutatingFunction<i32, i32, i32> for CloneableBiMutatingFunction {
        fn apply(&self, first: &mut i32, second: &mut i32) -> i32 {
            *first *= self.factor;
            *second += self.factor;
            *first + *second
        }
    }

    let custom_func = CloneableBiMutatingFunction { factor: 2 };
    let mut a = 3;
    let mut b = 5;

    // Test default to_box method (requires Clone)
    let boxed = custom_func.to_box();
    let result = boxed.apply(&mut a, &mut b);
    assert_eq!(result, 13); // (3*2) + (5+2) = 6 + 7 = 13
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Reset values
    a = 3;
    b = 5;

    // Test default to_rc method (requires Clone)
    let rc = custom_func.to_rc();
    let result = rc.apply(&mut a, &mut b);
    assert_eq!(result, 13);
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Reset values
    a = 3;
    b = 5;

    // Test default to_arc method (requires Clone)
    let arc = custom_func.to_arc();
    let result = arc.apply(&mut a, &mut b);
    assert_eq!(result, 13);
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Reset values
    a = 3;
    b = 5;

    // Test default to_fn method (requires Clone)
    let func = custom_func.to_fn();
    let result = func(&mut a, &mut b);
    assert_eq!(result, 13);
    assert_eq!(a, 6);
    assert_eq!(b, 7);

    // Reset values
    a = 3;
    b = 5;

    // Test default to_once method (requires Clone)
    let once = custom_func.to_once();
    let result = once.apply(&mut a, &mut b);
    assert_eq!(result, 13);
    assert_eq!(a, 6);
    assert_eq!(b, 7);
}

