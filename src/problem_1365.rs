#[allow(dead_code)]
fn solve(nums: &[i32]) -> Vec<usize> {
    nums.iter()
        .map(|&x| nums.iter().filter(|&&y| x > y).count())
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [8, 1, 2, 2, 3];
        let expected_output = [4, 0, 1, 1, 3];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let nums = [6, 5, 4, 8];
        let expected_output = [2, 1, 0, 3];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_3() {
        let nums = [7, 7, 7, 7];
        let expected_output = [0, 0, 0, 0];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }
}
