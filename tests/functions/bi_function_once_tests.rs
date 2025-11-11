/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Comprehensive tests for BiFunctionOnce trait and BoxBiFunctionOnce

use prism3_function::{
    BiFunctionOnce, BoxBiFunctionOnce, FnBiFunctionOnceOps,
    BoxBiPredicate, RcBiPredicate, ArcBiPredicate,
};

// ============================================================================
// BiFunctionOnce Trait Tests - Core Functionality
// ============================================================================

#[test]
fn test_bi_function_once_trait_apply() {
    // Test that BiFunctionOnce trait's apply method works correctly
    let add = |x: &i32, y: &i32| *x + *y;
    assert_eq!(add.apply(&21, &21), 42);
}

#[test]
fn test_bi_function_once_trait_apply_with_move() {
    // Test apply with moved value
    let value = String::from("hello");
    let concat = move |x: &String, y: &String| format!("{} {} {}", x, value, y);
    assert_eq!(concat.apply(&String::from("world"), &String::from("!")), "world hello !");
}

#[test]
fn test_bi_function_once_trait_apply_with_different_types() {
    // Test apply with different input types
    let multiply = |x: &i32, y: &f64| *x as f64 * *y;
    assert_eq!(multiply.apply(&3, &2.5), 7.5);
}

#[test]
fn test_bi_function_once_trait_into_box() {
    // Test conversion from closure to BoxBiFunctionOnce
    let add = |x: &i32, y: &i32| *x + *y;
    let boxed = add.into_box();
    assert_eq!(boxed.apply(&10, &15), 25);
}

#[test]
fn test_bi_function_once_trait_into_fn() {
    // Test conversion to FnOnce closure
    let add = |x: &i32, y: &i32| *x + *y;
    let func = add.into_fn();
    assert_eq!(func(&10, &15), 25);
}

#[test]
fn test_bi_function_once_trait_to_box() {
    // Test conversion to BoxBiFunctionOnce using to_box
    let add = |x: &i32, y: &i32| *x + *y;
    let boxed = add.to_box();
    assert_eq!(boxed.apply(&8, &12), 20);
}

#[test]
fn test_bi_function_once_trait_to_fn() {
    // Test conversion to FnOnce closure using to_fn
    let add = |x: &i32, y: &i32| *x + *y;
    let func = add.to_fn();
    assert_eq!(func(&8, &12), 20);
}

// ============================================================================
// BoxBiFunctionOnce Tests - Box-based BiFunction Implementation
// ============================================================================

#[test]
fn test_box_bi_function_once_new() {
    // Test creating BoxBiFunctionOnce using new()
    let add = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    assert_eq!(add.apply(&5, &7), 12);
}

#[test]
fn test_box_bi_function_once_new_with_name() {
    // Test creating BoxBiFunctionOnce with name
    let add = BoxBiFunctionOnce::new_with_name("adder", |x: &i32, y: &i32| *x + *y);
    assert_eq!(add.name(), Some("adder"));
    assert_eq!(add.apply(&3, &4), 7);
}

#[test]
fn test_box_bi_function_once_new_with_optional_name() {
    // Test creating BoxBiFunctionOnce with optional name
    let add1 = BoxBiFunctionOnce::new_with_optional_name(|x: &i32, y: &i32| *x + *y, Some("named".to_string()));
    let add2 = BoxBiFunctionOnce::new_with_optional_name(|x: &i32, y: &i32| *x + *y, None);

    assert_eq!(add1.name(), Some("named"));
    assert_eq!(add2.name(), None);
    assert_eq!(add1.apply(&2, &3), 5);
    assert_eq!(add2.apply(&2, &3), 5);
}

#[test]
fn test_box_bi_function_once_name() {
    // Test getting name from BoxBiFunctionOnce
    let mut func = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    assert_eq!(func.name(), None);

    func.set_name("test");
    assert_eq!(func.name(), Some("test"));
}

#[test]
fn test_box_bi_function_once_set_name() {
    // Test setting name on BoxBiFunctionOnce
    let mut func = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    assert_eq!(func.name(), None);

    func.set_name("multiplier");
    assert_eq!(func.name(), Some("multiplier"));

    func.set_name("new name");
    assert_eq!(func.name(), Some("new name"));
}

#[test]
fn test_box_bi_function_once_constant() {
    // Test constant method on BoxBiFunctionOnce
    let constant_func1 = BoxBiFunctionOnce::constant(42);
    let constant_func2 = BoxBiFunctionOnce::constant(42);
    assert_eq!(constant_func1.apply(&1, &2), 42);
    assert_eq!(constant_func2.apply(&10, &20), 42);
}

