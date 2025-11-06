/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Comprehensive tests for FunctionOnce trait and BoxFunctionOnce

use prism3_function::{
    BoxFunctionOnce,
    FunctionOnce,
    Predicate,
    RcPredicate,
};

// ============================================================================
// FunctionOnce Trait Tests - Core Functionality
// ============================================================================

#[test]
fn test_function_once_trait_apply() {
    // Test that FunctionOnce trait's apply method works correctly
    let double = |x: &i32| x * 2;
    assert_eq!(double.apply(&21), 42);
}

#[test]
fn test_function_once_trait_apply_with_move() {
    // Test apply with moved value
    let value = String::from("hello");
    let append = move |s: &String| format!("{} {}", s, value);
    assert_eq!(append.apply(&String::from("world")), "world hello");
}

#[test]
fn test_function_once_trait_into_box() {
    // Test conversion from closure to BoxFunctionOnce
    let double = |x: &i32| x * 2;
    let boxed = double.into_box();
    assert_eq!(boxed.apply(&21), 42);
}

#[test]
fn test_function_once_trait_into_fn() {
    // Test conversion to FnOnce closure
    let double = |x: &i32| x * 2;
    let func = double.into_fn();
    assert_eq!(func(&21), 42);
}

#[test]
fn test_function_once_trait_to_box() {
    // Test non-consuming conversion to BoxFunctionOnce
    let double = |x: &i32| x * 2;
    let boxed = double.to_box();
    assert_eq!(boxed.apply(&21), 42);
    // Original closure still usable
    assert_eq!(double.apply(&10), 20);
}

#[test]
fn test_function_once_trait_to_fn() {
    // Test non-consuming conversion to FnOnce closure
    let double = |x: &i32| x * 2;
    let func = double.to_fn();
    assert_eq!(func(&21), 42);
    // Original closure still usable
    assert_eq!(double.apply(&10), 20);
}

// ============================================================================
// BoxFunctionOnce Tests - Constructor and Basic Operations
// ============================================================================

#[test]
fn test_box_function_once_new() {
    // Test BoxFunctionOnce::new with simple closure
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    assert_eq!(double.apply(&21), 42);
}

#[test]
fn test_box_function_once_new_with_move() {
    // Test BoxFunctionOnce::new with moved value
    let data = vec![1, 2, 3];
    let extend = BoxFunctionOnce::new(move |v: &Vec<i32>| {
        let mut result = v.clone();
        result.extend(data);
        result
    });
    let input = vec![0];
    assert_eq!(extend.apply(&input), vec![0, 1, 2, 3]);
}

#[test]
fn test_box_function_once_identity() {
    // Test BoxFunctionOnce::identity
    let identity = BoxFunctionOnce::<i32, i32>::identity();
    assert_eq!(identity.apply(&42), 42);
}

#[test]
fn test_box_function_once_constant() {
    // Test BoxFunctionOnce::constant
    let constant = BoxFunctionOnce::constant("hello");
    assert_eq!(constant.apply(&123), "hello");
}

#[test]
fn test_box_function_once_apply() {
    // Test FunctionOnce trait implementation for BoxFunctionOnce
    let add_one = BoxFunctionOnce::new(|x: &i32| x + 1);
    assert_eq!(add_one.apply(&41), 42);
}

// ============================================================================
// BoxFunctionOnce Tests - Composition Methods
// ============================================================================

#[test]
fn test_box_function_once_and_then() {
    // Test BoxFunctionOnce::and_then composition
    let add_one = BoxFunctionOnce::new(|x: &i32| x + 1);
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let composed = add_one.and_then(double);
    assert_eq!(composed.apply(&5), 12); // (5 + 1) * 2
}

#[test]
fn test_box_function_once_and_then_with_move() {
    // Test and_then with moved values
    let data1 = vec![1, 2];
    let data2 = vec![3, 4];

    let extend1 = BoxFunctionOnce::new(move |v: &Vec<i32>| {
        let mut result = v.clone();
        result.extend(data1);
        result
    });

    let extend2 = BoxFunctionOnce::new(move |v: &Vec<i32>| {
        let mut result = v.clone();
        result.extend(data2);
        result
    });

    let composed = extend1.and_then(extend2);
    let input = vec![0];
    assert_eq!(composed.apply(&input), vec![0, 1, 2, 3, 4]);
}

// ============================================================================
// BoxFunctionOnce Tests - Conditional Execution
// ============================================================================

#[test]
fn test_box_function_once_when_or_else() {
    // Test conditional execution with when/or_else
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let identity = BoxFunctionOnce::<i32, i32>::identity();
    let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
    assert_eq!(conditional.apply(&5), 10);
}

