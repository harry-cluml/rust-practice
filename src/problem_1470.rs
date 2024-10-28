fn solve(nums: &[i32], n: i32) -> Result<Vec<i32>, String> {
    let n: usize = n
        .try_into()
        .or_else(|e| Err(format!("n must be cast to usize. n: {} error: {}", n, e)))?;
    if n >= nums.len() {
        return Err(format!(
            "n must be less than nums length. nums length: {} n: {}",
            nums.len(),
            n
        ));
    }
    let xs = &nums[0..n];
    let ys = &nums[n..nums.len()];
    let output = xs
        .iter()
        .zip(ys.iter())
        .flat_map(|(&x, &y)| [x, y])
        .collect::<Vec<i32>>();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [2, 5, 1, 3, 4, 7];
        let n = 3;
        let expected_output = [2, 3, 5, 4, 1, 7];
        let output = solve(&nums, n);
        assert_eq!(output.expect("Unexpected error occurred"), expected_output);
    }

    #[test]
    fn example_2() {
        let nums = [1, 2, 3, 4, 4, 3, 2, 1];
        let n = 4;
        let expected_output = [1, 4, 2, 3, 3, 2, 4, 1];
        let output = solve(&nums, n);
        assert_eq!(output.expect("Unexpected error occurred"), expected_output);
    }

    #[test]
    fn example_3() {
        let nums = [1, 1, 2, 2];
        let n = 2;
        let expected_output = [1, 2, 1, 2];
        let output = solve(&nums, n);
        assert_eq!(output.expect("Unexpected error occurred"), expected_output);
    }
}