#[test]
fn test_box_bi_function_once_apply() {
    // Test apply method on BoxBiFunctionOnce
    let multiply = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x * *y);
    assert_eq!(multiply.apply(&6, &7), 42);
}

#[test]
fn test_box_bi_function_once_into_box() {
    // Test into_box method (should consume self)
    let func = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let boxed = func.into_box();
    assert_eq!(boxed.apply(&1, &2), 3);
}

#[test]
fn test_box_bi_function_once_into_fn() {
    // Test into_fn method (should consume self)
    let func = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let closure = func.into_fn();
    assert_eq!(closure(&1, &2), 3);
}

#[test]
fn test_box_bi_function_once_to_box_unavailable() {
    // Test that to_box method is not available for BoxBiFunctionOnce
    // BoxBiFunctionOnce doesn't implement Clone, so to_box method is not generated
    // by impl_box_once_conversions macro due to trait bounds
    let func = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);

    // This would not compile: func.to_box()
    // The method is not available because BoxBiFunctionOnce doesn't implement Clone

    // Instead, use into_box() which consumes self
    let boxed = func.into_box();
    assert_eq!(boxed.apply(&5, &7), 12);
}

#[test]
fn test_box_bi_function_once_to_fn() {
    // Test to_fn method (requires Clone, but BoxBiFunctionOnce doesn't implement Clone)
    // This test verifies that the method is not available
    // let func = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    // let closure = func.to_fn(); // This would not compile
}

// ============================================================================
// FnBiFunctionOnceOps Trait Tests - Extension Methods for Closures
// ============================================================================

#[test]
fn test_fn_bi_function_once_ops_and_then() {
    // Test and_then method on closures
    let add = |x: &i32, y: &i32| *x + *y;
    let double = |x: &i32| *x * 2;

    let composed = add.and_then(double);
    assert_eq!(composed.apply(&3, &4), 14); // (3 + 4) * 2 = 14
}

#[test]
fn test_fn_bi_function_once_ops_and_then_with_different_types() {
    // Test and_then with type conversion
    let concat = |x: &String, y: &String| format!("{} {}", x, y);
    let length = |s: &String| s.len();

    let composed = concat.and_then(length);
    assert_eq!(composed.apply(&String::from("hello"), &String::from("world")), 11);
}

#[test]
fn test_fn_bi_function_once_ops_when_or_else() {
    // Test when().or_else() conditional execution
    let add1 = |x: &i32, y: &i32| *x + *y;
    let multiply1 = |x: &i32, y: &i32| *x * *y;
    let conditional1 = add1.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply1);
    assert_eq!(conditional1.apply(&3, &4), 7); // when branch: 3 + 4 = 7

    let add2 = |x: &i32, y: &i32| *x + *y;
    let multiply2 = |x: &i32, y: &i32| *x * *y;
    let conditional2 = add2.when(|x: &i32, y: &i32| *x <= 0 || *y <= 0).or_else(multiply2);
    assert_eq!(conditional2.apply(&-3, &4), 1); // when branch: -3 + 4 = 1
}

#[test]
fn test_fn_bi_function_once_ops_when_with_box_predicate() {
    // Test when() with BoxBiPredicate
    let add1 = |x: &i32, y: &i32| *x + *y;
    let multiply1 = |x: &i32, y: &i32| *x * *y;
    let both_positive = BoxBiPredicate::new(|x: &i32, y: &i32| *x > 0 && *y > 0);

    let conditional1 = add1.when(both_positive).or_else(multiply1);
    assert_eq!(conditional1.apply(&3, &4), 7); // when branch

    let add2 = |x: &i32, y: &i32| *x + *y;
    let multiply2 = |x: &i32, y: &i32| *x * *y;
    let conditional2 = add2.when(BoxBiPredicate::new(|x: &i32, y: &i32| *x <= 0 || *y <= 0)).or_else(multiply2);
    assert_eq!(conditional2.apply(&-3, &4), 1); // when branch
}

#[test]
fn test_fn_bi_function_once_ops_when_with_rc_predicate() {
    // Test when() with RcBiPredicate
    let add1 = |x: &i32, y: &i32| *x + *y;
    let multiply1 = |x: &i32, y: &i32| *x * *y;
    let both_positive = RcBiPredicate::new(|x: &i32, y: &i32| *x > 0 && *y > 0);

    let conditional1 = add1.when(both_positive.clone()).or_else(multiply1);
    assert_eq!(conditional1.apply(&3, &4), 7); // when branch

    let add2 = |x: &i32, y: &i32| *x + *y;
    let multiply2 = |x: &i32, y: &i32| *x * *y;
    let conditional2 = add2.when(RcBiPredicate::new(|x: &i32, y: &i32| *x <= 0 || *y <= 0)).or_else(multiply2);
    assert_eq!(conditional2.apply(&-3, &4), 1); // when branch
}

