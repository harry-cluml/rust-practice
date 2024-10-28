#[allow(dead_code)]
fn solve(num1: i32, num2: i32) -> i32 {
    num1 + num2
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let num1 = 12;
        let num2 = 5;
        let expected_output = 17;
        let output = solve(num1, num2);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let num1 = -10;
        let num2 = 4;
        let expected_output = -6;
        let output = solve(num1, num2);
        assert_eq!(output, expected_output);
    }
}
