#[cfg(test)]
mod test_cases {
    use test_case::test_case;

    #[test_case(2)]
    #[test_case(4)]
    fn multiple_test_cases(x: u32) {
        assert!(x < 10)
    }

    #[test_case(1)]
    fn basic_test(x: u32) {
        assert_eq!(x, 1)
    }

    #[test_case("foo")]
    fn impl_trait(x: impl AsRef<str>) {
        assert_eq!("foo", x.as_ref());
    }

    #[test_case(2 => 4)]
    #[test_case(4 => 8)]
    fn result(x: u32) -> u32 {
        x * 2
    }

    #[test_case(1, 8 ; "test 1 + 8 = 9")]
    #[test_case(2, 7 ; "2nd test")]
    #[test_case(3, 6 ; "test_3_+6_=_9")]
    #[test_case(4, 5)]
    fn name(x: u32, y: u32) {
        assert_eq!(9, x + y)
    }

    #[test_case(1, 2 => 3 ; "test no. 1")]
    #[test_case(4, 5 => 9)]
    fn result_and_name(x: u32, y: u32) -> u32 {
        x + y
    }

    #[test_case(true)]
    fn keyword_test(x: bool) {
        assert!(x)
    }

    #[test_case(2 + 4, "6".to_string())]
    fn arg_expressions(x: u32, expected: String) {
        assert_eq!(expected, x.to_string())
    }

    #[test_case(2, 2 => 2 + 2)]
    fn result_expression(x: u32, y: u32) -> u32 {
        x + y
    }

    #[test_case(2, 2 => 2 + 3)]
    #[should_panic(expected = "assertion failed: `(left == right)`")]
    fn result_which_panics(x: u32, y: u32) -> u32 {
        x + y
    }

    #[test_case(2, 2 => 2 + 2 ; "test result expression")]
    fn result_expresion_with_name(x: u32, y: u32) -> u32 {
        x + y
    }

    fn foo() -> u32 {
        42
    }

    #[test_case("dummy")]
    fn leading_underscore_in_test_name(x: &str) {
        assert_eq!("dummy", x)
    }

    #[test_case("DUMMY_CODE")]
    fn lowercase_test_name(x: &str) {
        assert_eq!("DUMMY_CODE", x)
    }

    mod nested {
        use super::*;
        use test_case::test_case;

        #[test_case(1, 1)]
        fn nested_test_case(x: u32, y: u32) {
            assert_eq!(x, y)
        }

        #[test_case(20 + 22)]
        #[test_case(42)]
        fn using_fn_from_super(x: u32) {
            assert_eq!(foo(), x)
        }
    }

    #[test_case(42 => std::string::String::new())]
    fn result_with_mod_sep(_: i8) -> String {
        "".to_string()
    }

    // tests from documentation

    #[test_case( 2 =>  2 ; "returns given number for positive input")]
    #[test_case(-2 =>  2 ; "returns opposite number for non-positive input")]
    #[test_case( 0 =>  0 ; "returns 0 for 0")]
    fn abs_tests(x: i8) -> i8 {
        if x > 0 {
            x
        } else {
            -x
        }
    }

    #[test_case(None,    None    => 0 ; "treats none as 0")]
    #[test_case(Some(2), Some(3) => 5)]
    #[test_case(Some(2 + 3), Some(4) => 2 + 3 + 4)]
    fn fancy_addition(x: Option<i8>, y: Option<i8>) -> i8 {
        x.unwrap_or(0) + y.unwrap_or(0)
    }

    #[test_case( 2,  4 ; "when both operands are possitive")]
    #[test_case( 4,  2 ; "when operands are swapped")]
    #[test_case(-2, -4 ; "when both operands are negative")]
    fn multiplication_tests(x: i8, y: i8) {
        let actual = x * y;

        assert_eq!(8, actual);
    }

    const MY_CONST: &str = "my const";

    #[test_case(MY_CONST ; "this is desc, not an argument")]
    fn const_in_arg(_s: &str) {}

    #[test_case(""     => String::default())]
    fn bar(_: &str) -> String {
        String::default()
    }

    #[derive(Debug)]
    #[allow(dead_code)]
    enum SimpleEnum {
        Var1,
        Var2,
    }

    #[test_case(SimpleEnum::Var2 => matches SimpleEnum::Var2)]
    fn pattern_matching_result(e: SimpleEnum) -> SimpleEnum {
        e
    }

    #[should_panic(expected = "Expected SimpleEnum :: Var2 found Var1")]
    #[test_case(SimpleEnum::Var1 => matches SimpleEnum::Var2)]
    fn pattern_matching_result_fails(e: SimpleEnum) -> SimpleEnum {
        e
    }

    #[test_case(() => panics "It has to panic")]
    #[test_case(() => panics "This should fail")]
    fn panicing(_: ()) {
        panic!("It has to panic")
    }

    #[test_case(() => panics)]
    fn panics_without_value(_: ()) {
        panic!("Message doesn't matter")
    }

    #[test_case(() => inconclusive ())]
    #[test_case(() => inconclusive (); "test is not ran")]
    #[test_case(() => inconclusive (); "inconclusive test")]
    #[test_case(() => ignore (); "ignore keyword")]
    fn inconclusives(_: ()) {
        unreachable!()
    }

    #[test_case::test_case(1; "first test")]
    #[test_case::test_case(1; "second test")]
    fn qualified_attribute(_: u8) {}

    #[test_case(1.0 => with |v: f64| assert!(v.is_infinite()))]
    #[test_case(0.0 => with |v: f64| assert!(v.is_nan()))]
    fn divide_by_zero_f64_with_lambda(input: f64) -> f64 {
        input / 0.0f64
    }

