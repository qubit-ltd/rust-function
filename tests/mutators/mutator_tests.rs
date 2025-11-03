/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Unit tests for Mutator types (stateless Fn(&mut T))

use prism3_function::{
    ArcMutator,
    BoxMutator,
    FnMutatorOps,
    Mutator,
    MutatorOnce,
    RcMutator,
};

// ============================================================================
// Mutator Default Implementation Tests
// ============================================================================

/// Test struct that implements Mutator to test default methods
struct TestMutator {
    multiplier: i32,
}

impl TestMutator {
    fn new(multiplier: i32) -> Self {
        TestMutator { multiplier }
    }
}

impl Mutator<i32> for TestMutator {
    fn apply(&self, input: &mut i32) {
        *input *= self.multiplier;
    }
}

impl Clone for TestMutator {
    fn clone(&self) -> Self {
        TestMutator {
            multiplier: self.multiplier,
        }
    }
}

#[cfg(test)]
mod test_mutator_default_impl {
    use super::*;

    #[test]
    fn test_into_box() {
        let mutator = TestMutator::new(2);
        let boxed = mutator.into_box();

        let mut value = 5;
        boxed.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_rc() {
        let mutator = TestMutator::new(3);
        let rc = mutator.into_rc();

        let mut value = 4;
        rc.apply(&mut value);
        assert_eq!(value, 12);
    }

    #[test]
    fn test_into_arc() {
        let mutator = TestMutator::new(4);
        let arc = mutator.into_arc();

        let mut value = 3;
        arc.apply(&mut value);
        assert_eq!(value, 12);
    }

    #[test]
    fn test_into_fn() {
        let mutator = TestMutator::new(5);
        let closure = mutator.into_fn();

        let mut value = 2;
        closure(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_box() {
        let mutator = TestMutator::new(2);
        let boxed = mutator.to_box();

        let mut value = 5;
        boxed.apply(&mut value);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut value2 = 3;
        mutator.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_rc() {
        let mutator = TestMutator::new(3);
        let rc = mutator.to_rc();

        let mut value = 4;
        rc.apply(&mut value);
        assert_eq!(value, 12);

        // Original should still be usable since it was cloned
        let mut value2 = 2;
        mutator.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_arc() {
        let mutator = TestMutator::new(4);
        let arc = mutator.to_arc();

        let mut value = 3;
        arc.apply(&mut value);
        assert_eq!(value, 12);

        // Original should still be usable since it was cloned
        let mut value2 = 2;
        mutator.apply(&mut value2);
        assert_eq!(value2, 8);
    }

    #[test]
    fn test_to_fn() {
        let mutator = TestMutator::new(5);
        let closure = mutator.to_fn();

        let mut value = 2;
        closure(&mut value);
        assert_eq!(value, 10);

        // Original should still be usable since it was cloned
        let mut value2 = 1;
        mutator.apply(&mut value2);
        assert_eq!(value2, 5);
    }
}

// ============================================================================
// BoxMutator Tests
// ============================================================================

#[cfg(test)]
mod test_box_mutator {
    use super::*;

    #[test]
    fn test_new() {
        let mutator = BoxMutator::new(|x: &mut i32| *x += 1);
        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_with_different_types() {
        // String
        let string_mutator = BoxMutator::new(|s: &mut String| s.push('!'));
        let mut text = String::from("hello");
        string_mutator.apply(&mut text);
        assert_eq!(text, "hello!");

        // Vec
        let vec_mutator = BoxMutator::new(|v: &mut Vec<i32>| v.push(42));
        let mut numbers = vec![1, 2, 3];
        vec_mutator.apply(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3, 42]);

        // bool
        let bool_mutator = BoxMutator::new(|b: &mut bool| *b = !*b);
        let mut flag = true;
        bool_mutator.apply(&mut flag);
        assert!(!flag);
    }

    #[test]
    fn test_and_then() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2).and_then(|x: &mut i32| *x += 10);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 20); // (5 * 2) + 10
    }

    #[test]
    fn test_and_then_multiple_chains() {
        let mutator = BoxMutator::new(|x: &mut i32| *x += 1)
            .and_then(|x: &mut i32| *x *= 2)
            .and_then(|x: &mut i32| *x -= 5);

        let mut value = 10;
        mutator.apply(&mut value);
        assert_eq!(value, 17); // ((10 + 1) * 2) - 5
    }

    #[test]
    fn test_and_then_with_box_mutator() {
        let c1 = BoxMutator::new(|x: &mut i32| *x *= 2);
        let c2 = BoxMutator::new(|x: &mut i32| *x += 10);
        let combined = c1.and_then(c2);

        let mut value = 5;
        combined.apply(&mut value);
        assert_eq!(value, 20);
    }

    #[test]
    fn test_noop() {
        let noop = BoxMutator::<i32>::noop();
        let mut value = 42;
        noop.apply(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_noop_with_different_types() {
        // Test with String
        let noop = BoxMutator::<String>::noop();
        let mut text = String::from("hello");
        noop.apply(&mut text);
        assert_eq!(text, "hello");

        // Test with Vec
        let noop = BoxMutator::<Vec<i32>>::noop();
        let mut numbers = vec![1, 2, 3];
        noop.apply(&mut numbers);
        assert_eq!(numbers, vec![1, 2, 3]);
    }

    #[test]
    fn test_noop_chaining() {
        let chained = BoxMutator::<i32>::noop()
            .and_then(|x: &mut i32| *x *= 2)
            .and_then(BoxMutator::<i32>::noop());

        let mut value = 5;
        chained.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_if_then_true() {
        let mutator = BoxMutator::new(|x: &mut i32| *x += 10).when(|x: &i32| *x > 0);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 15);
    }

    #[test]
    fn test_if_then_false() {
        let mutator = BoxMutator::new(|x: &mut i32| *x += 10).when(|x: &i32| *x > 0);

        let mut value = -5;
        mutator.apply(&mut value);
        assert_eq!(value, -5); // unchanged
    }

    #[test]
    fn test_if_then_else() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(|x: &mut i32| *x = -*x);

        let mut positive = 10;
        mutator.apply(&mut positive);
        assert_eq!(positive, 20);

        let mut negative = -10;
        mutator.apply(&mut negative);
        assert_eq!(negative, 10);
    }

    #[test]
    fn test_conditional_and_then() {
        let cond1 = BoxMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let cond2 = BoxMutator::new(|x: &mut i32| *x += 5);
        let chained = cond1.and_then(cond2);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 25); // (10 * 2) + 5

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, -5); // -10 + 5 (condition not met, only second mutator runs)
    }

    #[test]
    fn test_conditional_into_box() {
        let conditional = BoxMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let boxed = conditional.into_box();

        let mut positive = 5;
        boxed.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_into_rc() {
        let conditional = BoxMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let rc = conditional.into_rc();

        let mut positive = 5;
        rc.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_into_fn() {
        let conditional = BoxMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let closure = conditional.into_fn();

        let mut positive = 5;
        closure(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_into_box() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let boxed = mutator.into_box();
        let mut value = 5;
        boxed.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_rc() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let rc = mutator.into_rc();
        let mut value = 5;
        rc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_new_with_name() {
        let mutator = BoxMutator::new_with_name("test_mutator", |x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), Some("test_mutator"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_some() {
        let mutator = BoxMutator::new_with_optional_name(|x: &mut i32| *x += 1, Some("optional_name".to_string()));
        assert_eq!(mutator.name(), Some("optional_name"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_none() {
        let mutator = BoxMutator::new_with_optional_name(|x: &mut i32| *x += 1, None);
        assert_eq!(mutator.name(), None);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_name_and_set_name() {
        let mut mutator = BoxMutator::new(|x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), None);

        mutator.set_name("set_name_test");
        assert_eq!(mutator.name(), Some("set_name_test"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_box_mutator_debug() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let debug_str = format!("{:?}", mutator);
        assert!(debug_str.contains("BoxMutator"));
        assert!(debug_str.contains("name"));
        assert!(debug_str.contains("function"));
    }

    #[test]
    fn test_box_mutator_display() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let display_str = format!("{}", mutator);
        assert_eq!(display_str, "BoxMutator");

        let named_mutator = BoxMutator::new_with_name("test_mutator", |x: &mut i32| *x *= 2);
        let named_display_str = format!("{}", named_mutator);
        assert_eq!(named_display_str, "BoxMutator(test_mutator)");
    }

    // Note: BoxMutator cannot be safely converted to ArcMutator because the
    // inner function may not be Send. This test has been removed.
}

// ============================================================================
// ArcMutator Tests
// ============================================================================

#[cfg(test)]
mod test_arc_mutator {
    use super::*;
    use std::thread;

    #[test]
    fn test_new() {
        let mutator = ArcMutator::new(|x: &mut i32| *x += 1);
        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_clone() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let clone1 = mutator.clone();
        let clone2 = mutator.clone();

        let mut value1 = 5;
        clone1.apply(&mut value1);
        assert_eq!(value1, 10);

        let mut value2 = 3;
        clone2.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_and_then() {
        let first = ArcMutator::new(|x: &mut i32| *x *= 2);
        let second = ArcMutator::new(|x: &mut i32| *x += 10);

        let chained = first.and_then(second);

        let mut value = 5;
        chained.apply(&mut value);
        assert_eq!(value, 20); // (5 * 2) + 10

        // first and second are still usable
        let mut value2 = 3;
        first.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_thread_safety() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let clone = mutator.clone();

        let handle = thread::spawn(move || {
            let mut value = 5;
            clone.apply(&mut value);
            value
        });

        let mut value = 3;
        mutator.apply(&mut value);
        assert_eq!(value, 6);

        assert_eq!(handle.join().unwrap(), 10);
    }

    #[test]
    fn test_into_box() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let boxed = mutator.into_box();
        let mut value = 5;
        boxed.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_rc() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let rc = mutator.into_rc();
        let mut value = 5;
        rc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_arc() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let arc = mutator.into_arc();
        let mut value = 5;
        arc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_noop() {
        let noop = ArcMutator::<i32>::noop();
        let mut value = 42;
        noop.apply(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_noop_clone() {
        let noop = ArcMutator::<i32>::noop();
        let clone1 = noop.clone();
        let clone2 = noop.clone();

        let mut value1 = 42;
        clone1.apply(&mut value1);
        assert_eq!(value1, 42);

        let mut value2 = 100;
        clone2.apply(&mut value2);
        assert_eq!(value2, 100);
    }

    #[test]
    fn test_noop_chaining() {
        let noop = ArcMutator::<i32>::noop();
        let double = ArcMutator::new(|x: &mut i32| *x *= 2);

        let chained = noop.and_then(double);

        let mut value = 5;
        chained.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_box() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let boxed = mutator.to_box();
        let mut value = 5;
        boxed.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_rc() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let rc = mutator.to_rc();
        let mut value = 5;
        rc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_arc() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let arc = mutator.to_arc();
        let mut value = 5;
        arc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_fn() {
        let mutator = ArcMutator::new(|x: &mut i32| *x += 10);
        let mut values = vec![1, 2, 3];
        values.iter_mut().for_each(mutator.to_fn());
        assert_eq!(values, vec![11, 12, 13]);
    }

    #[test]
    fn test_to_box_preserves_original() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let boxed = mutator.to_box();

        // Original still usable
        let mut value1 = 5;
        mutator.apply(&mut value1);
        assert_eq!(value1, 10);

        // Boxed version also works
        let mut value2 = 3;
        boxed.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_rc_preserves_original() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let rc = mutator.to_rc();

        // Original still usable
        let mut value1 = 5;
        mutator.apply(&mut value1);
        assert_eq!(value1, 10);

        // to_rc version also works
        let mut value2 = 3;
        rc.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_arc_preserves_original() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let arc = mutator.to_arc();

        // Original still usable
        let mut value1 = 5;
        mutator.apply(&mut value1);
        assert_eq!(value1, 10);

        // to_arc version also works
        let mut value2 = 3;
        arc.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_fn_preserves_original() {
        let mutator = ArcMutator::new(|x: &mut i32| *x += 10);

        // to_fn version works
        let mut values = vec![1, 2, 3];
        values.iter_mut().for_each(mutator.to_fn());
        assert_eq!(values, vec![11, 12, 13]);

        // Original still usable after to_fn (because ArcMutator is Clone)
        let mut value1 = 5;
        mutator.apply(&mut value1);
        assert_eq!(value1, 15);
    }

    #[test]
    fn test_to_arc_thread_safe() {
        use std::thread;

        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let arc = mutator.to_arc();
        let clone = arc.clone();

        let handle = thread::spawn(move || {
            let mut value = 5;
            clone.apply(&mut value);
            value
        });

        let mut value = 3;
        arc.apply(&mut value);
        assert_eq!(value, 6);

        assert_eq!(handle.join().unwrap(), 10);
    }

    #[test]
    fn test_when() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let mut positive = 5;
        conditional.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        conditional.apply(&mut negative);
        assert_eq!(negative, -5); // unchanged
    }

    #[test]
    fn test_into_fn() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let closure = mutator.into_fn();

        let mut value = 5;
        closure(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_conditional_into_box() {
        let conditional = ArcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let boxed = conditional.into_box();

        let mut positive = 5;
        boxed.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_into_rc() {
        let conditional = ArcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let rc = conditional.into_rc();

        let mut positive = 5;
        rc.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_into_arc() {
        let conditional = ArcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let arc = conditional.into_arc();

        let mut positive = 5;
        arc.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_into_fn() {
        let conditional = ArcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let closure = conditional.into_fn();

        let mut positive = 5;
        closure(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_to_box() {
        let conditional = ArcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let boxed = conditional.to_box();

        let mut positive = 5;
        boxed.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_to_rc() {
        let conditional = ArcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let rc = conditional.to_rc();

        let mut positive = 5;
        rc.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_to_arc() {
        let conditional = ArcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let arc = conditional.to_arc();

        let mut positive = 5;
        arc.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_to_fn() {
        let conditional = ArcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let closure = conditional.to_fn();

        let mut positive = 5;
        closure(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_or_else() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(|x: &mut i32| *x = -*x);

        let mut positive = 10;
        mutator.apply(&mut positive);
        assert_eq!(positive, 20);

        let mut negative = -10;
        mutator.apply(&mut negative);
        assert_eq!(negative, 10);
    }

    #[test]
    fn test_conditional_clone() {
        let conditional = ArcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let clone = conditional.clone();

        let mut positive = 5;
        conditional.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut value2 = 3;
        clone.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_new_with_name() {
        let mutator = ArcMutator::new_with_name("arc_test_mutator", |x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), Some("arc_test_mutator"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_some() {
        let mutator = ArcMutator::new_with_optional_name(|x: &mut i32| *x += 1, Some("arc_optional".to_string()));
        assert_eq!(mutator.name(), Some("arc_optional"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_none() {
        let mutator = ArcMutator::new_with_optional_name(|x: &mut i32| *x += 1, None);
        assert_eq!(mutator.name(), None);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_name_and_set_name() {
        let mut mutator = ArcMutator::new(|x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), None);

        mutator.set_name("arc_set_name");
        assert_eq!(mutator.name(), Some("arc_set_name"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }
}

// ============================================================================
// RcMutator Tests
// ============================================================================

#[cfg(test)]
mod test_rc_mutator {
    use super::*;

    #[test]
    fn test_new() {
        let mutator = RcMutator::new(|x: &mut i32| *x += 1);
        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_clone() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let clone1 = mutator.clone();
        let clone2 = mutator.clone();

        let mut value1 = 5;
        clone1.apply(&mut value1);
        assert_eq!(value1, 10);

        let mut value2 = 3;
        clone2.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_and_then() {
        let first = RcMutator::new(|x: &mut i32| *x *= 2);
        let second = RcMutator::new(|x: &mut i32| *x += 10);

        let chained = first.and_then(second.clone());

        let mut value = 5;
        chained.apply(&mut value);
        assert_eq!(value, 20); // (5 * 2) + 10

        // first and second are still usable
        let mut value2 = 3;
        first.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_into_box() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let boxed = mutator.into_box();
        let mut value = 5;
        boxed.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_into_rc() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let rc = mutator.into_rc();
        let mut value = 5;
        rc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_noop() {
        let noop = RcMutator::<i32>::noop();
        let mut value = 42;
        noop.apply(&mut value);
        assert_eq!(value, 42);
    }

    #[test]
    fn test_noop_clone() {
        let noop = RcMutator::<i32>::noop();
        let clone1 = noop.clone();
        let clone2 = noop.clone();

        let mut value1 = 42;
        clone1.apply(&mut value1);
        assert_eq!(value1, 42);

        let mut value2 = 100;
        clone2.apply(&mut value2);
        assert_eq!(value2, 100);
    }

    #[test]
    fn test_noop_chaining() {
        let noop = RcMutator::<i32>::noop();
        let double = RcMutator::new(|x: &mut i32| *x *= 2);

        let chained = noop.and_then(double.clone());

        let mut value = 5;
        chained.apply(&mut value);
        assert_eq!(value, 10);
    }

    // Note: RcMutator cannot be converted to ArcMutator because Rc is not
    // Send. This test has been removed.

    #[test]
    fn test_when() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let mut positive = 5;
        conditional.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut negative = -5;
        conditional.apply(&mut negative);
        assert_eq!(negative, -5); // unchanged
    }

    #[test]
    fn test_into_fn() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let closure = mutator.into_fn();

        let mut value = 5;
        closure(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_box() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let boxed = mutator.to_box();
        let mut value = 5;
        boxed.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_rc() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let rc = mutator.to_rc();
        let mut value = 5;
        rc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_to_fn() {
        let mutator = RcMutator::new(|x: &mut i32| *x += 10);
        let mut values = vec![1, 2, 3];
        values.iter_mut().for_each(mutator.to_fn());
        assert_eq!(values, vec![11, 12, 13]);
    }

    #[test]
    fn test_to_box_preserves_original() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let boxed = mutator.to_box();

        // Original still usable
        let mut value1 = 5;
        mutator.apply(&mut value1);
        assert_eq!(value1, 10);

        // Boxed version also works
        let mut value2 = 3;
        boxed.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_rc_preserves_original() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let rc = mutator.to_rc();

        // Original still usable
        let mut value1 = 5;
        mutator.apply(&mut value1);
        assert_eq!(value1, 10);

        // to_rc version also works
        let mut value2 = 3;
        rc.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_to_fn_preserves_original() {
        let mutator = RcMutator::new(|x: &mut i32| *x += 10);

        // to_fn version works
        let mut values = vec![1, 2, 3];
        values.iter_mut().for_each(mutator.to_fn());
        assert_eq!(values, vec![11, 12, 13]);

        // Original still usable after to_fn (because RcMutator is Clone)
        let mut value1 = 5;
        mutator.apply(&mut value1);
        assert_eq!(value1, 15);
    }

    #[test]
    fn test_conditional_into_box() {
        let conditional = RcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let boxed = conditional.into_box();

        let mut positive = 5;
        boxed.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_into_rc() {
        let conditional = RcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let rc = conditional.into_rc();

        let mut positive = 5;
        rc.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_into_fn() {
        let conditional = RcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let closure = conditional.into_fn();

        let mut positive = 5;
        closure(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_to_box() {
        let conditional = RcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let boxed = conditional.to_box();

        let mut positive = 5;
        boxed.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_to_rc() {
        let conditional = RcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let rc = conditional.to_rc();

        let mut positive = 5;
        rc.apply(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_to_fn() {
        let conditional = RcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let closure = conditional.to_fn();

        let mut positive = 5;
        closure(&mut positive);
        assert_eq!(positive, 10);
    }

    #[test]
    fn test_conditional_or_else() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2)
            .when(|x: &i32| *x > 0)
            .or_else(|x: &mut i32| *x = -*x);

        let mut positive = 10;
        mutator.apply(&mut positive);
        assert_eq!(positive, 20);

        let mut negative = -10;
        mutator.apply(&mut negative);
        assert_eq!(negative, 10);
    }

    #[test]
    fn test_conditional_clone() {
        let conditional = RcMutator::new(|x: &mut i32| *x *= 2).when(|x: &i32| *x > 0);
        let clone = conditional.clone();

        let mut positive = 5;
        conditional.apply(&mut positive);
        assert_eq!(positive, 10);

        let mut value2 = 3;
        clone.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_new_with_name() {
        let mutator = RcMutator::new_with_name("rc_test_mutator", |x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), Some("rc_test_mutator"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_some() {
        let mutator = RcMutator::new_with_optional_name(|x: &mut i32| *x += 1, Some("rc_optional".to_string()));
        assert_eq!(mutator.name(), Some("rc_optional"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_new_with_optional_name_none() {
        let mutator = RcMutator::new_with_optional_name(|x: &mut i32| *x += 1, None);
        assert_eq!(mutator.name(), None);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_name_and_set_name() {
        let mut mutator = RcMutator::new(|x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), None);

        mutator.set_name("rc_set_name");
        assert_eq!(mutator.name(), Some("rc_set_name"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }
}

// ============================================================================
// Closure Extension Methods Tests
// ============================================================================

#[cfg(test)]
mod test_fn_mutator_ops {
    use super::*;

    #[test]
    fn test_closure_mutate() {
        let closure = |x: &mut i32| *x *= 2;
        let mut value = 5;
        closure.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_and_then() {
        let chained = (|x: &mut i32| *x *= 2).and_then(|x: &mut i32| *x += 10);

        let mut value = 5;
        chained.apply(&mut value);
        assert_eq!(value, 20); // (5 * 2) + 10
    }

    #[test]
    fn test_closure_into_box() {
        let closure = |x: &mut i32| *x *= 2;
        let boxed = Mutator::into_box(closure);
        let mut value = 5;
        boxed.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_into_rc() {
        let closure = |x: &mut i32| *x *= 2;
        let rc = closure.into_rc();
        let mut value = 5;
        rc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_into_arc() {
        let closure = |x: &mut i32| *x *= 2;
        let arc = closure.into_arc();
        let mut value = 5;
        arc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_to_box() {
        // Test non-consuming conversion to BoxMutator
        // Note: Only works with cloneable closures (no mutable captures)
        let closure = |x: &mut i32| *x *= 2;
        let boxed = Mutator::to_box(&closure);
        let mut value = 5;
        boxed.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_to_rc() {
        // Test non-consuming conversion to RcMutator
        // Note: Only works with cloneable closures (no mutable captures)
        let closure = |x: &mut i32| *x *= 2;
        let rc = closure.to_rc();
        let mut value = 5;
        rc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_to_arc() {
        // Test non-consuming conversion to ArcMutator
        // Note: Only works with cloneable closures (no mutable captures)
        let closure = |x: &mut i32| *x *= 2;
        let arc = closure.to_arc();
        let mut value = 5;
        arc.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_closure_to_fn() {
        // Test non-consuming conversion to Fn
        // Note: Only works with cloneable closures (no mutable captures)
        let closure = |x: &mut i32| *x += 10;
        let mut values = vec![1, 2, 3];
        values.iter_mut().for_each(Mutator::to_fn(&closure));
        assert_eq!(values, vec![11, 12, 13]);
    }

    #[test]
    fn test_closure_to_rc_preserves_original() {
        let closure = |x: &mut i32| *x *= 2;
        let rc = closure.to_rc();

        // to_rc version works
        let mut value = 5;
        rc.apply(&mut value);
        assert_eq!(value, 10);

        // Original closure is still usable (was copied, not moved)
        let mut value2 = 3;
        let closure_copy = closure;
        closure_copy.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_closure_to_arc_preserves_original() {
        let closure = |x: &mut i32| *x *= 2;
        let arc = closure.to_arc();

        // to_arc version works
        let mut value = 5;
        arc.apply(&mut value);
        assert_eq!(value, 10);

        // Original closure is still usable (was copied, not moved)
        let mut value2 = 3;
        let closure_copy = closure;
        closure_copy.apply(&mut value2);
        assert_eq!(value2, 6);
    }

    #[test]
    fn test_closure_to_fn_preserves_original() {
        let closure = |x: &mut i32| *x += 10;
        let fn_mutator = Mutator::to_fn(&closure);

        // to_fn version works
        let mut values = vec![1, 2, 3];
        values.iter_mut().for_each(fn_mutator);
        assert_eq!(values, vec![11, 12, 13]);

        // Original closure is still usable (was copied, not moved)
        let mut value = 5;
        let closure_copy = closure;
        closure_copy.apply(&mut value);
        assert_eq!(value, 15);
    }

    #[test]
    fn test_closure_to_arc_thread_safe() {
        use std::thread;

        let closure = |x: &mut i32| *x *= 2;
        let arc = closure.to_arc();
        let clone = arc.clone();

        let handle = thread::spawn(move || {
            let mut value = 5;
            clone.apply(&mut value);
            value
        });

        let mut value = 3;
        arc.apply(&mut value);
        assert_eq!(value, 6);

        assert_eq!(handle.join().unwrap(), 10);
    }

    #[test]
    fn test_mutator_into_once() {
        let mutator = TestMutator::new(2);
        let once_mutator = mutator.into_once();
        let mut value = 5;
        once_mutator.apply(&mut value);
        assert_eq!(value, 10);
    }

    #[test]
    fn test_mutator_to_once() {
        let mutator = TestMutator::new(3);
        let once_mutator = mutator.to_once();
        let mut value = 4;
        once_mutator.apply(&mut value);
        assert_eq!(value, 12);
    }
}

// ============================================================================
// Unified Interface Tests
// ============================================================================

#[cfg(test)]
mod test_unified_interface {
    use super::*;

    fn apply_mutator<C: Mutator<i32>>(mutator: &C, value: i32) -> i32 {
        let mut val = value;
        mutator.apply(&mut val);
        val
    }

    #[test]
    fn test_with_box_mutator() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mutator, 5), 10);
    }

    #[test]
    fn test_with_arc_mutator() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mutator, 5), 10);
    }

    #[test]
    fn test_with_rc_mutator() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        assert_eq!(apply_mutator(&mutator, 5), 10);
    }

    #[test]
    fn test_with_closure() {
        let closure = |x: &mut i32| *x *= 2;
        assert_eq!(apply_mutator(&closure, 5), 10);
    }
}

// ============================================================================
// Conditional Mutator and_then Tests
// ============================================================================

#[cfg(test)]
mod test_conditional_mutator_and_then {
    use super::*;

    #[test]
    fn test_box_conditional_mutator_and_then_with_closure() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(|x: &mut i32| *x += 10);

        let mut positive = 5;
        chained.apply(&mut positive);
        assert_eq!(positive, 20); // 5 * 2 + 10

        let mut negative = -5;
        chained.apply(&mut negative);
        assert_eq!(negative, 5); // -5 + 10 (condition not met)
    }

    #[test]
    fn test_box_conditional_mutator_and_then_with_box_mutator() {
        let mutator1 = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator1.when(|x: &i32| *x > 0);
        let mutator2 = BoxMutator::new(|x: &mut i32| *x += 100);
        let chained = conditional.and_then(mutator2);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 120); // 10 * 2 + 100

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, 90); // -10 + 100 (condition not met)
    }

    #[test]
    fn test_rc_conditional_mutator_and_then_with_closure() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(|x: &mut i32| *x += 10);

        let mut positive = 5;
        chained.apply(&mut positive);
        assert_eq!(positive, 20); // 5 * 2 + 10

        let mut negative = -5;
        chained.apply(&mut negative);
        assert_eq!(negative, 5); // -5 + 10 (condition not met)
    }

    #[test]
    fn test_rc_conditional_mutator_and_then_with_rc_mutator() {
        let mutator1 = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator1.when(|x: &i32| *x > 0);
        let mutator2 = RcMutator::new(|x: &mut i32| *x += 100);
        let chained = conditional.and_then(mutator2);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 120); // 10 * 2 + 100

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, 90); // -10 + 100 (condition not met)
    }

    #[test]
    fn test_arc_conditional_mutator_and_then_with_closure() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);
        let chained = conditional.and_then(|x: &mut i32| *x += 10);

        let mut positive = 5;
        chained.apply(&mut positive);
        assert_eq!(positive, 20); // 5 * 2 + 10

        let mut negative = -5;
        chained.apply(&mut negative);
        assert_eq!(negative, 5); // -5 + 10 (condition not met)
    }

    #[test]
    fn test_arc_conditional_mutator_and_then_with_arc_mutator() {
        let mutator1 = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator1.when(|x: &i32| *x > 0);
        let mutator2 = ArcMutator::new(|x: &mut i32| *x += 100);
        let chained = conditional.and_then(mutator2);

        let mut positive = 10;
        chained.apply(&mut positive);
        assert_eq!(positive, 120); // 10 * 2 + 100

        let mut negative = -10;
        chained.apply(&mut negative);
        assert_eq!(negative, 90); // -10 + 100 (condition not met)
    }
}

// ============================================================================
// Conditional Mutator Debug/Display Tests
// ============================================================================

#[cfg(test)]
mod test_conditional_mutator_debug_display {
    use super::*;

    #[test]
    fn test_box_conditional_mutator_debug() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalMutator"));
        assert!(debug_str.contains("BoxMutator"));
        assert!(debug_str.contains("BoxPredicate"));
    }

    #[test]
    fn test_box_conditional_mutator_display() {
        let mutator = BoxMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalMutator"));
    }

    #[test]
    fn test_rc_conditional_mutator_debug() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("RcConditionalMutator"));
        assert!(debug_str.contains("RcMutator"));
        assert!(debug_str.contains("RcPredicate"));
    }

    #[test]
    fn test_rc_conditional_mutator_display() {
        let mutator = RcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("RcConditionalMutator"));
    }

    #[test]
    fn test_arc_conditional_mutator_debug() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("ArcConditionalMutator"));
        assert!(debug_str.contains("ArcMutator"));
        assert!(debug_str.contains("ArcPredicate"));
    }

    #[test]
    fn test_arc_conditional_mutator_display() {
        let mutator = ArcMutator::new(|x: &mut i32| *x *= 2);
        let conditional = mutator.when(|x: &i32| *x > 0);

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("ArcConditionalMutator"));
    }
}
