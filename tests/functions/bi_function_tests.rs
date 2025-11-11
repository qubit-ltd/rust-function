/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

//! Comprehensive tests for BiFunction trait and its implementations

use prism3_function::{
    ArcBiFunction,
    ArcBiPredicate,
    BiFunction,
    BiFunctionOnce,
    BoxBiFunction,
    RcBiFunction,
    RcBiPredicate,
    FnBiFunctionOps,
};

// ============================================================================
// BiFunction Trait Tests - Core Functionality
// ============================================================================

#[test]
fn test_bi_function_trait_apply() {
    // Test that BiFunction trait's apply method works correctly
    let add = |x: &i32, y: &i32| *x + *y;
    assert_eq!(add.apply(&20, &22), 42);
    assert_eq!(add.apply(&0, &0), 0);
    assert_eq!(add.apply(&-10, &5), -5);
}

#[test]
fn test_bi_function_trait_into_box() {
    // Test conversion from closure to BoxBiFunction
    let add = |x: &i32, y: &i32| *x + *y;
    let boxed = BiFunction::into_box(add);
    assert_eq!(boxed.apply(&20, &22), 42);
}

#[test]
fn test_bi_function_trait_into_rc() {
    // Test conversion from closure to RcBiFunction
    let add = |x: &i32, y: &i32| *x + *y;
    let rc = add.into_rc();
    assert_eq!(rc.apply(&20, &22), 42);
}

#[test]
fn test_bi_function_trait_into_arc() {
    // Test conversion from closure to ArcBiFunction
    let add = |x: &i32, y: &i32| *x + *y;
    let arc = add.into_arc();
    assert_eq!(arc.apply(&20, &22), 42);
}

#[test]
fn test_bi_function_trait_into_fn() {
    // Test conversion to closure
    let add = |x: &i32, y: &i32| *x + *y;
    let func = BiFunction::into_fn(add);
    assert_eq!(func(&20, &22), 42);
}

#[test]
fn test_bi_function_trait_into_once() {
    // Test conversion to BiFunctionOnce
    let add = |x: &i32, y: &i32| *x + *y;
    let once = add.into_once();
    assert_eq!(once.apply(&20, &22), 42);
}

// ============================================================================
// Custom BiFunction Implementation Tests - Test Trait Default Methods
// ============================================================================

#[test]
fn test_custom_bi_function_default_methods() {
    // Test BiFunction trait default methods on custom implementation
    #[derive(Debug)]
    struct CustomBiFunction {
        multiplier: i32,
    }

    impl BiFunction<i32, i32, i32> for CustomBiFunction {
        fn apply(&self, first: &i32, second: &i32) -> i32 {
            first * second * self.multiplier
        }
    }

    let custom_func = CustomBiFunction { multiplier: 3 };

    // Test default into_box method
    let boxed = custom_func.into_box();
    assert_eq!(boxed.apply(&2, &4), 24); // 2 * 4 * 3 = 24
}

#[test]
fn test_cloneable_bi_function_default_methods() {
    // Test BiFunction trait default methods on cloneable implementation
    #[derive(Clone, Debug)]
    struct CloneableBiFunction {
        multiplier: i32,
    }

    impl BiFunction<i32, i32, i32> for CloneableBiFunction {
        fn apply(&self, first: &i32, second: &i32) -> i32 {
            first * second * self.multiplier
        }
    }

    let custom_func = CloneableBiFunction { multiplier: 2 };

    // Test default to_box method (requires Clone)
    let boxed = custom_func.to_box();
    assert_eq!(boxed.apply(&3, &5), 30); // 3 * 5 * 2 = 30

    // Test default to_rc method (requires Clone)
    let rc_func = custom_func.to_rc();
    assert_eq!(rc_func.apply(&3, &5), 30);

    // Test default to_fn method (requires Clone)
    let func = custom_func.to_fn();
    assert_eq!(func(&3, &5), 30);

    // Test default to_once method (requires Clone)
    let once = custom_func.to_once();
    assert_eq!(once.apply(&3, &5), 30);
}

