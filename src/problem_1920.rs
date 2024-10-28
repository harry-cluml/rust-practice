fn solve(nums: &[usize]) -> Result<Vec<usize>, String> {
    let outs = nums
        .iter()
        .filter_map(|&x| nums.get(x))
        .copied()
        .collect::<Vec<usize>>();
    if nums.len() != outs.len() {
        return Err(format!(
            "The lengths of nums and outs must be the same. nums length: {} outs length: {}",
            nums.len(),
            outs.len()
        ));
    }
    Ok(outs)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [0, 2, 1, 5, 3, 4];
        let expected_output = [0, 1, 2, 4, 5, 3];
        let output = solve(&nums);
        assert_eq!(output.expect(""), expected_output);
    }

    #[test]
    fn example_2() {
        let nums = [5, 0, 1, 2, 3, 4];
        let expected_output = [4, 5, 0, 1, 2, 3];
        let output = solve(&nums);
        assert_eq!(output.expect(""), expected_output);
    }
}
