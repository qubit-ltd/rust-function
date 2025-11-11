/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/
//! # MutatorOnce Tests
//!
//! Tests the complete functionality of MutatorOnce trait and its implementations.

use prism3_function::{
    BoxMutatorOnce,
    FnMutatorOnceOps,
    MutatorOnce,
};

// Test closures specialization and default behaviors
#[test]
fn test_closure_into_and_to_variants() {
    let data = vec![1, 2, 3];
    let closure = move |x: &mut Vec<i32>| x.extend(data);

    // into_box consumes the closure and returns BoxMutatorOnce
    let boxed = closure.into_box();
    let mut v = vec![0];
    boxed.apply(&mut v);
    assert_eq!(v, vec![0, 1, 2, 3]);

    // Note: closure was moved - create another closure for to_box/to_fn
    let closure2 = move |x: &mut Vec<i32>| x.push(99);
    // to_box uses Clone; simple closure is zero-sized and Clone, so to_box exists
    let boxed2 = closure2.to_box();
    let mut v2 = vec![0];
    boxed2.apply(&mut v2);
    assert_eq!(v2, vec![0, 99]);

    // to_fn for cloneable closure
    let closure3 = move |x: &mut Vec<i32>| x.push(7);
    let f = closure3.to_fn();
    let mut v3 = vec![0];
    f(&mut v3);
    assert_eq!(v3, vec![0, 7]);
}

#[test]
fn test_box_mutator_once_identity_and_chain() {
    // identity: into_box should be identity for BoxMutatorOnce
    let m = BoxMutatorOnce::new(|x: &mut Vec<i32>| x.push(1));
    let m2 = m.into_box();
    let mut v = Vec::new();
    m2.apply(&mut v);
    assert_eq!(v, vec![1]);

    // chain
    let m1 = BoxMutatorOnce::new(|x: &mut Vec<i32>| x.push(2));
    let m2 = BoxMutatorOnce::new(|x: &mut Vec<i32>| x.push(3));
    let chained = m1.and_then(m2);
    let mut v2 = Vec::new();
    chained.apply(&mut v2);
    assert_eq!(v2, vec![2, 3]);
}

// Custom MutatorOnce using default into_box/into_fn/to_box/to_fn
struct MyMutatorOnce {
    data: Vec<i32>,
}

impl MutatorOnce<Vec<i32>> for MyMutatorOnce {
    fn apply(self, value: &mut Vec<i32>) {
        value.extend(self.data);
    }
}

#[test]
fn test_custom_mutator_default_adapters() {
    let my = MyMutatorOnce { data: vec![4, 5] };
    let boxed = my.into_box();
    let mut v = vec![0];
    boxed.apply(&mut v);
    assert_eq!(v, vec![0, 4, 5]);

    // to test to_box/to_fn we need a cloneable type
    #[derive(Clone)]
    struct CloneMutator {
        data: Vec<i32>,
    }
    impl MutatorOnce<Vec<i32>> for CloneMutator {
        fn apply(self, value: &mut Vec<i32>) {
            value.extend(self.data);
        }
    }

    let c = CloneMutator { data: vec![6] };
    let boxed_c = c.to_box();
    let mut v2 = vec![0];
    boxed_c.apply(&mut v2);
    assert_eq!(v2, vec![0, 6]);

    let c2 = CloneMutator { data: vec![8] };
    let f = c2.to_fn();
    let mut v3 = vec![0];
    f(&mut v3);
    assert_eq!(v3, vec![0, 8]);
}

// ============================================================================
// Tests for MutatorOnce trait default implementations
// ============================================================================

#[test]
fn test_mutator_once_default_into_fn() {
    // Test the default implementation of into_fn() for custom MutatorOnce types
    let my = MyMutatorOnce { data: vec![10, 20] };
    let f = my.into_fn();
    let mut v = vec![0];
    f(&mut v);
    assert_eq!(v, vec![0, 10, 20]);
}

// ============================================================================
// Tests for BoxMutatorOnce
// ============================================================================