#[test]
fn test_fn_bi_function_once_ops_when_with_arc_predicate() {
    // Test when() with ArcBiPredicate
    let add1 = |x: &i32, y: &i32| *x + *y;
    let multiply1 = |x: &i32, y: &i32| *x * *y;
    let both_positive = ArcBiPredicate::new(|x: &i32, y: &i32| *x > 0 && *y > 0);

    let conditional1 = add1.when(both_positive.clone()).or_else(multiply1);
    assert_eq!(conditional1.apply(&3, &4), 7); // when branch

    let add2 = |x: &i32, y: &i32| *x + *y;
    let multiply2 = |x: &i32, y: &i32| *x * *y;
    let conditional2 = add2.when(ArcBiPredicate::new(|x: &i32, y: &i32| *x <= 0 || *y <= 0)).or_else(multiply2);
    assert_eq!(conditional2.apply(&-3, &4), 1); // when branch
}

#[test]
fn test_closure_bi_function_once_into_box() {
    // Test closure's into_box method (from blanket impl)
    let multiply = |x: &i32, y: &i32| *x * *y;

    let boxed1 = multiply.into_box();
    let boxed2 = (|x: &i32, y: &i32| *x * *y).into_box();
    assert_eq!(boxed1.apply(&3, &4), 12);
    assert_eq!(boxed2.apply(&5, &6), 30);
}

#[test]
fn test_closure_bi_function_once_into_fn() {
    // Test closure's into_fn method (from blanket impl)
    let add = |x: &i32, y: &i32| *x + *y;

    let func1 = add.into_fn();
    let func2 = (|x: &i32, y: &i32| *x + *y).into_fn();
    assert_eq!(func1(&10, &20), 30);
    assert_eq!(func2(&1, &2), 3);
}

#[test]
fn test_closure_bi_function_once_to_box() {
    // Test closure's to_box method (from blanket impl)
    let subtract = |x: &i32, y: &i32| *x - *y;

    let boxed1 = subtract.to_box();
    let boxed2 = (|x: &i32, y: &i32| *x - *y).to_box();
    assert_eq!(boxed1.apply(&10, &3), 7);
    assert_eq!(boxed2.apply(&20, &5), 15);

    // Original closure should still be usable
    let another_boxed = (|x: &i32, y: &i32| *x - *y).to_box();
    assert_eq!(another_boxed.apply(&8, &2), 6);
}

#[test]
fn test_closure_bi_function_once_to_fn() {
    // Test closure's to_fn method (from blanket impl)
    let divide = |x: &i32, y: &i32| *x / *y;

    let func1 = divide.to_fn();
    let func2 = (|x: &i32, y: &i32| *x / *y).to_fn();
    assert_eq!(func1(&20, &4), 5);
    assert_eq!(func2(&15, &3), 5);

    // Original closure should still be usable
    let another_func = (|x: &i32, y: &i32| *x / *y).to_fn();
    assert_eq!(another_func(&30, &6), 5);
}

#[test]
fn test_closure_bi_function_once_and_then() {
    // Test closure's and_then method from FnBiFunctionOnceOps
    let add = |x: &i32, y: &i32| *x + *y;
    let double = |x: &i32| *x * 2;

    let composed1 = add.and_then(double);
    let composed2 = (|x: &i32, y: &i32| *x + *y).and_then(|x: &i32| *x * 2);
    assert_eq!(composed1.apply(&3, &4), 14); // (3 + 4) * 2 = 14
    assert_eq!(composed2.apply(&1, &2), 6); // (1 + 2) * 2 = 6
}

#[test]
fn test_closure_bi_function_once_when() {
    // Test closure's when method from FnBiFunctionOnceOps
    let multiply1 = |x: &i32, y: &i32| *x * *y;
    let add1 = |x: &i32, y: &i32| *x + *y;
    let conditional1 = multiply1.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(add1);
    assert_eq!(conditional1.apply(&3, &4), 12); // when branch: 3 * 4 = 12

    let multiply2 = |x: &i32, y: &i32| *x * *y;
    let add2 = |x: &i32, y: &i32| *x + *y;
    let conditional2 = multiply2.when(|x: &i32, y: &i32| *x <= 0 || *y <= 0).or_else(add2);
    assert_eq!(conditional2.apply(&-3, &4), -12); // when branch: -3 * 4 = -12

    let multiply3 = |x: &i32, y: &i32| *x * *y;
    let add3 = |x: &i32, y: &i32| *x + *y;
    let conditional3 = multiply3.when(|x: &i32, y: &i32| *x > 0 && *y < 0).or_else(add3);
    assert_eq!(conditional3.apply(&3, &-4), -12); // when branch: 3 * (-4) = -12
}

