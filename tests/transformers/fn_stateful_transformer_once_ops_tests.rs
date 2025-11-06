/*******************************************************************************
 *
 *    Copyright (c) 2025.
 *    3-Prism Co. Ltd.
 *
 *    All rights reserved.
 *
 ******************************************************************************/

use prism3_function::{
    BoxTransformerOnce,
    FnTransformerOnceOps,
    FnTransformerOps,
    Transformer,
    TransformerOnce,
};

// ============================================================================
// FnTransformerOnceOps Tests - Extension trait for closure transformers
// ============================================================================

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_closure_and_then() {
        let parse = |s: String| s.parse::<i32>().unwrap_or(0);
        let double = |x: i32| x * 2;

        let composed = FnTransformerOnceOps::and_then(parse, double);
        assert_eq!(composed.apply("21".to_string()), 42);
    }

    #[test]
    fn test_closure_compose() {
        let double = |x: i32| x * 2;
        let to_string = |x: i32| x.to_string();

        let composed = to_string.compose(double);
        assert_eq!(composed.apply(21), "42");
    }

    #[test]
    fn test_closure_when() {
        let double = |x: i32| x * 2;
        let conditional = FnTransformerOnceOps::when(double, |x: &i32| *x > 0).or_else(|x: i32| -x);

        assert_eq!(conditional.apply(5), 10);
        let conditional2 =
            FnTransformerOnceOps::when(|x: i32| x * 2, |x: &i32| *x > 0).or_else(|x: i32| -x);
        assert_eq!(conditional2.apply(-5), 5);
    }
}

// ============================================================================
// Composition with BoxTransformerOnce Tests
// ============================================================================

#[cfg(test)]
mod composition_with_box_tests {
    use super::*;

    #[test]
    fn test_closure_and_then_box() {
        let parse = |s: String| s.parse::<i32>().unwrap_or(0);
        let double = BoxTransformerOnce::new(|x: i32| x * 2);

        let composed = FnTransformerOnceOps::and_then(parse, double);
        assert_eq!(composed.apply("21".to_string()), 42);
    }

    #[test]
    fn test_closure_compose_box() {
        let double = BoxTransformerOnce::new(|x: i32| x * 2);
        let to_string = BoxTransformerOnce::new(|x: i32| x.to_string());

        // First apply double, then to_string
        let composed = double.and_then(to_string);
        assert_eq!(composed.apply(21), "42");
    }

    #[test]
    fn test_box_and_then_closure() {
        let parse = BoxTransformerOnce::new(|s: String| s.parse::<i32>().unwrap_or(0));
        let double = BoxTransformerOnce::new(|x: i32| x * 2);

        let composed = parse.and_then(double);
        assert_eq!(composed.apply("21".to_string()), 42);
    }

    #[test]
    fn test_box_compose_closure() {
        let to_string = BoxTransformerOnce::new(|x: i32| x.to_string());
        let double = BoxTransformerOnce::new(|x: i32| x * 2);

        // First apply double, then to_string
        let composed = double.and_then(to_string);
        assert_eq!(composed.apply(21), "42");
    }
}

// ============================================================================
// Multiple Composition Tests
// ============================================================================

#[cfg(test)]
mod multiple_composition_tests {
    use super::*;

    #[test]
    fn test_multiple_closures_and_then() {
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        let to_string = |x: i32| x.to_string();

        let composed = FnTransformerOnceOps::and_then(
            add_one,
            FnTransformerOnceOps::and_then(double, to_string),
        );
        assert_eq!(composed.apply(5), "12");
    }

    #[test]
    fn test_multiple_closures_compose() {
        let add_one = |x: i32| x + 1;
        let double = |x: i32| x * 2;
        let square = |x: i32| x * x;

        // ((5 + 1) * 2)^2 = 144
        // Create a combined closure: |x| square(double(add_one(x)))
        let composed = |x: i32| square(double(add_one(x)));
        assert_eq!(composed(5), 144);
    }