#[test]
fn test_box_function_once_when_or_else_negative() {
    // Test conditional execution with negative value
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let identity = BoxFunctionOnce::<i32, i32>::identity();
    let conditional = double.when(|x: &i32| *x > 0).or_else(identity);
    assert_eq!(conditional.apply(&-5), -5);
}

#[test]
fn test_box_function_once_when_with_closure() {
    // Test when with closure predicate and or_else
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let conditional = double.when(|x: &i32| *x >= 10).or_else(|x: &i32| *x);
    assert_eq!(conditional.apply(&15), 30);
}

#[test]
fn test_box_function_once_when_with_predicate() {
    // Test when with RcPredicate (cloneable)
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let is_positive = RcPredicate::new(|x: &i32| *x > 0);
    let conditional = double
        .when(is_positive.clone())
        .or_else(BoxFunctionOnce::<i32, i32>::identity());

    assert_eq!(conditional.apply(&5), 10);
    assert!(is_positive.test(&3));
}

#[test]
fn test_box_function_once_when_with_move() {
    // Test when with moved values in branches
    let multiplier = 3;
    let double = BoxFunctionOnce::new(move |x: &i32| x * multiplier);
    let negate = BoxFunctionOnce::new(|x: &i32| -(*x));
    let conditional = double.when(|x: &i32| *x > 0).or_else(negate);
    assert_eq!(conditional.apply(&5), 15);
}

// ============================================================================
// BoxFunctionOnce Tests - Type Conversions
// ============================================================================

#[test]
fn test_box_function_once_into_box() {
    // Test BoxFunctionOnce::into_box (should return itself)
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let boxed = double.into_box();
    assert_eq!(boxed.apply(&21), 42);
}

#[test]
fn test_box_function_once_into_fn() {
    // Test BoxFunctionOnce::into_fn conversion
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let func = double.into_fn();
    assert_eq!(func(&21), 42);
}

// ============================================================================
// Edge Cases and Boundary Tests
// ============================================================================

#[test]
fn test_function_once_with_zero() {
    // Test function with zero input
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    assert_eq!(double.apply(&0), 0);
}

#[test]
fn test_function_once_with_negative() {
    // Test function with negative input
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    assert_eq!(double.apply(&-42), -84);
}

#[test]
fn test_function_once_with_max_value() {
    // Test function with maximum value
    let identity = BoxFunctionOnce::<i32, i32>::identity();
    assert_eq!(identity.apply(&i32::MAX), i32::MAX);
}

#[test]
fn test_function_once_with_min_value() {
    // Test function with minimum value
    let identity = BoxFunctionOnce::<i32, i32>::identity();
    assert_eq!(identity.apply(&i32::MIN), i32::MIN);
}

#[test]
fn test_function_once_chain_multiple() {
    // Test chaining multiple functions
    let add_one = BoxFunctionOnce::new(|x: &i32| x + 1);
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let add_ten = BoxFunctionOnce::new(|x: &i32| x + 10);

    let composed = add_one.and_then(double).and_then(add_ten);
    assert_eq!(composed.apply(&5), 22); // ((5 + 1) * 2) + 10
}

#[test]
fn test_function_once_with_string() {
    // Test function with String type
    let to_upper = BoxFunctionOnce::new(|s: &String| s.to_uppercase());
    let input = String::from("hello");
    assert_eq!(to_upper.apply(&input), "HELLO");
}

#[test]
fn test_function_once_with_vec() {
    // Test function with Vec type
    let get_len = BoxFunctionOnce::new(|v: &Vec<i32>| v.len());
    let vec = vec![1, 2, 3, 4, 5];
    assert_eq!(get_len.apply(&vec), 5);
}

#[test]
fn test_function_once_with_option() {
    // Test function with Option type
    let unwrap_or_zero = BoxFunctionOnce::new(|opt: &Option<i32>| opt.unwrap_or(0));
    assert_eq!(unwrap_or_zero.apply(&Some(42)), 42);
}

#[test]
fn test_function_once_with_option_none() {
    // Test function with None
    let unwrap_or_zero = BoxFunctionOnce::new(|opt: &Option<i32>| opt.unwrap_or(0));
    assert_eq!(unwrap_or_zero.apply(&None), 0);
}

#[test]
fn test_conditional_function_once_edge_cases() {
    // Test conditional function with boundary values
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let negate = BoxFunctionOnce::new(|x: &i32| -(*x));
    let conditional = double.when(|x: &i32| *x >= 0).or_else(negate);
    assert_eq!(conditional.apply(&0), 0);
}

#[test]
fn test_function_once_with_moved_vec() {
    // Test function that moves a Vec
    let data = vec![1, 2, 3];
    let func = BoxFunctionOnce::new(move |x: &i32| {
        let mut result = data.clone();
        result.push(*x);
        result
    });
    assert_eq!(func.apply(&4), vec![1, 2, 3, 4]);
}

