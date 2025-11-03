/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Unit tests for MutatingFunctionOnce types (one-time FnOnce(&mut T) -> R)

use prism3_function::{
    BoxMutatingFunctionOnce,
    FnMutatingFunctionOnceOps,
    MutatingFunctionOnce,
};

// ============================================================================
// MutatingFunctionOnce Default Implementation Tests
// ============================================================================

/// Test struct that implements MutatingFunctionOnce to test default methods
#[derive(Clone)]
struct TestMutatingFunctionOnce {
    multiplier: i32,
}

impl TestMutatingFunctionOnce {
    fn new(multiplier: i32) -> Self {
        TestMutatingFunctionOnce { multiplier }
    }
}

impl MutatingFunctionOnce<i32, i32> for TestMutatingFunctionOnce {
    fn apply(self, input: &mut i32) -> i32 {
        let old_value = *input;
        *input *= self.multiplier;
        old_value
    }
}

#[cfg(test)]
mod test_mutating_function_once_default_impl {
    use super::*;

    #[test]
    fn test_into_box() {
        let func = TestMutatingFunctionOnce::new(2);
        let boxed = func.into_box();

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 5);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_fn() {
        let func = TestMutatingFunctionOnce::new(3);
        let closure = func.into_fn();

        let mut value = 4;
        assert_eq!(closure(&mut value), 4);
        assert_eq!(value, 12);
    }

    #[test]
    fn test_to_box() {
        let func = TestMutatingFunctionOnce::new(2);
        let boxed = func.to_box();

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 5);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_fn() {
        let func = TestMutatingFunctionOnce::new(3);
        let closure = func.to_fn();

        let mut value = 4;
        assert_eq!(closure(&mut value), 4);
        assert_eq!(value, 12);
    }
}

// ============================================================================
// BoxMutatingFunctionOnce Tests
// ============================================================================

#[cfg(test)]
mod test_box_mutating_function_once {
    use super::*;

    #[test]
    fn test_new() {
        let data = vec![1, 2, 3];
        let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        });

        let mut target = vec![0];
        let old_len = func.apply(&mut target);
        assert_eq!(old_len, 1);
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_with_string() {
        let data = String::from(" world");
        let func = BoxMutatingFunctionOnce::new(move |x: &mut String| {
            let old_len = x.len();
            x.push_str(&data);
            old_len
        });

        let mut target = String::from("hello");
        let old_len = func.apply(&mut target);
        assert_eq!(old_len, 5);
        assert_eq!(target, "hello world");
    }

    #[test]
    fn test_and_then() {
        let data1 = vec![1, 2];
        let data2 = vec![3, 4];

        let chained = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
            x.extend(data1);
            x.len()
        })
        .and_then(move |len: &usize| {
            // First function returned the length, now we can use it
            *len + data2.len()
        });

        let mut target = vec![0];
        let final_len = chained.apply(&mut target);
        assert_eq!(final_len, 5); // 3 (target.len() after extend) + 2 (data2.len())
        assert_eq!(target, vec![0, 1, 2]);
    }

    #[test]
    fn test_and_then_multiple_chains() {
        let data1 = vec![1, 2];
        let data2 = vec![3, 4];
        let data3 = vec![5, 6];

        let chained = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
            x.extend(data1);
            x.len()
        })
        .and_then(move |len: &usize| {
            // First chain: add data2 length to current length
            *len + data2.len()
        })
        .and_then(move |len: &usize| {
            // Second chain: add data3 length to current length
            *len + data3.len()
        });

        let mut target = vec![0];
        let final_len = chained.apply(&mut target);
        assert_eq!(final_len, 7); // 3 (initial len) + 2 (data2) + 2 (data3)
        assert_eq!(target, vec![0, 1, 2]);
    }

    #[test]
    fn test_identity() {
        let identity = BoxMutatingFunctionOnce::<i32, i32>::identity();
        let mut value = 42;
        let result = identity.apply(&mut value);
        assert_eq!(result, 42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_map() {
        let data = vec![1, 2, 3];
        let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        });
        let mapped = func.and_then::<String, _>(|old_len: &usize| format!("Old length: {}", *old_len));

        let mut target = vec![0];
        let result = mapped.apply(&mut target);
        assert_eq!(result, "Old length: 1");
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_validation_pattern() {
        struct Data {
            value: i32,
        }

        let validator = BoxMutatingFunctionOnce::new(|data: &mut Data| {
            if data.value < 0 {
                data.value = 0;
                Err("Fixed negative value")
            } else {
                Ok("Valid")
            }
        });

        let mut data = Data { value: -5 };
        let result = validator.apply(&mut data);
        assert_eq!(data.value, 0);
        assert!(result.is_err());
    }

    #[test]
    fn test_resource_transfer() {
        let resource = vec![1, 2, 3, 4, 5];
        let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
            let old_sum: i32 = x.iter().sum();
            x.extend(resource);
            old_sum
        });

        let mut target = vec![10, 20];
        let old_sum = func.apply(&mut target);
        assert_eq!(old_sum, 30);
        assert_eq!(target, vec![10, 20, 1, 2, 3, 4, 5]);
    }
}

