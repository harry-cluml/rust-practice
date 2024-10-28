#[allow(dead_code)]
fn solve(nums: &[i32]) -> Vec<i32> {
    nums.iter()
        .scan(0, |state, &x| {
            *state += x;
            Some(*state)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [1, 2, 3, 4];
        let expected_output = [1, 3, 6, 10];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let nums = [1, 1, 1, 1, 1];
        let expected_output = [1, 2, 3, 4, 5];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_3() {
        let nums = [3, 1, 2, 10, 1];
        let expected_output = [3, 4, 6, 16, 17];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }
}