    #[test]
    fn test_mixed_and_then_compose() {
        let parse = |s: String| s.parse::<i32>().unwrap_or(0);
        let add_ten = |x: i32| x + 10;
        let double = |x: i32| x * 2;

        let composed =
            FnTransformerOnceOps::and_then(parse, FnTransformerOnceOps::and_then(add_ten, double));
        assert_eq!(composed.apply("16".to_string()), 52); // (16 + 10) * 2
    }
}

// ============================================================================
// Conditional Mapping Tests
// ============================================================================

#[cfg(test)]
mod conditional_mapping_tests {
    use super::*;
    use prism3_function::{
        Predicate,
        RcPredicate,
    };

    #[test]
    fn test_when_with_closure_predicate() {
        let double = |x: i32| x * 2;
        let is_positive = |x: &i32| *x > 0;

        let conditional = FnTransformerOnceOps::when(double, is_positive).or_else(|x: i32| -x);
        assert_eq!(conditional.apply(5), 10);
    }

    #[test]
    fn test_when_with_rc_predicate() {
        let double = |x: i32| x * 2;
        let is_positive = RcPredicate::new(|x: &i32| *x > 0);

        // Clone to preserve original predicate
        let conditional =
            FnTransformerOnceOps::when(double, is_positive.clone()).or_else(|x: i32| -x);

        assert_eq!(conditional.apply(5), 10);

        // Original predicate still usable
        assert!(is_positive.test(&3));
    }

    #[test]
    fn test_when_chain() {
        let double = |x: i32| x * 2;
        let is_positive = |x: &i32| *x > 0;
        let negate = |x: i32| -x;

        let conditional = FnTransformerOnceOps::when(double, is_positive).or_else(negate);
        let composed = conditional.and_then(|x| x + 1);

        assert_eq!(composed.apply(5), 11); // (5 * 2) + 1
    }
}

// ============================================================================
// Complex Pipeline Tests
// ============================================================================

#[cfg(test)]
mod complex_pipeline_tests {
    use super::*;

    #[test]
    fn test_parse_transform_format_pipeline() {
        let parse = |s: String| s.parse::<i32>().unwrap_or(0);
        let double = |x: i32| x * 2;
        let add_ten = |x: i32| x + 10;
        let to_string = |x: i32| format!("Result: {}", x);

        let pipeline = FnTransformerOnceOps::and_then(
            parse,
            FnTransformerOnceOps::and_then(
                double,
                FnTransformerOnceOps::and_then(add_ten, to_string),
            ),
        );
        assert_eq!(pipeline.apply("16".to_string()), "Result: 42");
    }

    #[test]
    fn test_conditional_pipeline() {
        let parse = |s: String| s.parse::<i32>().unwrap_or(0);
        let double = |x: i32| x * 2;
        let is_even = |x: &i32| x % 2 == 0;
        let identity = |x: i32| x;
        let half = |x: i32| x / 2;

        let temp1 = FnTransformerOnceOps::and_then(parse, double);
        let conditional = FnTransformerOnceOps::when(|y: i32| y, is_even).or_else(identity);
        let temp2 = temp1.and_then(move |x: i32| conditional.apply(x));
        let pipeline = temp2.and_then(half);

        assert_eq!(pipeline.apply("21".to_string()), 21); // (21 * 2) / 2
    }

    #[test]
    fn test_option_pipeline() {
        let parse = |s: String| s.parse::<i32>().ok();
        let double = |opt: Option<i32>| opt.map(|x| x * 2);
        let to_string = |opt: Option<i32>| opt.map(|x| x.to_string());

        let pipeline = FnTransformerOnceOps::and_then(
            parse,
            FnTransformerOnceOps::and_then(double, to_string),
        );
        assert_eq!(pipeline.apply("21".to_string()), Some("42".to_string()));
    }
}

// ============================================================================
// Function Pointer Tests
// ============================================================================