// ============================================================================
// Closure Tests
// ============================================================================

#[cfg(test)]
mod test_closure {
    use super::*;

    #[test]
    fn test_closure_implements_trait() {
        let data = vec![1, 2, 3];
        let closure = move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        };

        let mut target = vec![0];
        let old_len = closure.apply(&mut target);
        assert_eq!(old_len, 1);
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_closure_and_then() {
        let data1 = vec![1, 2];
        let data2 = vec![3, 4];

        let chained = (move |x: &mut Vec<i32>| {
            x.extend(data1);
            x.len() // returns usize
        })
        .and_then(move |len: &usize| {
            // Calculate based on the length returned by the previous function
            *len + data2.len()
        });

        let mut target = vec![0];
        let final_len = chained.apply(&mut target);
        assert_eq!(final_len, 5); // 2 (from data1) + 2 (data2.len()) + 1 (original len)
        assert_eq!(target, vec![0, 1, 2]);
    }

    #[test]
    fn test_closure_map() {
        let data = vec![1, 2, 3];
        let mapped = (move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        })
        .and_then::<String, _>(|old_len: &usize| format!("Old length: {}", *old_len));

        let mut target = vec![0];
        let result = mapped.apply(&mut target);
        assert_eq!(result, "Old length: 1");
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_closure_into_box() {
        let data = vec![1, 2, 3];
        let closure = move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        };
        let box_func = closure.into_box();

        let mut target = vec![0];
        let old_len = box_func.apply(&mut target);
        assert_eq!(old_len, 1);
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_closure_into_fn() {
        let data = vec![1, 2, 3];
        let closure = move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        };
        let fn_closure = closure.into_fn();

        let mut target = vec![0];
        let old_len = fn_closure(&mut target);
        assert_eq!(old_len, 1);
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_closure_to_box() {
        let data = vec![1, 2, 3];
        let closure = move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        };
        let box_func = closure.to_box();

        let mut target = vec![0];
        let old_len = box_func.apply(&mut target);
        assert_eq!(old_len, 1);
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_closure_to_fn() {
        let data = vec![1, 2, 3];
        let closure = move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        };
        let fn_closure = closure.to_fn();