#[test]
fn test_function_once_with_moved_string() {
    // Test function that moves a String
    let prefix = String::from("Hello, ");
    let func = BoxFunctionOnce::new(move |s: &String| format!("{}{}", prefix, s));
    assert_eq!(func.apply(&String::from("World")), "Hello, World");
}

#[test]
fn test_function_once_with_complex_closure() {
    // Test function with complex closure logic
    let threshold = 10;
    let multiplier = 2;
    let func = BoxFunctionOnce::new(
        move |x: &i32| {
            if *x > threshold {
                x * multiplier
            } else {
                *x
            }
        },
    );
    assert_eq!(func.apply(&15), 30);
}

#[test]
fn test_function_once_with_complex_closure_below_threshold() {
    // Test complex closure with value below threshold
    let threshold = 10;
    let multiplier = 2;
    let func = BoxFunctionOnce::new(
        move |x: &i32| {
            if *x > threshold {
                x * multiplier
            } else {
                *x
            }
        },
    );
    assert_eq!(func.apply(&5), 5);
}

// ============================================================================
// FnFunctionOnceOps Extension Trait Tests
// ============================================================================

#[test]
fn test_fn_function_once_ops_and_then() {
    // Test FnFunctionOnceOps::and_then for closures
    use prism3_function::FnFunctionOnceOps;

    let parse = |s: &String| s.parse::<i32>().unwrap_or(0);
    let double = |x: &i32| x * 2;
    let composed = parse.and_then(double);
    assert_eq!(composed.apply(&String::from("21")), 42);
}

#[test]
fn test_fn_function_once_ops_when() {
    // Test FnFunctionOnceOps::when for closures
    use prism3_function::FnFunctionOnceOps;

    let double = |x: &i32| x * 2;
    let conditional = double.when(|x: &i32| *x > 0).or_else(|x: &i32| -(*x));
    assert_eq!(conditional.apply(&5), 10);
}

#[test]
fn test_fn_function_once_ops_when_negative() {
    // Test FnFunctionOnceOps::when with negative value
    use prism3_function::FnFunctionOnceOps;

    let double = |x: &i32| x * 2;
    let conditional = double.when(|x: &i32| *x > 0).or_else(|x: &i32| -(*x));
    assert_eq!(conditional.apply(&-5), 5);
}

// ============================================================================
// Resource Transfer Tests
// ============================================================================

#[test]
fn test_function_once_resource_transfer() {
    // Test transferring ownership of resources
    let buffer = vec![1, 2, 3];
    let transfer = BoxFunctionOnce::new(move |target: &Vec<i32>| {
        let mut result = target.clone();
        result.extend(buffer);
        result
    });

    let target = vec![0];
    let result = transfer.apply(&target);
    assert_eq!(result, vec![0, 1, 2, 3]);
}

#[test]
fn test_function_once_with_box() {
    // Test function with Box type
    let data = Box::new(42);
    let func = BoxFunctionOnce::new(move |x: &i32| *data + *x);
    assert_eq!(func.apply(&8), 50);
}

#[test]
fn test_function_once_with_rc() {
    // Test function with Rc type
    use std::rc::Rc;
    let data = Rc::new(vec![1, 2, 3]);
    let func = BoxFunctionOnce::new(move |x: &i32| data.len() + (*x as usize));
    assert_eq!(func.apply(&2), 5);
}

// ============================================================================
// FunctionOnce Trait Default Implementation Tests
// ============================================================================

#[cfg(test)]
mod function_once_default_impl_tests {
    use prism3_function::{
        BoxFunctionOnce,
        FunctionOnce,
    };

    /// Custom struct that only implements the core apply method of FunctionOnce trait
    /// All to_xxx_once() methods use default implementation
    struct CustomFunctionOnce {
        multiplier: i32,
    }

    impl FunctionOnce<i32, i32> for CustomFunctionOnce {
        fn apply(self, input: &i32) -> i32 {
            input * self.multiplier
        }
        // Does not override any to_xxx_once() methods, testing default implementations
    }

    /// Cloneable custom one-time function for testing to_xxx_once() methods
    #[derive(Clone)]
    struct CloneableCustomFunctionOnce {
        multiplier: i32,
    }

    impl FunctionOnce<i32, i32> for CloneableCustomFunctionOnce {
        fn apply(self, input: &i32) -> i32 {
            input * self.multiplier
        }
        // Does not override any to_xxx_once() methods, testing default implementations
    }

    #[test]
    fn test_custom_into_box() {
        let custom = CustomFunctionOnce { multiplier: 3 };
        let boxed = custom.into_box();

        assert_eq!(boxed.apply(&14), 42);
    }