#[test]
fn test_thread_safe_bi_function_default_methods() {
    // Test BiFunction trait default methods on thread-safe implementation
    #[derive(Clone, Debug)]
    struct ThreadSafeBiFunction {
        multiplier: i32,
    }

    impl BiFunction<i32, i32, i32> for ThreadSafeBiFunction {
        fn apply(&self, first: &i32, second: &i32) -> i32 {
            first * second * self.multiplier
        }
    }

    let custom_func = ThreadSafeBiFunction { multiplier: 4 };

    // Test default to_arc method (requires Clone + Send + Sync)
    let arc_func = custom_func.to_arc();
    assert_eq!(arc_func.apply(&2, &3), 24); // 2 * 3 * 4 = 24

    // Test default into_arc method
    let arc_func2 = custom_func.into_arc();
    assert_eq!(arc_func2.apply(&2, &3), 24);
}

// ============================================================================
// BoxBiFunction Tests
// ============================================================================

#[test]
fn test_box_bi_function_new() {
    let add = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    assert_eq!(add.apply(&10, &15), 25);
}

#[test]
fn test_box_bi_function_constant() {
    let constant = BoxBiFunction::constant(42);
    assert_eq!(constant.apply(&1, &2), 42);
    assert_eq!(constant.apply(&100, &200), 42);
}

// BoxBiFunction doesn't implement Clone
// #[test]
// fn test_box_bi_function_clone() {
//     let original = BoxBiFunction::new(|x: &i32, y: &i32| *x * *y);
//     let cloned = original.clone();
//     assert_eq!(original.apply(&6, &7), 42);
//     assert_eq!(cloned.apply(&6, &7), 42);
// }

#[test]
fn test_box_bi_function_debug_display() {
    let func = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let debug_str = format!("{:?}", func);
    assert!(debug_str.contains("BoxBiFunction"));
    let display_str = format!("{}", func);
    assert!(display_str.starts_with("BoxBiFunction"));
}

// ============================================================================
// RcBiFunction Tests
// ============================================================================

#[test]
fn test_rc_bi_function_new() {
    let multiply = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    assert_eq!(multiply.apply(&6, &7), 42);
}

#[test]
fn test_rc_bi_function_constant() {
    let constant = RcBiFunction::constant(100);
    assert_eq!(constant.apply(&1, &2), 100);
}

#[test]
fn test_rc_bi_function_clone() {
    let original = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let cloned = original.clone();
    assert_eq!(original.apply(&10, &20), 30);
    assert_eq!(cloned.apply(&10, &20), 30);
}

#[test]
fn test_rc_bi_function_debug_display() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let debug_str = format!("{:?}", func);
    assert!(debug_str.contains("RcBiFunction"));
    let display_str = format!("{}", func);
    assert!(display_str.starts_with("RcBiFunction"));
}

// ============================================================================
// ArcBiFunction Tests
// ============================================================================

#[test]
fn test_arc_bi_function_new() {
    let divide = ArcBiFunction::new(|x: &i32, y: &i32| *x / *y);
    assert_eq!(divide.apply(&42, &2), 21);
}

#[test]
fn test_arc_bi_function_constant() {
    let constant = ArcBiFunction::constant("hello".to_string());
    assert_eq!(constant.apply(&1, &2), "hello");
}

