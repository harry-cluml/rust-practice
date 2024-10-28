fn solve(accounts: &Vec<Vec<i32>>) -> Result<i32, String> {
    accounts
        .iter()
        .map(|v| v.iter().sum())
        .max()
        .ok_or_else(|| "Max value must exist".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let accounts: Vec<Vec<i32>> = vec![vec![1, 2, 3], vec![3, 2, 1]];
        let expected_output = 6;
        let output = solve(&accounts);
        assert_eq!(output.expect("unexpected error occurred"), expected_output);
    }

    #[test]
    fn example_2() {
        let accounts: Vec<Vec<i32>> = vec![vec![1, 5], vec![7, 3], vec![3, 5]];
        let expected_output = 10;
        let output = solve(&accounts);
        assert_eq!(output.expect("unexpected error occurred"), expected_output);
    }

    #[test]
    fn example_3() {
        let accounts: Vec<Vec<i32>> = vec![vec![2, 8, 7], vec![7, 1, 3], vec![1, 9, 5]];
        let expected_output = 17;
        let output = solve(&accounts);
        assert_eq!(output.expect("unexpected error occurred"), expected_output);
    }
}