    #[test]
    fn test_custom_into_fn() {
        let custom = CustomFunctionOnce { multiplier: 6 };
        let func = custom.into_fn();

        assert_eq!(func(&7), 42);
    }

    #[test]
    fn test_cloneable_to_box() {
        let custom = CloneableCustomFunctionOnce { multiplier: 3 };
        let boxed = custom.to_box();

        assert_eq!(boxed.apply(&14), 42);

        // Original function is still usable (because to_box only borrows)
        assert_eq!(custom.apply(&10), 30);
    }

    #[test]
    fn test_cloneable_to_fn() {
        let custom = CloneableCustomFunctionOnce { multiplier: 6 };
        let func = custom.to_fn();

        assert_eq!(func(&7), 42);

        // Original function is still usable (because to_fn only borrows)
        assert_eq!(custom.apply(&5), 30);
    }

    #[test]
    fn test_custom_chained_conversions() {
        let custom = CustomFunctionOnce { multiplier: 2 };
        let boxed: BoxFunctionOnce<i32, i32> = custom.into_box();

        assert_eq!(boxed.apply(&21), 42);
    }

    #[test]
    fn test_custom_composition() {
        let custom1 = CloneableCustomFunctionOnce { multiplier: 2 };
        let custom2 = CloneableCustomFunctionOnce { multiplier: 3 };

        let composed = custom1.to_box().and_then(custom2.to_box());
        assert_eq!(composed.apply(&7), 42); // 7 * 2 = 14, 14 * 3 = 42
    }

    #[test]
    fn test_custom_with_captured_value() {
        let captured = [1, 2, 3];
        let custom = CloneableCustomFunctionOnce { multiplier: 2 };

        let func = BoxFunctionOnce::new(move |x: &i32| {
            let base = custom.apply(x);
            base + captured.len() as i32
        });

        assert_eq!(func.apply(&10), 23); // 10 * 2 + 3
    }
}

// ============================================================================
// FunctionOnce Debug and Display Tests
// ============================================================================

#[test]
fn test_box_function_once_debug_display() {
    // Test Debug and Display for BoxFunctionOnce without name
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let debug_str = format!("{:?}", double);
    assert!(debug_str.contains("BoxFunctionOnce"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));

    let display_str = format!("{}", double);
    assert_eq!(display_str, "BoxFunctionOnce");

    // Test Debug and Display for BoxFunctionOnce with name
    let named_double = BoxFunctionOnce::new_with_name("double", |x: &i32| x * 2);
    let named_debug_str = format!("{:?}", named_double);
    assert!(named_debug_str.contains("BoxFunctionOnce"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));

    let named_display_str = format!("{}", named_double);
    assert_eq!(named_display_str, "BoxFunctionOnce(double)");
}

// ============================================================================
// FunctionOnce Name Management Tests
// ============================================================================

#[test]
fn test_box_function_once_name_methods() {
    // Test new_with_name, name(), and set_name()
    let mut double = BoxFunctionOnce::new_with_name("double_func_once", |x: &i32| x * 2);

    // Test name() returns the initial name
    assert_eq!(double.name(), Some("double_func_once"));

    // Test set_name() changes the name
    double.set_name("modified_double_once");
    assert_eq!(double.name(), Some("modified_double_once"));

    // Test that function still works after name change
    assert_eq!(double.apply(&5), 10);
}

// ============================================================================
// ConditionalFunctionOnce Debug and Display Tests
// ============================================================================

#[test]
fn test_box_conditional_function_once_debug_display() {
    // Test Debug and Display for BoxConditionalFunctionOnce without name
    let double = BoxFunctionOnce::new(|x: &i32| x * 2);
    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("BoxConditionalFunctionOnce"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));
    assert!(debug_str.contains("predicate"));

    let display_str = format!("{}", conditional);
    assert!(display_str.starts_with("BoxConditionalFunctionOnce("));
    assert!(display_str.contains("BoxFunctionOnce"));
    assert!(display_str.contains("BoxPredicate"));
    assert!(display_str.ends_with(")"));

    // Test Debug and Display for BoxConditionalFunctionOnce with name
    let triple = BoxFunctionOnce::new_with_name("triple_func_once", |x: &i32| x * 3);
    let named_conditional = triple.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(named_debug_str.contains("BoxConditionalFunctionOnce"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));
    assert!(named_debug_str.contains("predicate"));

    let named_display_str = format!("{}", named_conditional);
    assert!(named_display_str.starts_with("BoxConditionalFunctionOnce("));
    assert!(named_display_str.contains("BoxFunctionOnce(triple_func_once)"));
    assert!(named_display_str.contains("BoxPredicate"));
    assert!(named_display_str.ends_with(")"));
}