#[test]
fn test_arc_bi_function_clone() {
    let original = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let cloned = original.clone();
    assert_eq!(original.apply(&50, &8), 42);
    assert_eq!(cloned.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_debug_display() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let debug_str = format!("{:?}", func);
    assert!(debug_str.contains("ArcBiFunction"));
    let display_str = format!("{}", func);
    assert!(display_str.starts_with("ArcBiFunction"));
}

// ============================================================================
// Conversion Tests
// ============================================================================

#[test]
fn test_bi_function_conversions() {
    let add = |x: &i32, y: &i32| *x + *y;

    // Test to_box
    let boxed = BiFunction::to_box(&add);
    assert_eq!(boxed.apply(&10, &20), 30);

    // Test to_rc
    let rc = BiFunction::to_rc(&add);
    assert_eq!(rc.apply(&10, &20), 30);

    // Test to_arc
    let arc = BiFunction::to_arc(&add);
    assert_eq!(arc.apply(&10, &20), 30);

    // Test to_fn
    let func = BiFunction::to_fn(&add);
    assert_eq!(func(&10, &20), 30);

    // Test to_once
    let once = add.to_once();
    assert_eq!(once.apply(&10, &20), 30);
}

// ============================================================================
// BiFunction Composition Tests
// ============================================================================

#[test]
fn test_bi_function_and_then() {
    use prism3_function::FnBiFunctionOps;

    let add = |x: &i32, y: &i32| *x + *y;
    let double = |x: &i32| *x * 2;

    let composed = add.and_then(double);
    assert_eq!(composed.apply(&10, &15), 50); // (10 + 15) * 2 = 50
}

#[test]
fn test_bi_function_when_or_else() {
    use prism3_function::FnBiFunctionOps;

    let add = |x: &i32, y: &i32| *x + *y;
    let multiply = |x: &i32, y: &i32| *x * *y;

    let conditional = add
        .when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(multiply);

    assert_eq!(conditional.apply(&5, &3), 8); // add: 5 + 3 = 8
    assert_eq!(conditional.apply(&-5, &3), -15); // multiply: -5 * 3 = -15
}

// ============================================================================
// Integration Tests
// ============================================================================

#[test]
fn test_bi_function_with_complex_types() {
    let concat = |s1: &&str, s2: &&str| format!("{} {}", *s1, *s2);
    let boxed = BoxBiFunction::new(concat);

    assert_eq!(boxed.apply(&"Hello", &"World"), "Hello World");
}

#[test]
fn test_bi_function_with_option_types() {
    let combine_options = |opt1: &Option<i32>, opt2: &Option<i32>| match (opt1, opt2) {
        (Some(a), Some(b)) => Some(a + b),
        _ => None,
    };

    let func = RcBiFunction::new(combine_options);

    assert_eq!(func.apply(&Some(10), &Some(20)), Some(30));
    assert_eq!(func.apply(&Some(10), &None), None);
    assert_eq!(func.apply(&None, &Some(20)), None);
}

#[test]
fn test_bi_function_with_result_types() {
    let safe_divide = |a: &i32, b: &i32| {
        if *b == 0 {
            Err("Division by zero")
        } else {
            Ok(*a / *b)
        }
    };

    let func = ArcBiFunction::new(safe_divide);

    assert_eq!(func.apply(&10, &2), Ok(5));
    assert_eq!(func.apply(&10, &0), Err("Division by zero"));
}

// ============================================================================
// BoxBiFunction Extended Tests
// ============================================================================

#[test]
fn test_box_bi_function_new_with_name() {
    let func = BoxBiFunction::new_with_name("adder", |x: &i32, y: &i32| *x + *y);
    assert_eq!(func.name(), Some("adder"));
    assert_eq!(func.apply(&10, &20), 30);
}

#[test]
fn test_box_bi_function_new_with_optional_name() {
    let func1 = BoxBiFunction::new_with_optional_name(|x: &i32, y: &i32| *x + *y, Some("named".to_string()));
    let func2 = BoxBiFunction::new_with_optional_name(|x: &i32, y: &i32| *x + *y, None);

    assert_eq!(func1.name(), Some("named"));
    assert_eq!(func2.name(), None);
    assert_eq!(func1.apply(&5, &7), 12);
    assert_eq!(func2.apply(&5, &7), 12);
}

#[test]
fn test_box_bi_function_name_and_set_name() {
    let mut func = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    assert_eq!(func.name(), None);

    func.set_name("test_func");
    assert_eq!(func.name(), Some("test_func"));

    func.set_name("updated_name");
    assert_eq!(func.name(), Some("updated_name"));
}

#[test]
fn test_box_bi_function_into_box() {
    let func = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let boxed = func.into_box();
    assert_eq!(boxed.apply(&1, &2), 3);
}

#[test]
fn test_box_bi_function_into_rc() {
    let func = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let rc = func.into_rc();
    assert_eq!(rc.apply(&1, &2), 3);
}

// BoxBiFunction doesn't implement Clone, so to_* methods are not available
// The following methods are tested via the trait implementations:
// - into_box() is tested above
// - to_box(), to_rc(), to_arc(), to_fn(), to_once() require Clone trait

// ============================================================================
// RcBiFunction Extended Tests
// ============================================================================

#[test]
fn test_rc_bi_function_new_with_name() {
    let func = RcBiFunction::new_with_name("multiplier", |x: &i32, y: &i32| *x * *y);
    assert_eq!(func.name(), Some("multiplier"));
    assert_eq!(func.apply(&6, &7), 42);
}

#[test]
fn test_rc_bi_function_new_with_optional_name() {
    let func1 = RcBiFunction::new_with_optional_name(|x: &i32, y: &i32| *x * *y, Some("named".to_string()));
    let func2 = RcBiFunction::new_with_optional_name(|x: &i32, y: &i32| *x * *y, None);

    assert_eq!(func1.name(), Some("named"));
    assert_eq!(func2.name(), None);
    assert_eq!(func1.apply(&6, &7), 42);
    assert_eq!(func2.apply(&6, &7), 42);
}

#[test]
fn test_rc_bi_function_name_and_set_name() {
    let mut func = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    assert_eq!(func.name(), None);

    func.set_name("test_func");
    assert_eq!(func.name(), Some("test_func"));
}

#[test]
fn test_rc_bi_function_into_box() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    let boxed = func.into_box();
    assert_eq!(boxed.apply(&6, &7), 42);
}