        let mut target = vec![0];
        let old_len = fn_closure(&mut target);
        assert_eq!(old_len, 1);
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_move_semantics() {
        let data = vec![1, 2, 3];
        let closure = move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data); // data is moved into closure
            old_len
        };
        // data is no longer accessible here

        let mut target = vec![0];
        let old_len = closure.apply(&mut target);
        assert_eq!(old_len, 1);
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_into_box() {
        let data = vec![1, 2, 3];
        let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        });

        let box_func = func.into_box();

        let mut target = vec![0];
        let old_len = box_func.apply(&mut target);
        assert_eq!(old_len, 1);
        assert_eq!(target, vec![0, 1, 2, 3]);
    }

    #[test]
    fn test_into_fn() {
        let data = vec![1, 2, 3];
        let func = BoxMutatingFunctionOnce::new(move |x: &mut Vec<i32>| {
            let old_len = x.len();
            x.extend(data);
            old_len
        });

        let closure = func.into_fn();

        let mut target = vec![0];
        let old_len = closure(&mut target);
        assert_eq!(old_len, 1);
        assert_eq!(target, vec![0, 1, 2, 3]);
    }
}

// ============================================================================
// MutatingFunctionOnce Debug and Display Tests
// ============================================================================

#[test]
fn test_box_mutating_function_once_debug_display() {
    // Test Debug and Display for BoxMutatingFunctionOnce without name
    let double = BoxMutatingFunctionOnce::new(|x: &mut i32| *x * 2);
    let debug_str = format!("{:?}", double);
    assert!(debug_str.contains("BoxMutatingFunctionOnce"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));

    let display_str = format!("{}", double);
    assert_eq!(display_str, "BoxMutatingFunctionOnce");

    // Test Debug and Display for BoxMutatingFunctionOnce with name
    let named_double = BoxMutatingFunctionOnce::new_with_name("mutating_once_double", |x: &mut i32| *x * 2);
    let named_debug_str = format!("{:?}", named_double);
    assert!(named_debug_str.contains("BoxMutatingFunctionOnce"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));

    let named_display_str = format!("{}", named_double);
    assert_eq!(named_display_str, "BoxMutatingFunctionOnce(mutating_once_double)");
}

// ============================================================================
// MutatingFunctionOnce Name Management Tests
// ============================================================================

#[test]
fn test_box_mutating_function_once_name_methods() {
    // Test new_with_name, name(), and set_name()
    let mut double = BoxMutatingFunctionOnce::new_with_name("box_mutating_once_func", |x: &mut i32| {
        *x = *x * 2;
        *x
    });

    // Test name() returns the initial name
    assert_eq!(double.name(), Some("box_mutating_once_func"));

    // Test set_name() changes the name
    double.set_name("modified_box_mutating_once");
    assert_eq!(double.name(), Some("modified_box_mutating_once"));

    // Test that function still works after name change
    let mut value = 5;
    assert_eq!(double.apply(&mut value), 10);
    assert_eq!(value, 10);
}

// ============================================================================
// ConditionalMutatingFunctionOnce Debug and Display Tests
// ============================================================================

#[test]
fn test_box_conditional_mutating_function_once_debug_display() {
    // Test Debug and Display for BoxConditionalMutatingFunctionOnce without name
    let double = BoxMutatingFunctionOnce::new(|x: &mut i32| *x * 2);
    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("BoxConditionalMutatingFunctionOnce"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));
    assert!(debug_str.contains("predicate"));

    let display_str = format!("{}", conditional);
    assert!(display_str.starts_with("BoxConditionalMutatingFunctionOnce("));
    assert!(display_str.contains("BoxMutatingFunctionOnce"));
    assert!(display_str.contains("BoxPredicate"));
    assert!(display_str.ends_with(")"));

    // Test Debug and Display for BoxConditionalMutatingFunctionOnce with name
    let triple = BoxMutatingFunctionOnce::new_with_name("triple_mutating_once_func", |x: &mut i32| *x * 3);
    let named_conditional = triple.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(named_debug_str.contains("BoxConditionalMutatingFunctionOnce"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));
    assert!(named_debug_str.contains("predicate"));

    let named_display_str = format!("{}", named_conditional);
    assert!(named_display_str.starts_with("BoxConditionalMutatingFunctionOnce("));
    assert!(named_display_str.contains("BoxMutatingFunctionOnce(triple_mutating_once_func)"));
    assert!(named_display_str.contains("BoxPredicate"));
    assert!(named_display_str.ends_with(")"));
}
