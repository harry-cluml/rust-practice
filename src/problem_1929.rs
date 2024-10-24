fn solve(nums: &[i32]) -> Vec<i32> {
    [nums, nums].concat()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [1, 2, 1];
        let expected_output = [1, 2, 1, 1, 2, 1];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let nums = [1, 3, 2, 1];
        let expected_output = [1, 3, 2, 1, 1, 3, 2, 1];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }
}