#[test]
fn test_rc_bi_function_into_rc() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    let rc = func.into_rc();
    assert_eq!(rc.apply(&6, &7), 42);
}

// RcBiFunction cannot be converted to ArcBiFunction (not Send + Sync)

#[test]
fn test_rc_bi_function_into_fn() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    let closure = func.into_fn();
    assert_eq!(closure(&6, &7), 42);
}

#[test]
fn test_rc_bi_function_into_once() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    let once = func.into_once();
    assert_eq!(once.apply(&6, &7), 42);
}

#[test]
fn test_rc_bi_function_to_box() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    let boxed = func.to_box();
    assert_eq!(boxed.apply(&6, &7), 42);
    assert_eq!(func.apply(&6, &7), 42);
}

#[test]
fn test_rc_bi_function_to_rc() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    let rc = func.to_rc();
    assert_eq!(rc.apply(&6, &7), 42);
    assert_eq!(func.apply(&6, &7), 42);
}

// RcBiFunction cannot be converted to ArcBiFunction (not Send + Sync)

#[test]
fn test_rc_bi_function_to_fn() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    let closure = func.to_fn();
    assert_eq!(closure(&6, &7), 42);
    assert_eq!(func.apply(&6, &7), 42);
}

#[test]
fn test_rc_bi_function_to_once() {
    let func = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
    let once = func.to_once();
    assert_eq!(once.apply(&6, &7), 42);
    assert_eq!(func.apply(&6, &7), 42);
}

// ============================================================================
// ArcBiFunction Extended Tests
// ============================================================================

#[test]
fn test_arc_bi_function_new_with_name() {
    let func = ArcBiFunction::new_with_name("divider", |x: &i32, y: &i32| *x / *y);
    assert_eq!(func.name(), Some("divider"));
    assert_eq!(func.apply(&42, &2), 21);
}

#[test]
fn test_arc_bi_function_new_with_optional_name() {
    let func1 = ArcBiFunction::new_with_optional_name(|x: &i32, y: &i32| *x / *y, Some("named".to_string()));
    let func2 = ArcBiFunction::new_with_optional_name(|x: &i32, y: &i32| *x / *y, None);

    assert_eq!(func1.name(), Some("named"));
    assert_eq!(func2.name(), None);
    assert_eq!(func1.apply(&42, &2), 21);
    assert_eq!(func2.apply(&42, &2), 21);
}

#[test]
fn test_arc_bi_function_name_and_set_name() {
    let mut func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    assert_eq!(func.name(), None);

    func.set_name("test_func");
    assert_eq!(func.name(), Some("test_func"));
}

