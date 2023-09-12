#[cfg(test)]
mod tests {
    use std::error::Error;
    use test_case::test_case;


    #[test_case(- 2, - 4; "when both operands are negative")]
    #[test_case(2, 4; "when both operands are positive")]
    #[test_case(4, 2; "when operands are swapped")]
    fn multiplication_tests(x: i8, y: i8) {
        let actual = (x * y).abs();

        assert_eq!(8, actual)
    }

    #[test_case(1; "valid")]
    // #[test_case(4; "invalid")]
    fn is_odd(i: i32) {
        assert_eq!(i % 2, 1);
    }

    #[test_case("input"; "string length is 5")]
    fn is_5_chars_long(s: &str) -> Result<(), Box<dyn Error>> {
        if s.len() == 5 {
            Ok(())
        } else {
            Err("Isn't 5 characters long".to_string().into())
        }
    }

    #[test_case(1, 2 => 3; "test_add_1_2")]
    fn test_add(a: u32, b: u32) -> u32 {
        a + b
    }

    #[test_case(9 => ignore 3)]
    #[test_case(4 => inconclusive 2)]
    #[test_case(9 => - 3)]
    #[test_case(4 => - 2)]
    fn sqrt(number: u64) -> i64 {
        -(number as f64).sqrt() as i64
    }

    #[test_case(2 => true)]
    #[test_case(3 => true)]
    #[test_case(0 => ignore true)]
    fn is_natural(number: i32) -> bool {
        number >= 0
    }

    // 匹配对比
    #[test_case(3 => matches Ok(3))]
    #[test_case(4 => matches Ok(_))]
    #[test_case(5 => matches Ok(v) if v == 8)]
    #[test_case(- 1 => matches Err(_))]
    pub fn fibbonacci(seq_idx: i32) -> Result<i32, ()> {
        if seq_idx < 0 {
            Err(())
        } else if seq_idx < 2 {
            Ok(1)
        } else {
            Ok(fibbonacci(seq_idx - 1).unwrap() + fibbonacci(seq_idx - 2).unwrap())
        }
    }

    // 恐慌验证器
    #[test_case(2.0, 0.0 => panics)]
    #[test_case(2.0, - 0.0 => panics "Division by zero")]
    #[test_case(2.0, 1.0 => 2.0)]
    fn div(dividend: f32, divisor: f32) -> f32 {
        if divisor.abs() < f32::EPSILON {
            panic!("Division by zero")
        }
        dividend / divisor
    }



    // 闭包验证器
    #[test_case(2.0 => 0.0)]
    #[test_case(0.0 => with | i: f64 | assert ! (i.is_nan()))]
    fn test_division(i: f64) -> f64 {
        0.0 / i
    }

    // 函数验证器
    fn simple_validate(actual: u64) {
        assert_eq!(actual, 2)
    }

    fn wrapped_pretty_assert(expected: u64) -> impl Fn(u64) {
        move |actual: u64| { pretty_assertions::assert_eq!(actual, expected) }
    }

    #[test_case(2 => using simple_validate)]
    #[test_case(1 => using wrapped_pretty_assert(1))]
    fn pretty_assertions_usage(input: u64) -> u64 {
        input
    }


    // 数字比较
    #[test_case(1.0 => is equal_to 2.0; "eq1")]
    #[test_case(1.0 => is eq 2.0; "eq2")]
    #[test_case(1.0 => is less_than 3.0; "lt1")]
    #[test_case(1.0 => is lt 3.0; "lt2")]
    #[test_case(1.0 => is greater_than 0.0; "gt1")]
    #[test_case(1.0 => is gt 0.0; "gt2")]
    #[test_case(1.0 => is less_or_equal_than 2.0; "leq1")]
    #[test_case(1.0 => is leq 2.0; "leq2")]
    #[test_case(1.0 => is greater_or_equal_than 1.0; "geq1")]
    #[test_case(1.0 => is geq 1.0; "geq2")]
    #[test_case(1.0 => is almost_equal_to 2.1 precision 0.15; "almost_eq1")]
    #[test_case(1.0 => is almost 2.0 precision 0.01; "almost_eq2")]
    fn complex_tests(input: f64) -> f64 {
        input * 2.0
    }

    // 断言文件系统
    #[test_case("Cargo.toml" => is existing_path)]
    // #[test_case("src/lib.rs" => is file)]
    #[test_case("src/" => is dir ; "short_dir")]
    #[test_case("src/" => is directory ; "long_dir")]
    fn create_path(val: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(val)
    }

    // 测试集合
    #[test_case(vec![1, 2, 3, 4] => it contains 1)]
    #[test_case(vec![1, 2, 3, 4] => it contains_in_order [3, 4])]
    fn contains_tests(items: Vec<u64>) -> Vec<u64> {
        items
    }

    // 逻辑运算符
    #[test_case(vec![1, 2, 3] => it (contains 1 or contains 4) and contains 2)]
    #[test_case(vec![1, 2, 3] => it (contains 1 or contains 4) and not contains 7)]
    #[test_case(vec![1, 2, 3] => it contains 1 and contains 2 and contains_in_order [2, 3])]
    #[test_case(vec![1, 2, 3] => it (contains 6 and contains 7) or (contains 1 and contains_in_order [1, 2, 3]))]
    pub fn combinators_with_arrays(a: Vec<u8>) -> Vec<u8> {
        a
    }
}