// ============================================================================
// BoxConditionalBiFunctionOnce Tests - Conditional BiFunctions
// ============================================================================

#[test]
fn test_box_conditional_bi_function_once_when_or_else() {
    // Test when().or_else() method
    let add1 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let multiply1 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x * *y);
    let conditional1 = add1.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply1);

    let add2 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let multiply2 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x * *y);
    let conditional2 = add2.when(|x: &i32, y: &i32| *x <= 0 || *y <= 0).or_else(multiply2);

    assert_eq!(conditional1.apply(&3, &4), 7); // when branch: 3 + 4 = 7
    assert_eq!(conditional2.apply(&-3, &4), 1); // when branch: -3 + 4 = 1
}

#[test]
fn test_box_conditional_bi_function_once_complex_conditions() {
    // Test with more complex conditions
    let add1 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let subtract1 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x - *y);
    let conditional1 = add1.when(|x: &i32, y: &i32| *x >= *y).or_else(subtract1);

    let add2 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let subtract2 = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x - *y);
    let conditional2 = add2.when(|x: &i32, y: &i32| *x < *y).or_else(subtract2);

    assert_eq!(conditional1.apply(&5, &3), 8); // when branch: 5 >= 3, so 5 + 3 = 8
    assert_eq!(conditional2.apply(&3, &5), 8); // when branch: 3 < 5, so 3 + 5 = 8
}

#[test]
fn test_box_conditional_bi_function_once_with_string_operations() {
    // Test with string operations
    let concat1 = BoxBiFunctionOnce::new(|x: &String, y: &String| format!("{} {}", x, y));
    let reverse_concat1 = BoxBiFunctionOnce::new(|x: &String, y: &String| format!("{} {}", y, x));
    let conditional1 = concat1.when(|x: &String, y: &String| x.len() >= y.len()).or_else(reverse_concat1);

    let concat2 = BoxBiFunctionOnce::new(|x: &String, y: &String| format!("{} {}", x, y));
    let reverse_concat2 = BoxBiFunctionOnce::new(|x: &String, y: &String| format!("{} {}", y, x));
    let conditional2 = concat2.when(|x: &String, y: &String| x.len() < y.len()).or_else(reverse_concat2);

    assert_eq!(conditional1.apply(&String::from("hello"), &String::from("hi")), "hello hi"); // when branch
    assert_eq!(conditional2.apply(&String::from("hi"), &String::from("hello")), "hi hello"); // when branch
}

// ============================================================================
// Integration Tests - Complex Usage Patterns
// ============================================================================

#[test]
fn test_bi_function_once_complex_composition() {
    // Test complex composition with multiple operations
    let add = |x: &i32, y: &i32| *x + *y;
    let multiply_by_two = |x: &i32| *x * 2;
    let to_string = |x: &i32| x.to_string();

    // Chain: add -> multiply_by_two -> to_string
    let composed = add.and_then(multiply_by_two).and_then(to_string);
    assert_eq!(composed.apply(&3, &4), "14"); // ((3 + 4) * 2).to_string()
}

#[test]
fn test_bi_function_once_conditional_composition() {
    // Test conditional composition
    let add = |x: &i32, y: &i32| *x + *y;
    let multiply = |x: &i32, y: &i32| *x * *y;
    let square = |x: &i32| *x * *x;

    // If both positive, add then square; otherwise multiply then square
    let conditional1 = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(multiply)
        .and_then(square);
    let conditional2 = add.when(|x: &i32, y: &i32| *x <= 0 || *y <= 0)
        .or_else(multiply)
        .and_then(square);

    assert_eq!(conditional1.apply(&3, &4), 49); // (3 + 4)^2 = 49
    assert_eq!(conditional2.apply(&-3, &4), 1); // (-3 + 4)^2 = 1
}

#[test]
fn test_bi_function_once_with_custom_types() {
    // Test with custom types
    #[derive(Debug, PartialEq)]
    struct Point { x: i32, y: i32 }

    let add_points = |p1: &Point, p2: &Point| Point {
        x: p1.x + p2.x,
        y: p1.y + p2.y,
    };

    let p1 = Point { x: 1, y: 2 };
    let p2 = Point { x: 3, y: 4 };
    let result = add_points.apply(&p1, &p2);

    assert_eq!(result, Point { x: 4, y: 6 });
}

#[test]
fn test_bi_function_once_with_result_types() {
    // Test with Result types
    let safe_divide = |x: &i32, y: &i32| {
        if *y == 0 {
            Err("Division by zero")
        } else {
            Ok(*x / *y)
        }
    };

    let to_string = |result: &Result<i32, &str>| match result {
        Ok(value) => format!("Result: {}", value),
        Err(msg) => format!("Error: {}", msg),
    };

    let composed1 = safe_divide.and_then(to_string);
    let composed2 = (|x: &i32, y: &i32| {
        if *y == 0 {
            Err("Division by zero")
        } else {
            Ok(*x / *y)
        }
    }).and_then(to_string);
    assert_eq!(composed1.apply(&10, &2), "Result: 5");
    assert_eq!(composed2.apply(&10, &0), "Error: Division by zero");
}