    pub fn assert_is_power_of_two(input: u64) {
        assert!(input.is_power_of_two())
    }

    mod some_mod {
        pub use super::assert_is_power_of_two;
    }

    #[test_case(1 => using assert_is_power_of_two)]
    #[test_case(2 => using crate::test_cases::assert_is_power_of_two)]
    #[test_case(4 => using some_mod::assert_is_power_of_two)]
    fn power_of_two_with_using(input: u64) -> u64 {
        input
    }

    fn wrapped_pretty_assert(expected: u64) -> impl Fn(u64) {
        move |actual: u64| { pretty_assertions::assert_eq!(actual, expected) }
    }

    #[test_case(1 => using wrapped_pretty_assert(1))]
    fn pretty_assertions_usage(input: u64) -> u64 {
        input
    }

    struct Target { i: i64 }

    struct Source1;
    struct Source2;

    impl From<Source1> for Target {
        fn from(_: Source1) -> Self {
            Self { i: 1 }
        }
    }

    impl From<Source2> for Target {
        fn from(_: Source2) -> Self {
            Self { i: 2 }
        }
    }

    #[test_case(Source1 => 1)]
    #[test_case(Source2 => 2)]
    fn test_generics<T: Into<Target>>(input: T) -> i64 {
        let t: Target = input.into();
        t.i
    }

    #[test_case(Source1 => 1)]
    #[test_case(Source2 => 2)]
    fn test_impl(input: impl Into<Target>) -> i64 {
        let t: Target = input.into();
        t.i
    }

    #[test_case(1.0 => is equal_to 2.0 ; "eq1")]
    #[test_case(1.0 => is eq 2.0 ; "eq2")]
    #[test_case(1.0 => is less_than 3.0 ; "lt1")]
    #[test_case(1.0 => is lt 3.0 ; "lt2")]
    #[test_case(1.0 => is greater_than 0.0 ; "gt1")]
    #[test_case(1.0 => is gt 0.0 ; "gt2")]
    #[test_case(1.0 => is less_or_equal_than 2.0 ; "leq1")]
    #[test_case(1.0 => is leq 2.0 ; "leq2")]
    #[test_case(1.0 => is greater_or_equal_than 1.0 ; "geq1")]
    #[test_case(1.0 => is geq 1.0 ; "geq2")]
    #[test_case(1.0 => is almost_equal_to 2.1 precision 0.15 ; "almost_eq1")]
    #[test_case(1.0 => is almost 2.0 precision 0.01 ; "almost_eq2")]
    fn complex_tests(input: f64) -> f64 {
        input * 2.0
    }

    #[test_case("Cargo.toml" => is existing_path)]
    #[test_case("src/lib.rs" => is file)]
    #[test_case("src/" => is dir ; "short_dir")]
    #[test_case("src/" => is directory ; "long_dir")]
    fn create_path(val: &str) -> std::path::PathBuf {
        std::path::PathBuf::from(val)
    }

    #[test_case(vec![1, 2, 3, 4] => it contains 1)]
    #[test_case(vec![1, 2, 3, 4] => it contains_in_order [3, 4])]
    fn contains_tests(items: Vec<u64>) -> Vec<u64> {
        items
    }

    #[test_case(1.0 => is not eq 2.5)]
    #[test_case(1.0 => is not almost 2.1 precision 0.01)]
    fn not_complex(input: f32) -> f32 { input * 1.0 }

    #[test_case("Cargo.yaml".parse().unwrap() => is not existing_path)]
    #[test_case("Cargo.toml".parse().unwrap() => is not dir)]
    #[test_case("src/".parse().unwrap() => is not file)]
    fn not_path(path: std::path::PathBuf) -> String {
        path.to_string_lossy().to_string()
    }

    #[test_case(vec![1, 2, 3, 4] => it not contains 5)]
    #[test_case(vec![1, 2, 3, 4] => it not contains_in_order [3, 2])]
    fn not_contains_tests(items: Vec<u64>) -> Vec<u64> {
        items
    }

    #[test_case(2.0 => it (eq 2.0))]
    fn in_parens(_: f32) -> f32 {
        2.0
    }

    #[test_case(1.0 => is gt 0.0 and lt 5.0)]
    #[test_case(1.0 => is gt 0.0 or lt 0.0)]
    #[test_case(-2.0 => is gt 0.0 or lt 0.0)]
    #[test_case(-2.0 => is (gt 0.0 or lt 0.0) and lt -1.0)]
    #[test_case(1.0 => is (gt 0.0 or lt -1.5) and lt 2.0)]
    #[test_case(0.3 => is (gt 0.0 and lt 1.0) or gt 1.2)]
    #[test_case(0.7 => is (gt 0.0 and lt 1.0) or gt 1.2)]
    fn combinators(v: f32) -> f32 {
        v * 2.0
    }

    #[test_case(vec![1, 2, 3] => it contains 1 and contains 2 and contains_in_order [2, 3])]
    #[test_case(vec![1, 2, 3] => it contains 1 or contains 4)]
    #[test_case(vec![1, 2, 3] => it (contains 1 or contains 4) and contains 2)]
    #[test_case(vec![1, 2, 3] => it (contains 1 and contains 3) or contains 5)]
    #[test_case(vec![1, 2, 3] => it (contains 6 and contains 7) or contains 1)]
    #[test_case(vec![1, 2, 3] => it (contains 6 and contains 7) or (contains 1 and contains_in_order [1, 2, 3]))]
    fn combinators_with_arrays(a: Vec<u8>) -> Vec<u8> {
        a
    }
}