#[test]
fn test_arc_bi_function_into_box() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let boxed = func.into_box();
    assert_eq!(boxed.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_into_rc() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let rc = func.into_rc();
    assert_eq!(rc.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_into_arc() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let arc = func.into_arc();
    assert_eq!(arc.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_into_fn() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let closure = func.into_fn();
    assert_eq!(closure(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_into_once() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let once = func.into_once();
    assert_eq!(once.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_to_box() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let boxed = func.to_box();
    assert_eq!(boxed.apply(&50, &8), 42);
    assert_eq!(func.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_to_rc() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let rc = func.to_rc();
    assert_eq!(rc.apply(&50, &8), 42);
    assert_eq!(func.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_to_arc() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let arc = func.to_arc();
    assert_eq!(arc.apply(&50, &8), 42);
    assert_eq!(func.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_to_fn() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let closure = func.to_fn();
    assert_eq!(closure(&50, &8), 42);
    assert_eq!(func.apply(&50, &8), 42);
}

#[test]
fn test_arc_bi_function_to_once() {
    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
    let once = func.to_once();
    assert_eq!(once.apply(&50, &8), 42);
    assert_eq!(func.apply(&50, &8), 42);
}

// ============================================================================
// Conditional BiFunction Tests
// ============================================================================

#[test]
fn test_box_conditional_bi_function_when_or_else() {
    let add = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let multiply = BoxBiFunction::new(|x: &i32, y: &i32| *x * *y);

    let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
    assert_eq!(conditional.apply(&3, &4), 7); // when branch: 3 + 4 = 7
    assert_eq!(conditional.apply(&-3, &4), -12); // or_else branch: -3 * 4 = -12
}

#[test]
fn test_rc_conditional_bi_function_when_or_else() {
    let add = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let multiply = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);

    let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
    assert_eq!(conditional.apply(&3, &4), 7); // when branch
    assert_eq!(conditional.apply(&-3, &4), -12); // or_else branch
}

#[test]
fn test_arc_conditional_bi_function_when_or_else() {
    let add = ArcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let multiply = ArcBiFunction::new(|x: &i32, y: &i32| *x * *y);

    let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
    assert_eq!(conditional.apply(&3, &4), 7); // when branch
    assert_eq!(conditional.apply(&-3, &4), -12); // or_else branch
}

#[test]
fn test_conditional_bi_function_with_complex_predicates() {
    let add = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let subtract = BoxBiFunction::new(|x: &i32, y: &i32| *x - *y);

    let conditional = add.when(|x: &i32, y: &i32| *x >= *y).or_else(subtract);
    assert_eq!(conditional.apply(&5, &3), 8); // when branch: 5 >= 3, so 5 + 3 = 8
    assert_eq!(conditional.apply(&3, &5), -2); // or_else branch: 3 < 5, so 3 - 5 = -2
}

#[test]
fn test_conditional_bi_function_display_debug() {
    let add = BoxBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let conditional = add.when(|x: &i32, _y: &i32| *x > 0);

    let display = format!("{}", conditional);
    assert!(display.contains("BoxConditionalBiFunction"));

    let debug = format!("{:?}", conditional);
    assert!(debug.contains("BoxConditionalBiFunction"));
}

#[test]
fn test_rc_conditional_bi_function_clone() {
    let add = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let multiply = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);

    let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
    let cloned = conditional.clone();

    // Test original
    assert_eq!(conditional.apply(&3, &4), 7); // when branch: 3 + 4 = 7
    assert_eq!(conditional.apply(&-3, &4), -12); // or_else branch: -3 * 4 = -12

    // Test cloned (should behave identically)
    assert_eq!(cloned.apply(&3, &4), 7); // when branch: 3 + 4 = 7
    assert_eq!(cloned.apply(&-3, &4), -12); // or_else branch: -3 * 4 = -12
}

#[test]
fn test_arc_conditional_bi_function_clone() {
    let add = ArcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let multiply = ArcBiFunction::new(|x: &i32, y: &i32| *x * *y);

    let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0).or_else(multiply);
    let cloned = conditional.clone();

    // Test original
    assert_eq!(conditional.apply(&3, &4), 7); // when branch: 3 + 4 = 7
    assert_eq!(conditional.apply(&-3, &4), -12); // or_else branch: -3 * 4 = -12

    // Test cloned (should behave identically)
    assert_eq!(cloned.apply(&3, &4), 7); // when branch: 3 + 4 = 7
    assert_eq!(cloned.apply(&-3, &4), -12); // or_else branch: -3 * 4 = -12
}

#[test]
fn test_impl_conditional_function_clone_three_params_macro_coverage() {
    println!("Starting test_impl_conditional_function_clone_three_params_macro_coverage");

    // Test to ensure the three-parameter version of impl_conditional_function_clone macro is covered
    // This test verifies that the macro generates Clone implementations for three-parameter structs
    // by testing that RcConditionalBiFunction and ArcConditionalBiFunction implement Clone

    // Test RcConditionalBiFunction (three parameters: T, U, R)
    {
        println!("Testing RcConditionalBiFunction with macro-generated Clone (three parameters)");
        let add = RcBiFunction::new(|x: &i32, y: &i32| *x + *y);
        let pred = RcBiPredicate::new(|x: &i32, y: &i32| *x > 0 && *y > 0);

        let conditional_rc = add.when(pred);

        println!("Calling clone() on RcConditionalBiFunction - this should trigger macro-generated three-param code");
        let cloned_rc = conditional_rc.clone();
        println!("Clone completed for RcConditionalBiFunction");

        // Create or_else to test functionality
        let multiply = RcBiFunction::new(|x: &i32, y: &i32| *x * *y);
        let func = cloned_rc.or_else(multiply);

        // Verify functionality
        assert_eq!(func.apply(&3, &4), 7); // when branch
        assert_eq!(func.apply(&-3, &4), -12); // or_else branch: -3 * 4 = -12
        println!("RcConditionalBiFunction test passed");
    }

    // Test ArcConditionalBiFunction (three parameters: T, U, R)
    {
        println!("Testing ArcConditionalBiFunction with macro-generated Clone (three parameters)");
        let subtract = ArcBiFunction::new(|x: &i32, y: &i32| *x - *y);
        let pred = ArcBiPredicate::new(|x: &i32, y: &i32| *x >= *y);

        let conditional_arc = subtract.when(pred);

        println!("Calling clone() on ArcConditionalBiFunction - this should trigger macro-generated three-param code");
        let cloned_arc = conditional_arc.clone();
        println!("Clone completed for ArcConditionalBiFunction");

        // Create or_else to test functionality
        let negate = ArcBiFunction::new(|x: &i32, y: &i32| -*x - *y);
        let func = cloned_arc.or_else(negate);

        // Verify functionality
        assert_eq!(func.apply(&5, &3), 2); // when branch: 5 - 3 = 2
        assert_eq!(func.apply(&3, &5), -8); // or_else branch: -(3 + 5) = -8
        println!("ArcConditionalBiFunction test passed");
    }

    println!("Three-parameter conditional clone macro test passed!");
}

// ============================================================================
// Advanced Composition Tests
// ============================================================================

#[test]
fn test_bi_function_complex_composition() {
    let add = |x: &i32, y: &i32| *x + *y;
    let multiply_by_two = |x: &i32| *x * 2;
    let to_string = |x: &i32| x.to_string();

    // Chain: add -> multiply_by_two -> to_string
    let composed = add.and_then(multiply_by_two).and_then(to_string);
    assert_eq!(composed.apply(&3, &4), "14"); // ((3 + 4) * 2).to_string()
}

#[test]
fn test_bi_function_conditional_composition() {
    let add = |x: &i32, y: &i32| *x + *y;
    let multiply = |x: &i32, y: &i32| *x * *y;
    let square = |x: &i32| *x * *x;

    // If both positive, add then square; otherwise multiply then square
    let conditional = add.when(|x: &i32, y: &i32| *x > 0 && *y > 0)
        .or_else(multiply)
        .and_then(square);

    assert_eq!(conditional.apply(&3, &4), 49); // (3 + 4)^2 = 49
    assert_eq!(conditional.apply(&-3, &4), 144); // (-3 * 4)^2 = 144
}

// ============================================================================
// Thread Safety Tests for ArcBiFunction
// ============================================================================

#[test]
fn test_arc_bi_function_thread_safety() {
    use std::thread;

    let func = ArcBiFunction::new(|x: &i32, y: &i32| *x + *y);
    let func_clone = func.clone();

    let handle = thread::spawn(move || {
        func_clone.apply(&10, &20)
    });

    let result = handle.join().unwrap();
    assert_eq!(result, 30);
    assert_eq!(func.apply(&10, &20), 30);
}