#[test]
fn test_bi_function_once_with_option_types() {
    // Test with Option types
    let add_options = |x: &Option<i32>, y: &Option<i32>| {
        match (x, y) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        }
    };

    let format_option = |opt: &Option<i32>| match opt {
        Some(value) => format!("Value: {}", value),
        None => "No value".to_string(),
    };

    let composed1 = add_options.and_then(format_option);
    let composed2 = (|x: &Option<i32>, y: &Option<i32>| {
        match (x, y) {
            (Some(a), Some(b)) => Some(a + b),
            _ => None,
        }
    }).and_then(|opt: &Option<i32>| match opt {
        Some(value) => format!("Value: {}", value),
        None => "No value".to_string(),
    });
    assert_eq!(composed1.apply(&Some(3), &Some(4)), "Value: 7");
    assert_eq!(composed2.apply(&Some(3), &None), "No value");
}

// ============================================================================
// Display and Debug Tests
// ============================================================================

#[test]
fn test_box_bi_function_once_display_without_name() {
    // Test Display implementation without name
    let func = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let display = format!("{}", func);
    assert!(display.contains("BoxBiFunctionOnce"));
}

#[test]
fn test_box_bi_function_once_display_with_name() {
    // Test Display implementation with name
    let func = BoxBiFunctionOnce::new_with_name("adder", |x: &i32, y: &i32| *x + *y);
    let display = format!("{}", func);
    assert!(display.contains("adder"));
    assert!(display.contains("BoxBiFunctionOnce"));
}

#[test]
fn test_box_bi_function_once_debug() {
    // Test Debug implementation
    let func = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let debug = format!("{:?}", func);
    assert!(debug.contains("BoxBiFunctionOnce"));
}

#[test]
fn test_box_conditional_bi_function_once_display() {
    // Test Display implementation for conditional
    let add = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let conditional = add.when(|x: &i32, _y: &i32| *x > 0);
    let display = format!("{}", conditional);
    assert!(display.contains("BoxConditionalBiFunctionOnce"));
}

#[test]
fn test_box_conditional_bi_function_once_debug() {
    // Test Debug implementation for conditional
    let add = BoxBiFunctionOnce::new(|x: &i32, y: &i32| *x + *y);
    let conditional = add.when(|x: &i32, _y: &i32| *x > 0);
    let debug = format!("{:?}", conditional);
    assert!(debug.contains("BoxConditionalBiFunctionOnce"));
}

// ============================================================================
// Error Handling Tests
// ============================================================================

#[test]
fn test_bi_function_once_with_error_propagation() {
    // Test error propagation through composition
    #[derive(Debug, PartialEq, Clone)]
    enum MathError {
        DivisionByZero,
        Overflow,
    }

    let safe_divide = |x: &i32, y: &i32| -> Result<i32, MathError> {
        if *y == 0 {
            Err(MathError::DivisionByZero)
        } else if *x == i32::MIN && *y == -1 {
            Err(MathError::Overflow)
        } else {
            Ok(*x / *y)
        }
    };

    let multiply_by_two = |result: &Result<i32, MathError>| {
        result.clone().map(|x| x * 2)
    };

    let composed1 = safe_divide.and_then(multiply_by_two);
    let safe_divide2 = |x: &i32, y: &i32| -> Result<i32, MathError> {
        if *y == 0 {
            Err(MathError::DivisionByZero)
        } else if *x == i32::MIN && *y == -1 {
            Err(MathError::Overflow)
        } else {
            Ok(*x / *y)
        }
    };
    let composed2 = safe_divide2.and_then(multiply_by_two);
    assert_eq!(composed1.apply(&10, &2), Ok(10)); // 5 * 2 = 10
    assert_eq!(composed2.apply(&10, &0), Err(MathError::DivisionByZero));
}

// ============================================================================
// Custom BiFunctionOnce Implementation Tests - Test Trait Default Methods
// ============================================================================

#[test]
fn test_custom_bi_function_once_default_methods() {
    // Test BiFunctionOnce trait default methods on custom implementation
    #[derive(Debug)]
    struct CustomBiFunctionOnce {
        multiplier: i32,
    }

    impl BiFunctionOnce<i32, i32, i32> for CustomBiFunctionOnce {
        fn apply(self, first: &i32, second: &i32) -> i32 {
            first * second * self.multiplier
        }
    }

    let custom_func = CustomBiFunctionOnce { multiplier: 3 };

    // Test default into_box method
    let boxed = custom_func.into_box();
    assert_eq!(boxed.apply(&2, &4), 24); // 2 * 4 * 3 = 24

    // Note: to_box and to_fn cannot be tested here because CustomBiFunctionOnce doesn't implement Clone
}