#[test]
fn test_box_mutator_once_noop() {
    // Test that noop() creates a mutator that does nothing
    let noop = BoxMutatorOnce::<i32>::noop();
    let mut value = 42;
    noop.apply(&mut value);
    assert_eq!(value, 42); // Value should remain unchanged

    // Test with Vec
    let noop_vec = BoxMutatorOnce::<Vec<i32>>::noop();
    let mut vec = vec![1, 2, 3];
    noop_vec.apply(&mut vec);
    assert_eq!(vec, vec![1, 2, 3]); // Vec should remain unchanged
}

#[test]
fn test_box_mutator_once_when() {
    // Test when() with condition that passes
    let data = vec![1, 2, 3];
    let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data);
    });
    let conditional = mutator.when(|x: &Vec<i32>| !x.is_empty());

    let mut target = vec![0];
    conditional.apply(&mut target);
    assert_eq!(target, vec![0, 1, 2, 3]); // Should execute

    // Test when() with condition that fails
    let data2 = vec![4, 5];
    let mutator2 = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data2);
    });
    let conditional2 = mutator2.when(|x: &Vec<i32>| x.is_empty());

    let mut target2 = vec![0];
    conditional2.apply(&mut target2);
    assert_eq!(target2, vec![0]); // Should not execute
}

#[test]
fn test_box_mutator_once_into_fn() {
    // Test into_fn() for BoxMutatorOnce
    let mutator = BoxMutatorOnce::new(|x: &mut Vec<i32>| {
        x.push(100);
    });
    let f = mutator.into_fn();

    let mut v = vec![0];
    f(&mut v);
    assert_eq!(v, vec![0, 100]);
}

// ============================================================================
// Tests for BoxConditionalMutatorOnce
// ============================================================================

#[test]
fn test_box_conditional_mutator_once_mutate() {
    // Test mutate() when condition is true
    let data = vec![1, 2];
    let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data);
    });
    let conditional = mutator.when(|x: &Vec<i32>| x.len() < 5);

    let mut target = vec![0];
    conditional.apply(&mut target);
    assert_eq!(target, vec![0, 1, 2]);

    // Test mutate() when condition is false
    let data2 = vec![3, 4];
    let mutator2 = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data2);
    });
    let conditional2 = mutator2.when(|x: &Vec<i32>| x.len() > 10);

    let mut target2 = vec![0];
    conditional2.apply(&mut target2);
    assert_eq!(target2, vec![0]); // Should remain unchanged
}

#[test]
fn test_box_conditional_mutator_once_into_box() {
    // Test into_box() conversion
    let data = vec![5, 6];
    let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data);
    });
    let conditional = mutator.when(|x: &Vec<i32>| !x.is_empty());
    let boxed = conditional.into_box();

    let mut target = vec![0];
    boxed.apply(&mut target);
    assert_eq!(target, vec![0, 5, 6]);

    // Test with failing condition
    let data2 = vec![7, 8];
    let mutator2 = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data2);
    });
    let conditional2 = mutator2.when(|x: &Vec<i32>| x.is_empty());
    let boxed2 = conditional2.into_box();

    let mut target2 = vec![0];
    boxed2.apply(&mut target2);
    assert_eq!(target2, vec![0]); // Should remain unchanged
}

#[test]
fn test_box_conditional_mutator_once_into_fn() {
    // Test into_fn() conversion when condition is true
    let data = vec![9, 10];
    let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data);
    });
    let conditional = mutator.when(|x: &Vec<i32>| x.len() < 10);
    let f = conditional.into_fn();

    let mut target = vec![0];
    f(&mut target);
    assert_eq!(target, vec![0, 9, 10]);

    // Test into_fn() conversion when condition is false
    let data2 = vec![11, 12];
    let mutator2 = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data2);
    });
    let conditional2 = mutator2.when(|x: &Vec<i32>| x.len() > 10);
    let f2 = conditional2.into_fn();

    let mut target2 = vec![0];
    f2(&mut target2);
    assert_eq!(target2, vec![0]); // Should remain unchanged due to condition being false
}

