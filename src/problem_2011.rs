#[allow(dead_code)]
fn solve(operations: &[&str]) -> i32 {
    operations.iter().fold(0, |acc, x| {
        let op = match x {
            x if x.starts_with('+') => 1,
            x if x.ends_with('+') => 1,
            x if x.starts_with('-') => -1,
            x if x.ends_with('-') => -1,
            _ => 0,
        };
        acc + op
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let operations = ["--X", "X++", "X++"];
        let expected_output = 1;
        let output = solve(&operations);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let operations = ["++X", "++X", "X++"];
        let expected_output = 3;
        let output = solve(&operations);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_3() {
        let operations = ["X++", "++X", "--X", "X--"];
        let expected_output = 0;
        let output = solve(&operations);
        assert_eq!(output, expected_output);
    }
}