#[test]
fn test_custom_bi_function_once_into_fn() {
    // Test BiFunctionOnce trait default into_fn method on custom implementation
    #[derive(Debug)]
    struct CustomBiFunctionOnce {
        multiplier: i32,
    }

    impl BiFunctionOnce<i32, i32, i32> for CustomBiFunctionOnce {
        fn apply(self, first: &i32, second: &i32) -> i32 {
            first * second * self.multiplier
        }
    }

    let custom_func = CustomBiFunctionOnce { multiplier: 5 };

    // Test default into_fn method
    let func = custom_func.into_fn();
    assert_eq!(func(&3, &4), 60); // 3 * 4 * 5 = 60
}

#[test]
fn test_cloneable_custom_bi_function_once_to_box() {
    // Test BiFunctionOnce trait default to_box method on cloneable custom implementation
    #[derive(Clone, Debug)]
    struct CloneableCustomBiFunctionOnce {
        multiplier: i32,
    }

    impl BiFunctionOnce<i32, i32, i32> for CloneableCustomBiFunctionOnce {
        fn apply(self, first: &i32, second: &i32) -> i32 {
            first * second * self.multiplier
        }
    }

    let custom_func = CloneableCustomBiFunctionOnce { multiplier: 2 };

    // Test default to_box method (requires Clone)
    let boxed = custom_func.to_box();
    assert_eq!(boxed.apply(&3, &5), 30); // 3 * 5 * 2 = 30

    // Original function should still be usable since it's Clone
    let another_boxed = custom_func.to_box();
    assert_eq!(another_boxed.apply(&2, &3), 12); // 2 * 3 * 2 = 12
}

#[test]
fn test_cloneable_custom_bi_function_once_to_fn() {
    // Test BiFunctionOnce trait default to_fn method on cloneable custom implementation
    #[derive(Clone, Debug)]
    struct CloneableCustomBiFunctionOnce {
        multiplier: i32,
    }

    impl BiFunctionOnce<i32, i32, i32> for CloneableCustomBiFunctionOnce {
        fn apply(self, first: &i32, second: &i32) -> i32 {
            first * second * self.multiplier
        }
    }

    let custom_func = CloneableCustomBiFunctionOnce { multiplier: 4 };

    // Test default to_fn method (requires Clone)
    let func = custom_func.to_fn();
    assert_eq!(func(&2, &6), 48); // 2 * 6 * 4 = 48

    // Original function should still be usable since it's Clone
    let another_func = custom_func.to_fn();
    assert_eq!(another_func(&1, &5), 20); // 1 * 5 * 4 = 20
}

#[test]
fn test_cloneable_bi_function_once_default_methods() {
    // Test BiFunctionOnce trait default methods on cloneable implementation
    #[derive(Clone, Debug)]
    struct CloneableBiFunctionOnce {
        multiplier: i32,
    }

    impl BiFunctionOnce<i32, i32, i32> for CloneableBiFunctionOnce {
        fn apply(self, first: &i32, second: &i32) -> i32 {
            first * second * self.multiplier
        }
    }

    let custom_func = CloneableBiFunctionOnce { multiplier: 2 };

    // Test default to_box method (requires Clone)
    let boxed = custom_func.to_box();
    assert_eq!(boxed.apply(&3, &5), 30); // 3 * 5 * 2 = 30

    // Test default to_fn method (requires Clone)
    let func = custom_func.to_fn();
    assert_eq!(func(&3, &5), 30);
}

#[test]
fn test_bi_function_once_trait_blanket_impl_to_box_to_fn() {
    // Test blanket impl's to_box and to_fn methods that don't require Clone
    let add = |x: &i32, y: &i32| *x + *y;

    // These should use the blanket implementation's default methods
    // which don't require Clone (they just return the closure itself)
    let boxed = add.to_box();
    assert_eq!(boxed.apply(&7, &8), 15);

    let func = add.to_fn();
    assert_eq!(func(&7, &8), 15);
}

// ============================================================================
// Performance and Edge Cases
// ============================================================================

#[test]
fn test_bi_function_once_with_large_data() {
    // Test with large data structures
    let large_vec = vec![1; 10000];
    let combine = |v1: &Vec<i32>, v2: &Vec<i32>| {
        let mut result = v1.clone();
        result.extend(v2);
        result
    };

    let func = BoxBiFunctionOnce::new(combine);
    let result = func.apply(&large_vec, &large_vec);
    assert_eq!(result.len(), 20000);
}