#[test]
fn test_box_conditional_mutator_once_and_then() {
    // Test and_then() to chain conditional mutators
    let data1 = vec![1, 2];
    let cond1 = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data1);
    })
    .when(|x: &Vec<i32>| !x.is_empty());

    let data2 = vec![3, 4];
    let cond2 = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data2);
    })
    .when(|x: &Vec<i32>| x.len() < 10);

    let chained = cond1.and_then(cond2);

    let mut target = vec![0];
    chained.apply(&mut target);
    assert_eq!(target, vec![0, 1, 2, 3, 4]);

    // Test with one condition failing
    let data3 = vec![5, 6];
    let cond3 = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data3);
    })
    .when(|x: &Vec<i32>| x.is_empty()); // This will fail

    let data4 = vec![7, 8];
    let cond4 = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data4);
    })
    .when(|x: &Vec<i32>| x.len() < 10); // This will pass

    let chained2 = cond3.and_then(cond4);

    let mut target2 = vec![0];
    chained2.apply(&mut target2);
    assert_eq!(target2, vec![0, 7, 8]); // Only second mutator executes
}

#[test]
fn test_box_conditional_mutator_once_or_else() {
    // Test or_else() with condition true (when branch executes)
    let data1 = vec![1, 2, 3];
    let data2 = vec![99];
    let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data1);
    })
    .when(|x: &Vec<i32>| !x.is_empty())
    .or_else(move |x: &mut Vec<i32>| {
        x.extend(data2);
    });

    let mut target = vec![0];
    mutator.apply(&mut target);
    assert_eq!(target, vec![0, 1, 2, 3]); // when branch executes

    // Test or_else() with condition false (or_else branch executes)
    let data3 = vec![4, 5];
    let data4 = vec![99];
    let mutator2 = BoxMutatorOnce::new(move |x: &mut Vec<i32>| {
        x.extend(data3);
    })
    .when(|x: &Vec<i32>| x.is_empty())
    .or_else(move |x: &mut Vec<i32>| {
        x.extend(data4);
    });

    let mut target2 = vec![0];
    mutator2.apply(&mut target2);
    assert_eq!(target2, vec![0, 99]); // or_else branch executes
}

// ============================================================================
// Tests for closure implementations
// ============================================================================

#[test]
fn test_closure_into_fn() {
    // Test into_fn() for closures
    let data = vec![1, 2, 3];
    let closure = move |x: &mut Vec<i32>| x.extend(data);
    let f = closure.into_fn();

    let mut v = vec![0];
    f(&mut v);
    assert_eq!(v, vec![0, 1, 2, 3]);
}

#[test]
fn test_closure_and_then() {
    // Test and_then() from FnMutatorOnceOps trait
    let data1 = vec![1, 2];
    let data2 = vec![3, 4];

    let chained =
        (move |x: &mut Vec<i32>| x.extend(data1)).and_then(move |x: &mut Vec<i32>| x.extend(data2));

    let mut target = vec![0];
    chained.apply(&mut target);
    assert_eq!(target, vec![0, 1, 2, 3, 4]);

    // Test chaining multiple closures
    let data3 = vec![5];
    let data4 = vec![6];
    let data5 = vec![7];

    let multi_chained = (move |x: &mut Vec<i32>| x.extend(data3))
        .and_then(move |x: &mut Vec<i32>| x.extend(data4))
        .and_then(move |x: &mut Vec<i32>| x.extend(data5));

    let mut target2 = vec![0];
    multi_chained.apply(&mut target2);
    assert_eq!(target2, vec![0, 5, 6, 7]);
}

// ============================================================================
// BoxConditionalMutatorOnce Debug/Display Tests
// ============================================================================

#[cfg(test)]
mod test_box_conditional_mutator_once_debug_display {
    use super::*;

    #[test]
    fn test_box_conditional_mutator_once_debug() {
        let data = vec![1, 2];
        let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| x.extend(data));
        let conditional = mutator.when(|x: &Vec<i32>| x.len() < 5);

