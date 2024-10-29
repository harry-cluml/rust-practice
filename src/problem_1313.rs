#[allow(dead_code)]
fn solve(nums: &[i32]) -> Vec<i32> {
    nums.chunks(2)
        .filter_map(|x| {
            let freq = x.first()?;
            let val = x.last()?;
            let freq_usize: usize = (*freq).try_into().ok()?;
            Some(vec![*val; freq_usize])
        })
        .flatten()
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [1, 2, 3, 4];
        let expected_output = [2, 4, 4, 4];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let nums = [1, 1, 2, 3];
        let expected_output = [1, 3, 3];
        let output = solve(&nums);
        assert_eq!(output, expected_output);
    }
}