#[cfg(test)]
mod function_pointer_tests {
    use super::*;

    fn double(x: i32) -> i32 {
        x * 2
    }

    fn add_ten(x: i32) -> i32 {
        x + 10
    }

    fn to_string(x: i32) -> String {
        x.to_string()
    }

    #[test]
    fn test_function_pointer_and_then() {
        let composed = FnTransformerOnceOps::and_then(double, add_ten);
        assert_eq!(composed.apply(16), 42); // (16 * 2) + 10
    }

    #[test]
    fn test_function_pointer_compose() {
        let composed = add_ten.compose(double);
        assert_eq!(composed.apply(16), 42); // (16 * 2) + 10
    }

    #[test]
    fn test_function_pointer_chain() {
        let composed = FnTransformerOnceOps::and_then(
            double,
            FnTransformerOnceOps::and_then(add_ten, to_string),
        );
        assert_eq!(composed.apply(16), "42");
    }

    #[test]
    fn test_function_pointer_when() {
        let conditional = FnTransformerOnceOps::when(double, |x: &i32| *x > 0).or_else(|x: i32| -x);
        assert_eq!(conditional.apply(5), 10);
    }
}

// ============================================================================
// Captured State Tests
// ============================================================================

#[cfg(test)]
mod captured_state_tests {
    use super::*;

    #[test]
    fn test_closure_with_captured_value() {
        let multiplier = 3;
        let multiply = move |x: i32| x * multiplier;

        let composed = FnTransformerOnceOps::and_then(multiply, |x| x + 10);
        assert_eq!(composed.apply(10), 40); // (10 * 3) + 10
    }

    #[test]
    fn test_closure_with_moved_value() {
        let prefix = "Value: ".to_string();
        let format_with_prefix = move |x: i32| format!("{}{}", prefix, x);

        let composed = FnTransformerOnceOps::and_then(format_with_prefix, |s| s + "!");
        assert_eq!(composed.apply(42), "Value: 42!");
    }

    #[test]
    fn test_multiple_captured_values() {
        let multiplier = 2;
        let offset = 10;

        let transform = move |x: i32| (x * multiplier) + offset;
        let composed = FnTransformerOnceOps::and_then(transform, |x: i32| x.to_string());

        assert_eq!(composed.apply(16), "42"); // (16 * 2) + 10
    }
}

// ============================================================================
// Type Conversion Tests
// ============================================================================

#[cfg(test)]
mod type_conversion_tests {
    use super::*;

    #[test]
    fn test_i32_to_string() {
        let to_string = |x: i32| x.to_string();
        let add_suffix = |s: String| s + "_suffix";

        let composed = FnTransformerOnceOps::and_then(to_string, add_suffix);
        assert_eq!(composed.apply(42), "42_suffix");
    }

    #[test]
    fn test_string_to_i32() {
        let parse = |s: String| s.parse::<i32>().unwrap_or(0);
        let double = |x: i32| x * 2;

        let composed = FnTransformerOnceOps::and_then(parse, double);
        assert_eq!(composed.apply("21".to_string()), 42);
    }

    #[test]
    fn test_option_chain() {
        let parse = |s: String| s.parse::<i32>().ok();
        let double = |opt: Option<i32>| opt.map(|x| x * 2);
        let unwrap_or = |opt: Option<i32>| opt.unwrap_or(0);

        let composed = FnTransformerOnceOps::and_then(
            parse,
            FnTransformerOnceOps::and_then(double, unwrap_or),
        );
        assert_eq!(composed.apply("21".to_string()), 42);
    }

    #[test]
    fn test_result_chain() {
        let parse = |s: String| -> Result<i32, std::num::ParseIntError> { s.parse::<i32>() };
        let double = |r: Result<i32, _>| r.map(|x| x * 2);
        let unwrap_or = |r: Result<i32, _>| r.unwrap_or(0);

        let composed = FnTransformerOnceOps::and_then(
            parse,
            FnTransformerOnceOps::and_then(double, unwrap_or),
        );
        assert_eq!(composed.apply("21".to_string()), 42);
    }
}