        let debug_str = format!("{:?}", conditional);
        assert!(debug_str.contains("BoxConditionalMutatorOnce"));
        assert!(debug_str.contains("BoxMutatorOnce"));
        assert!(debug_str.contains("BoxPredicate"));
    }

    #[test]
    fn test_box_conditional_mutator_once_display() {
        let data = vec![3, 4];
        let mutator = BoxMutatorOnce::new(move |x: &mut Vec<i32>| x.extend(data));
        let conditional = mutator.when(|x: &Vec<i32>| !x.is_empty());

        let display_str = format!("{}", conditional);
        assert!(display_str.contains("BoxConditionalMutatorOnce"));
    }

    #[test]
    fn test_box_mutator_once_new_with_name() {
        let mutator = BoxMutatorOnce::new_with_name("test_mutator_once", |x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), Some("test_mutator_once"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_box_mutator_once_new_with_optional_name_some() {
        let mutator = BoxMutatorOnce::new_with_optional_name(
            |x: &mut i32| *x += 1,
            Some("optional_once".to_string()),
        );
        assert_eq!(mutator.name(), Some("optional_once"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_box_mutator_once_new_with_optional_name_none() {
        let mutator = BoxMutatorOnce::new_with_optional_name(|x: &mut i32| *x += 1, None);
        assert_eq!(mutator.name(), None);

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }

    #[test]
    fn test_box_mutator_once_name_and_set_name() {
        let mut mutator = BoxMutatorOnce::new(|x: &mut i32| *x += 1);
        assert_eq!(mutator.name(), None);

        mutator.set_name("set_name_once");
        assert_eq!(mutator.name(), Some("set_name_once"));

        let mut value = 5;
        mutator.apply(&mut value);
        assert_eq!(value, 6);
    }
}

// Tests for to_box() method
#[cfg(test)]
mod custom_mutator_to_methods_tests {
    use super::*;
    use std::sync::{Arc, Mutex};

    /// Cloneable custom mutator for testing to_xxx() methods
    #[derive(Clone)]
    struct CloneableMutator {
        log: Arc<Mutex<Vec<i32>>>,
        multiplier: i32,
    }

    impl CloneableMutator {
        fn new(log: Arc<Mutex<Vec<i32>>>, multiplier: i32) -> Self {
            Self { log, multiplier }
        }
    }

    impl MutatorOnce<i32> for CloneableMutator {
        fn apply(self, value: &mut i32) {
            *value *= self.multiplier;
            self.log.lock().unwrap().push(*value);
        }
    }

    #[test]
    fn test_custom_mutator_to_box() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let mutator = CloneableMutator::new(log.clone(), 2);
        let boxed = mutator.to_box();
        let mut value = 7;
        boxed.apply(&mut value);
        assert_eq!(value, 14);
        assert_eq!(*log.lock().unwrap(), vec![14]);
    }

    #[test]
    fn test_custom_mutator_to_box_multiple_times() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let mutator = CloneableMutator::new(log.clone(), 3);

        // to_box() should allow being called multiple times since the mutator is Clone
        let boxed1 = mutator.to_box();
        let boxed2 = mutator.to_box();

        let mut value1 = 6;
        boxed1.apply(&mut value1);
        assert_eq!(value1, 18);

        let mut value2 = 4;
        boxed2.apply(&mut value2);
        assert_eq!(value2, 12);

        assert_eq!(*log.lock().unwrap(), vec![18, 12]);
    }

    #[test]
    fn test_custom_mutator_to_box_original_still_usable() {
        let log = Arc::new(Mutex::new(Vec::new()));
        let mutator = CloneableMutator::new(log.clone(), 2);

        let boxed = mutator.to_box();
        let mut value1 = 5;
        boxed.apply(&mut value1);
        assert_eq!(value1, 10);

        // Original mutator should still be usable
        let mut value2 = 10;
        mutator.apply(&mut value2);
        assert_eq!(value2, 20);

        assert_eq!(*log.lock().unwrap(), vec![10, 20]);
    }
}
