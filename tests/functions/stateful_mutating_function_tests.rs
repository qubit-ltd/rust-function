/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Unit tests for StatefulMutatingFunction types (stateful FnMut(&mut T) ->
//! R)

use prism3_function::{
    ArcStatefulMutatingFunction,
    BoxMutatingFunctionOnce,
    BoxStatefulMutatingFunction,
    FnStatefulMutatingFunctionOps,
    MutatingFunctionOnce,
    RcStatefulMutatingFunction,
    StatefulMutatingFunction,
};
use std::cell::RefCell;
use std::rc::Rc;

// ============================================================================
// StatefulMutatingFunction Default Implementation Tests
// ============================================================================

/// Test struct that implements StatefulMutatingFunction to test default methods
struct TestStatefulMutatingFunction {
    multiplier: i32,
}

impl TestStatefulMutatingFunction {
    fn new(multiplier: i32) -> Self {
        TestStatefulMutatingFunction { multiplier }
    }
}

impl StatefulMutatingFunction<i32, i32> for TestStatefulMutatingFunction {
    fn apply(&mut self, input: &mut i32) -> i32 {
        let old_value = *input;
        *input *= self.multiplier;
        old_value
    }
}

impl Clone for TestStatefulMutatingFunction {
    fn clone(&self) -> Self {
        TestStatefulMutatingFunction {
            multiplier: self.multiplier,
        }
    }
}

#[cfg(test)]
mod test_stateful_mutating_function_default_impl {
    use super::*;

    #[test]
    fn test_into_box() {
        let func = TestStatefulMutatingFunction::new(2);
        let mut boxed = func.into_box();

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 5);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_rc() {
        let func = TestStatefulMutatingFunction::new(3);
        let mut rc = func.into_rc();

        let mut value = 4;
        assert_eq!(rc.apply(&mut value), 4);
        assert_eq!(value, 12);
    }

    #[test]
    fn test_into_arc() {
        let func = TestStatefulMutatingFunction::new(4);
        let mut arc = func.into_arc();

        let mut value = 3;
        assert_eq!(arc.apply(&mut value), 3);
        assert_eq!(value, 12);
    }

    #[test]
    fn test_into_fn() {
        let func = TestStatefulMutatingFunction::new(5);
        let mut closure = func.into_fn();

        let mut value = 2;
        assert_eq!(closure(&mut value), 2);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_box() {
        let mut func = TestStatefulMutatingFunction::new(2);
        let mut boxed = func.to_box();

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 5);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut value2 = 3;
        assert_eq!(func.apply(&mut value2), 3);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_rc() {
        let mut func = TestStatefulMutatingFunction::new(3);
        let mut rc = func.to_rc();

        let mut value = 4;
        assert_eq!(rc.apply(&mut value), 4);
        assert_eq!(value, 12);

        // Original should still be usable since it was cloned
        let mut value2 = 2;
        assert_eq!(func.apply(&mut value2), 2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_arc() {
        let mut func = TestStatefulMutatingFunction::new(4);
        let mut arc = func.to_arc();

        let mut value = 3;
        assert_eq!(arc.apply(&mut value), 3);
        assert_eq!(value, 12);

        // Original should still be usable since it was cloned
        let mut value2 = 2;
        assert_eq!(func.apply(&mut value2), 2);
        assert_eq!(value2, 8);
    }

    #[test]
    fn test_to_fn() {
        let func = TestStatefulMutatingFunction::new(5);

        // Test to_fn conversion
        let mut closure = func.to_fn();
        let mut value = 2;
        assert_eq!(closure(&mut value), 2);
        assert_eq!(value, 10);

        // Test that original is still usable (need to create a new instance for comparison)
        let mut func2 = TestStatefulMutatingFunction::new(5);
        let mut value2 = 1;
        assert_eq!(func2.apply(&mut value2), 1);
        assert_eq!(value2, 5);
    }
}

// ============================================================================
// BoxStatefulMutatingFunction Tests
// ============================================================================

#[cfg(test)]
mod test_box_stateful_mutating_function {
    use super::*;

    #[test]
    fn test_new() {
        let mut counter = {
            let mut count = 0;
            BoxStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x += 1;
                count
            })
        };
        let mut value = 5;
        assert_eq!(counter.apply(&mut value), 1);
        assert_eq!(value, 6);
        assert_eq!(counter.apply(&mut value), 2);
        assert_eq!(value, 7);
    }

    #[test]
    fn test_accumulator() {
        let mut accumulator = {
            let mut sum = 0;
            BoxStatefulMutatingFunction::new(move |x: &mut i32| {
                *x *= 2;
                sum += *x;
                sum
            })
        };

        let mut value = 5;
        assert_eq!(accumulator.apply(&mut value), 10);
        assert_eq!(value, 10);

        let mut value2 = 3;
        assert_eq!(accumulator.apply(&mut value2), 16); // 10 + 6
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_identity() {
        let mut identity = BoxStatefulMutatingFunction::<i32, i32>::identity();
        let mut value = 42;
        assert_eq!(identity.apply(&mut value), 42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_map() {
        let func = {
            let mut count = 0;
            BoxStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut mapped = func.and_then::<String, _>(|count: &i32| format!("Call #{}", *count));

        let mut value = 5;
        assert_eq!(mapped.apply(&mut value), "Call #1");
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_box() {
        let func = {
            let mut count = 0;
            BoxStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut boxed = func.into_box();

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_rc() {
        let func = {
            let mut count = 0;
            BoxStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut rc = func.into_rc();

        let mut value = 5;
        assert_eq!(rc.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_fn() {
        let func = {
            let mut count = 0;
            BoxStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut closure = func.into_fn();

        let mut value = 5;
        assert_eq!(closure(&mut value), 1);
        assert_eq!(value, 10);
    }
}

// ============================================================================
// RcStatefulMutatingFunction Tests
// ============================================================================

#[cfg(test)]
mod test_rc_stateful_mutating_function {
    use super::*;

    #[test]
    fn test_new() {
        let mut counter = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x += 1;
                count
            })
        };
        let mut value = 5;
        assert_eq!(counter.apply(&mut value), 1);
        assert_eq!(value, 6);
        assert_eq!(counter.apply(&mut value), 2);
        assert_eq!(value, 7);
    }

    #[test]
    fn test_clone() {
        let counter = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut clone = counter.clone();

        let mut value1 = 5;
        assert_eq!(clone.apply(&mut value1), 1);
        assert_eq!(value1, 10);

        // Shared state
        let mut value2 = 3;
        assert_eq!(clone.apply(&mut value2), 2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_identity() {
        let mut identity = RcStatefulMutatingFunction::<i32, i32>::identity();
        let mut value = 42;
        assert_eq!(identity.apply(&mut value), 42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_map() {
        let func = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut mapped = func.and_then::<String, _>(|count: &i32| format!("Call #{}", *count));

        let mut value = 5;
        assert_eq!(mapped.apply(&mut value), "Call #1");
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_box() {
        let func = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut boxed = func.into_box();

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_rc() {
        let func = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut rc = func.into_rc();

        let mut value = 5;
        assert_eq!(rc.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_fn() {
        let func = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut closure = func.into_fn();

        let mut value = 5;
        assert_eq!(closure(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_box() {
        let func = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut boxed = func.to_box();

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 1);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut func_clone = func.clone();
        let mut value2 = 3;
        assert_eq!(func_clone.apply(&mut value2), 2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_rc() {
        let func = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut rc = func.to_rc();

        let mut value = 5;
        assert_eq!(rc.apply(&mut value), 1);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut func_clone = func.clone();
        let mut value2 = 3;
        assert_eq!(func_clone.apply(&mut value2), 2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_fn() {
        let func = {
            let mut count = 0;
            RcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut closure = func.to_fn();

        let mut value = 5;
        assert_eq!(closure(&mut value), 1);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut func_clone = func.clone();
        let mut value2 = 3;
        assert_eq!(func_clone.apply(&mut value2), 2);
        assert_eq!(value2, 6);
    }
}

// ============================================================================
// ArcStatefulMutatingFunction Tests
// ============================================================================

#[cfg(test)]
mod test_arc_stateful_mutating_function {
    use super::*;
    use std::thread;

    #[test]
    fn test_new() {
        let mut counter = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x += 1;
                count
            })
        };
        let mut value = 5;
        assert_eq!(counter.apply(&mut value), 1);
        assert_eq!(value, 6);
        assert_eq!(counter.apply(&mut value), 2);
        assert_eq!(value, 7);
    }

    #[test]
    fn test_clone() {
        let counter = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut clone = counter.clone();

        let mut value1 = 5;
        assert_eq!(clone.apply(&mut value1), 1);
        assert_eq!(value1, 10);

        // Shared state
        let mut value2 = 3;
        assert_eq!(clone.apply(&mut value2), 2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_thread_safe() {
        let counter = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut counter_clone = counter.clone();

        let handle = thread::spawn(move || {
            let mut value = 5;
            counter_clone.apply(&mut value)
        });

        let result = handle.join().unwrap();
        assert_eq!(result, 1);
    }

    #[test]
    fn test_identity() {
        let mut identity = ArcStatefulMutatingFunction::<i32, i32>::identity();
        let mut value = 42;
        assert_eq!(identity.apply(&mut value), 42);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_map() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut mapped = func.and_then::<String, _>(|count: &i32| format!("Call #{}", *count));

        let mut value = 5;
        assert_eq!(mapped.apply(&mut value), "Call #1");
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_box() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut boxed = func.into_box();

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_rc() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut rc = func.into_rc();

        let mut value = 5;
        assert_eq!(rc.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_arc() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut arc = func.into_arc();

        let mut value = 5;
        assert_eq!(arc.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_fn() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut closure = func.into_fn();

        let mut value = 5;
        assert_eq!(closure(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_box() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut boxed = func.to_box();

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 1);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut func_clone = func.clone();
        let mut value2 = 3;
        assert_eq!(func_clone.apply(&mut value2), 2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_rc() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut rc = func.to_rc();

        let mut value = 5;
        assert_eq!(rc.apply(&mut value), 1);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut func_clone = func.clone();
        let mut value2 = 3;
        assert_eq!(func_clone.apply(&mut value2), 2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_arc() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut arc = func.to_arc();

        let mut value = 5;
        assert_eq!(arc.apply(&mut value), 1);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut func_clone = func.clone();
        let mut value2 = 3;
        assert_eq!(func_clone.apply(&mut value2), 2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_fn() {
        let func = {
            let mut count = 0;
            ArcStatefulMutatingFunction::new(move |x: &mut i32| {
                count += 1;
                *x *= 2;
                count
            })
        };
        let mut closure = func.to_fn();

        let mut value = 5;
        assert_eq!(closure(&mut value), 1);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut func_clone = func.clone();
        let mut value2 = 3;
        assert_eq!(func_clone.apply(&mut value2), 2);
        assert_eq!(value2, 6);
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
        // Use Rc<RefCell<>> to properly test stateful behavior
        let count = Rc::new(RefCell::new(0));
        let count_clone = Rc::clone(&count);
        let closure = move |x: &mut i32| {
            let mut current = count_clone.borrow_mut();
            *current += 1;
            *x *= 2;
            *current
        };

        // Test direct closure calls
        let mut value = 5;
        let direct_result1 = closure(&mut value);
        assert_eq!(direct_result1, 1);
        assert_eq!(value, 10);

        let direct_result2 = closure(&mut value);
        assert_eq!(direct_result2, 2);
        assert_eq!(value, 20);

        // Test with trait
        let count2 = Rc::new(RefCell::new(0));
        let count2_clone = Rc::clone(&count2);
        let mut closure2 = move |x: &mut i32| {
            let mut current = count2_clone.borrow_mut();
            *current += 1;
            *x *= 2;
            *current
        };

        // Test that closure implements StatefulMutatingFunction trait
        let _trait_check: &mut dyn StatefulMutatingFunction<i32, i32> = &mut closure2;

        let mut value2 = 5;
        let result1 = StatefulMutatingFunction::apply(&mut closure2, &mut value2);
        assert_eq!(result1, 1);
        assert_eq!(value2, 10);

        let result2 = StatefulMutatingFunction::apply(&mut closure2, &mut value2);
        assert_eq!(result2, 2);
        assert_eq!(value2, 20);
    }

    #[test]
    fn test_closure_and_then() {
        let mut count1 = 0;
        let count2 = Rc::new(RefCell::new(0));
        let count2_clone = Rc::clone(&count2);
        let mut chained = (move |x: &mut i32| {
            count1 += 1;
            *x *= 2;
            count1
        })
        .and_then::<i32, _>(move |x: &mut i32| {
            *count2_clone.borrow_mut() += 1;
            *x + 10
        });

        let mut value = 5;
        let result = chained.apply(&mut value);
        assert_eq!(result, 11); // First function returns 1, second function returns 1 + 10
        assert_eq!(value, 10); // Input only modified by first function
        assert_eq!(*count2.borrow(), 1); // Second function should be called once
    }

    #[test]
    fn test_closure_map() {
        let mut count = 0;
        let mut mapped = (move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        })
        .and_then::<String, _>(|count: &mut i32| format!("Call #{}", *count));

        let mut value = 5;
        assert_eq!(mapped.apply(&mut value), "Call #1");
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_into_box() {
        let mut count = 0;
        let closure = move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        };
        let mut boxed = StatefulMutatingFunction::into_box(closure);

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_into_once() {
        let mut count = 0;
        let closure = move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        };
        let once_func: BoxMutatingFunctionOnce<i32, i32> = closure.into_once();

        let mut value = 5;
        assert_eq!(once_func.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_to_once() {
        // Use Rc<RefCell<>> to properly test shared state with to_once
        let count = Rc::new(RefCell::new(0));
        let count_clone = Rc::clone(&count);
        let closure = move |x: &mut i32| {
            let mut current = count_clone.borrow_mut();
            *current += 1;
            *x *= 2;
            *current
        };

        let once_func: BoxMutatingFunctionOnce<i32, i32> = StatefulMutatingFunction::to_once(&closure);

        let mut value = 5;
        assert_eq!(once_func.apply(&mut value), 1);
        assert_eq!(value, 10);

        // Original closure should still be usable and have independent state
        let mut value2 = 3;
        assert_eq!(closure(&mut value2), 2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_closure_into_rc() {
        let mut count = 0;
        let closure = move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        };
        let mut rc = closure.into_rc();

        let mut value = 5;
        assert_eq!(rc.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_into_arc() {
        let mut count = 0;
        let closure = move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        };
        let mut arc = closure.into_arc();

        let mut value = 5;
        assert_eq!(arc.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_to_box() {
        let mut count = 0;
        let closure = move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        };
        let mut boxed = StatefulMutatingFunction::to_box(&closure);

        let mut value = 5;
        assert_eq!(boxed.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_to_rc() {
        let mut count = 0;
        let closure = move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        };
        let mut rc = closure.to_rc();

        let mut value = 5;
        assert_eq!(rc.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_to_arc() {
        let mut count = 0;
        let closure = move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        };
        let mut arc = closure.to_arc();

        let mut value = 5;
        assert_eq!(arc.apply(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_to_fn() {
        let mut count = 0;
        let closure = move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        };
        let mut fn_closure = StatefulMutatingFunction::to_fn(&closure);

        let mut value = 5;
        assert_eq!(fn_closure(&mut value), 1);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_into_fn() {
        let mut count = 0;
        let closure = move |x: &mut i32| {
            count += 1;
            *x *= 2;
            count
        };
        let mut fn_closure = StatefulMutatingFunction::into_fn(closure);

        let mut value = 5;
        assert_eq!(fn_closure(&mut value), 1);
        assert_eq!(value, 10);
    }
}

// ============================================================================
// StatefulMutatingFunction Debug and Display Tests
// ============================================================================

#[test]
fn test_box_stateful_mutating_function_debug_display() {
    // Test Debug and Display for BoxStatefulMutatingFunction without name

    let mut double = BoxStatefulMutatingFunction::new(move |x: &mut i32| *x * 2);
    // Call apply to use the counter variable
    let mut value1 = 5;
    let _result1 = double.apply(&mut value1);

    let debug_str = format!("{:?}", double);
    assert!(debug_str.contains("BoxStatefulMutatingFunction"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));

    let display_str = format!("{}", double);
    assert_eq!(display_str, "BoxStatefulMutatingFunction");

    // Test Debug and Display for BoxStatefulMutatingFunction with name
    let mut named_double =
        BoxStatefulMutatingFunction::new_with_name("box_stateful_mutating", |x: &mut i32| *x * 2);
    // Call apply to ensure the function works
    let mut value2 = 3;
    let _result2 = named_double.apply(&mut value2);

    let named_debug_str = format!("{:?}", named_double);
    assert!(named_debug_str.contains("BoxStatefulMutatingFunction"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));

    let named_display_str = format!("{}", named_double);
    assert_eq!(
        named_display_str,
        "BoxStatefulMutatingFunction(box_stateful_mutating)"
    );
}

#[test]
fn test_rc_stateful_mutating_function_debug_display() {
    // Test Debug and Display for RcStatefulMutatingFunction without name

    let mut double = RcStatefulMutatingFunction::new(move |x: &mut i32| *x * 2);
    // Call apply to use the counter variable
    let mut value1 = 5;
    let _result1 = double.apply(&mut value1);

    let debug_str = format!("{:?}", double);
    assert!(debug_str.contains("RcStatefulMutatingFunction"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));

    let display_str = format!("{}", double);
    assert_eq!(display_str, "RcStatefulMutatingFunction");

    // Test Debug and Display for RcStatefulMutatingFunction with name
    let mut named_double =
        RcStatefulMutatingFunction::new_with_name("rc_stateful_mutating", |x: &mut i32| *x * 2);
    // Call apply to ensure the function works
    let mut value2 = 3;
    let _result2 = named_double.apply(&mut value2);

    let named_debug_str = format!("{:?}", named_double);
    assert!(named_debug_str.contains("RcStatefulMutatingFunction"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));

    let named_display_str = format!("{}", named_double);
    assert_eq!(
        named_display_str,
        "RcStatefulMutatingFunction(rc_stateful_mutating)"
    );
}

#[test]
fn test_arc_stateful_mutating_function_debug_display() {
    // Test Debug and Display for ArcStatefulMutatingFunction without name

    let mut double = ArcStatefulMutatingFunction::new(move |x: &mut i32| *x * 2);
    // Call apply to use the counter variable
    let mut value1 = 5;
    let _result1 = double.apply(&mut value1);

    let debug_str = format!("{:?}", double);
    assert!(debug_str.contains("ArcStatefulMutatingFunction"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));

    let display_str = format!("{}", double);
    assert_eq!(display_str, "ArcStatefulMutatingFunction");

    // Test Debug and Display for ArcStatefulMutatingFunction with name
    let mut named_double =
        ArcStatefulMutatingFunction::new_with_name("arc_stateful_mutating", |x: &mut i32| *x * 2);
    // Call apply to ensure the function works
    let mut value2 = 3;
    let _result2 = named_double.apply(&mut value2);

    let named_debug_str = format!("{:?}", named_double);
    assert!(named_debug_str.contains("ArcStatefulMutatingFunction"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));

    let named_display_str = format!("{}", named_double);
    assert_eq!(
        named_display_str,
        "ArcStatefulMutatingFunction(arc_stateful_mutating)"
    );
}

// ============================================================================
// StatefulMutatingFunction Name Management Tests
// ============================================================================

// Allow unused variables in tests since they are used in closures
#[allow(unused_variables)]
#[test]
fn test_box_stateful_mutating_function_name_methods() {
    // Test new_with_name, name(), and set_name()

    let mut double = BoxStatefulMutatingFunction::new_with_name(
        "box_stateful_mutating_func",
        move |x: &mut i32| {
            *x *= 2;
            *x
        },
    );

    // Test name() returns the initial name
    assert_eq!(double.name(), Some("box_stateful_mutating_func"));

    // Test set_name() changes the name
    double.set_name("modified_box_stateful_mutating");
    assert_eq!(double.name(), Some("modified_box_stateful_mutating"));

    // Test that function still works after name change
    let mut value = 5;
    assert_eq!(double.apply(&mut value), 10);
    assert_eq!(value, 10);
}

// Allow unused variables in tests since they are used in closures
#[allow(unused_variables)]
#[test]
fn test_rc_stateful_mutating_function_name_methods() {
    // Test new_with_name, name(), and set_name()

    let mut double = RcStatefulMutatingFunction::new_with_name(
        "rc_stateful_mutating_func",
        move |x: &mut i32| {
            *x *= 2;
            *x
        },
    );

    // Test name() returns the initial name
    assert_eq!(double.name(), Some("rc_stateful_mutating_func"));

    // Test set_name() changes the name
    double.set_name("modified_rc_stateful_mutating");
    assert_eq!(double.name(), Some("modified_rc_stateful_mutating"));

    // Test that function still works after name change
    let mut value = 5;
    assert_eq!(double.apply(&mut value), 10);
    assert_eq!(value, 10);

    // Test cloning preserves name
    let mut cloned = double.clone();
    assert_eq!(cloned.name(), Some("modified_rc_stateful_mutating"));
    let mut value2 = 3;
    assert_eq!(cloned.apply(&mut value2), 6);
    assert_eq!(value2, 6);
}

// Allow unused variables in tests since they are used in closures
#[allow(unused_variables)]
#[test]
fn test_arc_stateful_mutating_function_name_methods() {
    // Test new_with_name, name(), and set_name()

    let mut double = ArcStatefulMutatingFunction::new_with_name(
        "arc_stateful_mutating_func",
        move |x: &mut i32| {
            *x *= 2;
            *x
        },
    );

    // Test name() returns the initial name
    assert_eq!(double.name(), Some("arc_stateful_mutating_func"));

    // Test set_name() changes the name
    double.set_name("modified_arc_stateful_mutating");
    assert_eq!(double.name(), Some("modified_arc_stateful_mutating"));

    // Test that function still works after name change
    let mut value = 5;
    assert_eq!(double.apply(&mut value), 10);
    assert_eq!(value, 10);

    // Test cloning preserves name
    let mut cloned = double.clone();
    assert_eq!(cloned.name(), Some("modified_arc_stateful_mutating"));
    let mut value2 = 3;
    assert_eq!(cloned.apply(&mut value2), 6);
    assert_eq!(value2, 6);
}

// ============================================================================
// ConditionalStatefulMutatingFunction Debug and Display Tests
// ============================================================================

// Allow unused variables in debug/display tests since they are used in closures
#[allow(unused_variables, unused_assignments)]
#[test]
fn test_box_conditional_stateful_mutating_function_debug_display() {
    // Test Debug and Display for BoxConditionalStatefulMutatingFunction without name

    let mut double = BoxStatefulMutatingFunction::new(move |x: &mut i32| {
        *x *= 2;
        *x
    });
    // Call apply to use the counter variable
    let mut test_val = 5;
    let _result1 = double.apply(&mut test_val);

    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("BoxConditionalStatefulMutatingFunction"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));
    assert!(debug_str.contains("predicate"));

    let display_str = format!("{}", conditional);
    assert!(display_str.starts_with("BoxConditionalStatefulMutatingFunction("));
    assert!(display_str.contains("BoxStatefulMutatingFunction"));
    assert!(display_str.contains("BoxPredicate"));
    assert!(display_str.ends_with(")"));

    // Test Debug and Display for BoxConditionalStatefulMutatingFunction with name
    let mut named_double =
        BoxStatefulMutatingFunction::new_with_name("stateful_mutating_double", |x: &mut i32| {
            *x *= 2;
            *x
        });
    // Call apply to ensure the function works
    let mut test_val2 = 3;
    let _result2 = named_double.apply(&mut test_val2);

    let named_conditional = named_double.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(named_debug_str.contains("BoxConditionalStatefulMutatingFunction"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));
    assert!(named_debug_str.contains("predicate"));

    let named_display_str = format!("{}", named_conditional);
    assert!(named_display_str.starts_with("BoxConditionalStatefulMutatingFunction("));
    assert!(named_display_str.contains("BoxStatefulMutatingFunction(stateful_mutating_double)"));
    assert!(named_display_str.contains("BoxPredicate"));
    assert!(named_display_str.ends_with(")"));
}

// Allow unused variables in debug/display tests since they are used in closures
#[allow(unused_variables, unused_assignments)]
#[test]
fn test_rc_conditional_stateful_mutating_function_debug_display() {
    // Test Debug and Display for RcConditionalStatefulMutatingFunction without name

    let mut double = RcStatefulMutatingFunction::new(move |x: &mut i32| {
        *x *= 2;
        *x
    });
    // Call apply to use the counter variable
    let mut test_val = 5;
    let _result1 = double.apply(&mut test_val);

    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("RcConditionalStatefulMutatingFunction"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));
    assert!(debug_str.contains("predicate"));

    let display_str = format!("{}", conditional);
    assert!(display_str.starts_with("RcConditionalStatefulMutatingFunction("));
    assert!(display_str.contains("RcStatefulMutatingFunction"));
    assert!(display_str.contains("RcPredicate"));
    assert!(display_str.ends_with(")"));

    // Test Debug and Display for RcConditionalStatefulMutatingFunction with name
    let mut named_double =
        RcStatefulMutatingFunction::new_with_name("rc_stateful_mutating_double", |x: &mut i32| {
            *x *= 2;
            *x
        });
    // Call apply to ensure the function works
    let mut test_val2 = 3;
    let _result2 = named_double.apply(&mut test_val2);

    let named_conditional = named_double.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(named_debug_str.contains("RcConditionalStatefulMutatingFunction"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));
    assert!(named_debug_str.contains("predicate"));

    let named_display_str = format!("{}", named_conditional);
    assert!(named_display_str.starts_with("RcConditionalStatefulMutatingFunction("));
    assert!(named_display_str.contains("RcStatefulMutatingFunction(rc_stateful_mutating_double)"));
    assert!(named_display_str.contains("RcPredicate"));
    assert!(named_display_str.ends_with(")"));
}

// Allow unused variables in debug/display tests since they are used in closures
#[allow(unused_variables, unused_assignments)]
#[test]
fn test_arc_conditional_stateful_mutating_function_debug_display() {
    // Test Debug and Display for ArcConditionalStatefulMutatingFunction without name

    let mut double = ArcStatefulMutatingFunction::new(move |x: &mut i32| {
        *x *= 2;
        *x
    });
    // Call apply to use the counter variable
    let mut test_val = 5;
    let _result1 = double.apply(&mut test_val);

    let conditional = double.when(|x: &i32| *x > 0);

    let debug_str = format!("{:?}", conditional);
    assert!(debug_str.contains("ArcConditionalStatefulMutatingFunction"));
    assert!(debug_str.contains("name"));
    assert!(debug_str.contains("function"));
    assert!(debug_str.contains("predicate"));

    let display_str = format!("{}", conditional);
    assert!(display_str.starts_with("ArcConditionalStatefulMutatingFunction("));
    assert!(display_str.contains("ArcStatefulMutatingFunction"));
    assert!(display_str.contains("ArcPredicate"));
    assert!(display_str.ends_with(")"));

    // Test Debug and Display for ArcConditionalStatefulMutatingFunction with name
    let mut named_double = ArcStatefulMutatingFunction::new_with_name(
        "arc_stateful_mutating_double",
        |x: &mut i32| {
            *x *= 2;
            *x
        },
    );
    // Call apply to ensure the function works
    let mut test_val2 = 3;
    let _result2 = named_double.apply(&mut test_val2);

    let named_conditional = named_double.when(|x: &i32| *x % 2 == 0);

    let named_debug_str = format!("{:?}", named_conditional);
    assert!(named_debug_str.contains("ArcConditionalStatefulMutatingFunction"));
    assert!(named_debug_str.contains("name"));
    assert!(named_debug_str.contains("function"));
    assert!(named_debug_str.contains("predicate"));

    let named_display_str = format!("{}", named_conditional);
    assert!(named_display_str.starts_with("ArcConditionalStatefulMutatingFunction("));
    assert!(named_display_str.contains("ArcStatefulMutatingFunction(arc_stateful_mutating_double)"));
    assert!(named_display_str.contains("ArcPredicate"));
    assert!(named_display_str.ends_with(")"));
}

// ============================================================================
// StatefulMutatingFunction Trait Default Methods Tests - into_once, to_once
// ============================================================================

#[cfg(test)]
mod test_stateful_mutating_function_trait_default_methods {
    use super::*;
    use prism3_function::MutatingFunctionOnce;
    use std::sync::{Arc, atomic::{AtomicUsize, Ordering}};

    #[test]
    fn test_custom_stateful_mutating_function_into_once() {
        let counter = Arc::new(AtomicUsize::new(0));

        struct MyStatefulMutatingFunction {
            counter: Arc<AtomicUsize>,
        }

        impl StatefulMutatingFunction<i32, i32> for MyStatefulMutatingFunction {
            fn apply(&mut self, value: &mut i32) -> i32 {
                self.counter.fetch_add(1, Ordering::SeqCst);
                *value += 1;
                *value
            }
        }

        let my_func = MyStatefulMutatingFunction {
            counter: counter.clone(),
        };

        // Test into_once() - should consume the function
        let once_func = my_func.into_once();
        let mut value = 5;
        let result = once_func.apply(&mut value);
        assert_eq!(result, 6);
        assert_eq!(value, 6);
        assert_eq!(counter.load(Ordering::SeqCst), 1);
    }

    #[test]
    fn test_custom_stateful_mutating_function_to_once() {
        let counter = Arc::new(AtomicUsize::new(0));

        #[derive(Clone)]
        struct MyStatefulMutatingFunction {
            counter: Arc<AtomicUsize>,
        }

        impl StatefulMutatingFunction<i32, i32> for MyStatefulMutatingFunction {
            fn apply(&mut self, value: &mut i32) -> i32 {
                self.counter.fetch_add(1, Ordering::SeqCst);
                *value += 1;
                *value
            }
        }

        let mut my_func = MyStatefulMutatingFunction {
            counter: counter.clone(),
        };

        // Test to_once() - should not consume the original
        let once_func = my_func.to_once();
        let mut value = 5;
        let result = once_func.apply(&mut value);
        assert_eq!(result, 6);
        assert_eq!(value, 6);
        assert_eq!(counter.load(Ordering::SeqCst), 1);

        // Original function should still be usable
        let mut value2 = 10;
        let result2 = my_func.apply(&mut value2);
        assert_eq!(result2, 11);
        assert_eq!(value2, 11);
        assert_eq!(counter.load(Ordering::SeqCst), 2);
    }
}