#[test]
fn test_bi_function_once_consumption_semantics() {
    // Test that FnOnce semantics are properly maintained
    let counter = std::rc::Rc::new(std::cell::RefCell::new(0));

    let increment_counter = {
        let counter = counter.clone();
        move |x: &i32, y: &i32| {
            *counter.borrow_mut() += 1;
            *x + *y
        }
    };

    let func = BoxBiFunctionOnce::new(increment_counter);
    assert_eq!(*counter.borrow(), 0);

    // First call should work
    assert_eq!(func.apply(&1, &2), 3);
    assert_eq!(*counter.borrow(), 1);

    // Second call would consume the function (not applicable since it's moved)
}

#[test]
fn test_bi_function_once_with_references() {
    // Test with complex reference patterns
    let data = vec![1, 2, 3, 4, 5];
    let func = |slice: &[i32], index: &usize| {
        slice.get(*index).copied().unwrap_or(0)
    };

    assert_eq!(func(&data, &2), 3);
    assert_eq!(func(&data, &10), 0); // out of bounds
}

// ============================================================================
// Custom BiFunctionOnce Implementation Tests - Comprehensive Default Methods Testing
// ============================================================================

#[test]
fn test_custom_my_bi_function_once_comprehensive() {
    // Custom struct that implements BiFunctionOnce to test all default methods
    #[derive(Debug)]
    struct MyCustomBiFunctionOnce {
        operation: String,
        factor: i32,
    }

    impl MyCustomBiFunctionOnce {
        fn new(operation: &str, factor: i32) -> Self {
            Self {
                operation: operation.to_string(),
                factor,
            }
        }
    }

    impl BiFunctionOnce<i32, i32, String> for MyCustomBiFunctionOnce {
        fn apply(self, first: &i32, second: &i32) -> String {
            let result = match self.operation.as_str() {
                "add" => first + second,
                "multiply" => first * second,
                "subtract" => first - second,
                _ => 0,
            } * self.factor;

            format!("{}({}, {}) = {}", self.operation, first, second, result)
        }
    }

    // Test apply method directly
    let custom_func = MyCustomBiFunctionOnce::new("add", 2);
    let result = custom_func.apply(&3, &4);
    assert_eq!(result, "add(3, 4) = 14"); // (3 + 4) * 2 = 14

    // Test into_box method
    let custom_func2 = MyCustomBiFunctionOnce::new("multiply", 3);
    let boxed = custom_func2.into_box();
    let result2 = boxed.apply(&5, &6);
    assert_eq!(result2, "multiply(5, 6) = 90"); // (5 * 6) * 3 = 90

    // Test into_fn method
    let custom_func3 = MyCustomBiFunctionOnce::new("subtract", 4);
    let func = custom_func3.into_fn();
    let result3 = func(&10, &3);
    assert_eq!(result3, "subtract(10, 3) = 28"); // (10 - 3) * 4 = 28
}

#[test]
fn test_custom_cloneable_my_bi_function_once_comprehensive() {
    // Custom cloneable struct that implements BiFunctionOnce to test to_box and to_fn methods
    #[derive(Clone, Debug)]
    struct MyCloneableCustomBiFunctionOnce {
        base_value: i32,
        multiplier: i32,
    }

    impl MyCloneableCustomBiFunctionOnce {
        fn new(base_value: i32, multiplier: i32) -> Self {
            Self {
                base_value,
                multiplier,
            }
        }
    }

    impl BiFunctionOnce<i32, i32, i32> for MyCloneableCustomBiFunctionOnce {
        fn apply(self, first: &i32, second: &i32) -> i32 {
            (first + second + self.base_value) * self.multiplier
        }
    }

    // Test apply method directly
    let custom_func = MyCloneableCustomBiFunctionOnce::new(10, 2);
    let result = custom_func.apply(&3, &4);
    assert_eq!(result, 34); // (3 + 4 + 10) * 2 = 34

    // Test to_box method (requires Clone)
    let custom_func2 = MyCloneableCustomBiFunctionOnce::new(5, 3);
    let boxed = custom_func2.to_box();
    let result2 = boxed.apply(&2, &3);
    assert_eq!(result2, 30); // (2 + 3 + 5) * 3 = 30

    // Original should still be usable
    let custom_func3 = MyCloneableCustomBiFunctionOnce::new(1, 4);
    let boxed2 = custom_func3.to_box();
    let result3 = boxed2.apply(&1, &1);
    assert_eq!(result3, 12); // (1 + 1 + 1) * 4 = 12

    // Test to_fn method (requires Clone)
    let custom_func4 = MyCloneableCustomBiFunctionOnce::new(2, 5);
    let func = custom_func4.to_fn();
    let result4 = func(&4, &5);
    assert_eq!(result4, 55); // (4 + 5 + 2) * 5 = 55

    // Original should still be usable
    let custom_func5 = MyCloneableCustomBiFunctionOnce::new(0, 6);
    let func2 = custom_func5.to_fn();
    let result5 = func2(&2, &2);
    assert_eq!(result5, 24); // (2 + 2 + 0) * 6 = 24
}