// ============================================================================
// Edge Cases Tests
// ============================================================================

#[cfg(test)]
mod edge_cases_tests {
    use super::*;

    #[test]
    fn test_identity_composition() {
        let identity = |x: i32| x;
        let double = |x: i32| x * 2;

        let composed = FnTransformerOnceOps::and_then(identity, double);
        assert_eq!(composed.apply(21), 42);
    }

    #[test]
    fn test_constant_mapper() {
        let constant = |_x: i32| 42;
        let double = |x: i32| x * 2;

        let composed = FnTransformerOnceOps::and_then(constant, double);
        assert_eq!(composed.apply(0), 84);
    }

    #[test]
    fn test_empty_string() {
        let parse = |s: String| s.parse::<i32>().unwrap_or(0);
        let double = |x: i32| x * 2;

        let composed = FnTransformerOnceOps::and_then(parse, double);
        assert_eq!(composed.apply("".to_string()), 0);
    }

    #[test]
    fn test_zero_value() {
        let double = |x: i32| x * 2;
        let add_ten = |x: i32| x + 10;

        let composed = FnTransformerOnceOps::and_then(double, add_ten);
        assert_eq!(composed.apply(0), 10);
    }

    #[test]
    fn test_negative_values() {
        let negate = |x: i32| -x;
        let double = |x: i32| x * 2;

        let composed = FnTransformerOnceOps::and_then(negate, double);
        assert_eq!(composed.apply(21), -42);
    }
}

// ============================================================================
// Advanced Usage Tests
// ============================================================================

#[cfg(test)]
mod advanced_usage_tests {
    use super::*;

    #[test]
    fn test_vec_transformation() {
        let split = |s: String| s.split(',').map(|s| s.to_string()).collect::<Vec<_>>();
        let count = |v: Vec<String>| v.len();

        let composed = FnTransformerOnceOps::and_then(split, count);
        assert_eq!(composed.apply("a,b,c".to_string()), 3);
    }

    #[test]
    fn test_nested_options() {
        let parse = |s: String| s.parse::<i32>().ok();
        let double_if_positive =
            |opt: Option<i32>| opt.and_then(|x| if x > 0 { Some(x * 2) } else { None });

        let composed = FnTransformerOnceOps::and_then(parse, double_if_positive);
        assert_eq!(composed.apply("21".to_string()), Some(42));
        let composed2 = FnTransformerOnceOps::and_then(
            |s: String| s.parse::<i32>().ok(),
            |opt: Option<i32>| opt.and_then(|x| if x > 0 { Some(x * 2) } else { None }),
        );
        assert_eq!(composed2.apply("-10".to_string()), None);
    }

    #[test]
    fn test_conditional_with_complex_logic() {
        let is_in_range = |x: &i32| *x >= 0 && *x <= 100;
        let scale = |x: i32| x * 2;
        let clamp = |x: i32| if x > 100 { 100 } else { x };

        let composed = FnTransformerOnceOps::when(scale, is_in_range).or_else(clamp);
        assert_eq!(composed.apply(21), 42); // in range, scaled
        let composed2 = FnTransformerOnceOps::when(|x: i32| x * 2, |x: &i32| *x >= 0 && *x <= 100)
            .or_else(|x: i32| if x > 100 { 100 } else { x });
        assert_eq!(composed2.apply(150), 100); // out of range, clamped
    }

    #[test]
    fn test_boxed_value_transformation() {
        let unbox = |b: Box<i32>| *b;
        let double = |x: i32| x * 2;
        let rebox = |x: i32| Box::new(x);

        let composed =
            FnTransformerOnceOps::and_then(unbox, FnTransformerOnceOps::and_then(double, rebox));
        let result = composed.apply(Box::new(21));
        assert_eq!(*result, 42);
    }
}
