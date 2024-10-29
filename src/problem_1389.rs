#[allow(dead_code)]
fn solve(nums: &[i32], index: &[usize]) -> Vec<i32> {
    nums.iter()
        .zip(index)
        .fold(Vec::<i32>::new(), |mut acc, (&num, &index)| {
            acc.insert(index, num);
            acc
        })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [0, 1, 2, 3, 4];
        let index = [0, 1, 2, 2, 1];
        let expected_output = [0, 4, 1, 3, 2];
        let output = solve(&nums, &index);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let nums = [1, 2, 3, 4, 0];
        let index = [0, 1, 2, 3, 0];
        let expected_output = [0, 1, 2, 3, 4];
        let output = solve(&nums, &index);
        assert_eq!(output, expected_output);
    }
}