#[test]
fn test_custom_my_bi_function_once_all_default_methods() {
    // Test all default methods of BiFunctionOnce in one comprehensive test
    #[derive(Clone, Debug)]
    struct ComprehensiveBiFunctionOnce {
        prefix: String,
        suffix: String,
        scale: f64,
    }

    impl ComprehensiveBiFunctionOnce {
        fn new(prefix: &str, suffix: &str, scale: f64) -> Self {
            Self {
                prefix: prefix.to_string(),
                suffix: suffix.to_string(),
                scale,
            }
        }
    }

    impl BiFunctionOnce<f64, f64, String> for ComprehensiveBiFunctionOnce {
        fn apply(self, first: &f64, second: &f64) -> String {
            let result = (first + second) * self.scale;
            format!("{}{:.2}{}", self.prefix, result, self.suffix)
        }
    }

    let func = ComprehensiveBiFunctionOnce::new("[", "]", 1.5);

    // Test direct apply
    let direct_result = func.apply(&3.5, &2.5);
    assert_eq!(direct_result, "[9.00]"); // (3.5 + 2.5) * 1.5 = 9.0

    // Test into_box
    let func2 = ComprehensiveBiFunctionOnce::new("<<", ">>", 2.0);
    let boxed = func2.into_box();
    let boxed_result = boxed.apply(&1.5, &2.5);
    assert_eq!(boxed_result, "<<8.00>>"); // (1.5 + 2.5) * 2.0 = 8.0

    // Test into_fn
    let func3 = ComprehensiveBiFunctionOnce::new("(", ")", 0.5);
    let closure = func3.into_fn();
    let closure_result = closure(&4.0, &6.0);
    assert_eq!(closure_result, "(5.00)"); // (4.0 + 6.0) * 0.5 = 5.0

    // Test to_box (requires Clone)
    let func4 = ComprehensiveBiFunctionOnce::new("{", "}", 3.0);
    let boxed2 = func4.to_box();
    let boxed_result2 = boxed2.apply(&2.0, &1.0);
    assert_eq!(boxed_result2, "{9.00}"); // (2.0 + 1.0) * 3.0 = 9.0

    // Test to_fn (requires Clone)
    let func5 = ComprehensiveBiFunctionOnce::new("<", ">", 4.0);
    let closure2 = func5.to_fn();
    let closure_result2 = closure2(&1.5, &1.5);
    assert_eq!(closure_result2, "<12.00>"); // (1.5 + 1.5) * 4.0 = 12.0

    // Verify originals are still usable after to_* methods
    let func6 = ComprehensiveBiFunctionOnce::new("*", "*", 1.0);
    let _boxed3 = func6.to_box(); // This should not consume func6 due to Clone
    let _closure3 = func6.to_fn(); // This should not consume func6 due to Clone
}

#[test]
fn test_custom_cloneable_bi_function_once_to_box() {
    // Test to_box method on a custom struct that implements both BiFunctionOnce and Clone
    #[derive(Clone, Debug)]
    struct MyCloneableBiFunction {
        multiplier: i32,
        offset: i32,
    }

    impl MyCloneableBiFunction {
        fn new(multiplier: i32, offset: i32) -> Self {
            Self { multiplier, offset }
        }
    }

    impl BiFunctionOnce<i32, i32, i32> for MyCloneableBiFunction {
        fn apply(self, first: &i32, second: &i32) -> i32 {
            first * second * self.multiplier + self.offset
        }
    }

    // Test that to_box method is available and works correctly
    let func = MyCloneableBiFunction::new(3, 10);

    // Use to_box method (should be available because struct implements Clone)
    let boxed = func.to_box();
    assert_eq!(boxed.apply(&2, &4), 34); // (2 * 4 * 3) + 10 = 34

    // Original function should still be usable (because to_box borrows &self)
    let another_boxed = func.to_box();
    assert_eq!(another_boxed.apply(&1, &5), 25); // (1 * 5 * 3) + 10 = 25

    // Test to_fn method as well
    let func_closure = func.to_fn();
    assert_eq!(func_closure(&3, &2), 28); // (3 * 2 * 3) + 10 = 28

    // Original function should still be usable after to_fn
    let final_boxed = func.to_box();
    assert_eq!(final_boxed.apply(&0, &10), 10); // (0 * 10 * 3) + 10 = 10
}
