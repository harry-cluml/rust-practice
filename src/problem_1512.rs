use std::collections::HashMap;

fn solve(nums: &[i32]) -> i32 {
    let mut count_map = HashMap::<i32, i32>::new();
    let mut acc = 0;
    for &n in nums {
        let value = count_map.entry(n).or_insert_with(|| 0);
        *value += 1;
        if *value > 1 {
            acc += *value - 1;
        }
    }
    acc
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [1, 2, 3, 1, 1, 3];
        let expected_output = 4;
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let nums = [1, 1, 1, 1];
        let expected_output = 6;
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_3() {
        let nums = [1, 2, 3];
        let expected_output = 0;
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }
}
