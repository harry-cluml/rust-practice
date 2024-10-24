fn solve(nums: &[i32]) -> Result<Vec<i32>, String> {
    let mut ans = vec![0; nums.len()];
    for i in 0..ans.len() {
        let Some(&num) = &nums.get(i) else {
            return Err(format!(
                "Index is out of range. nums length: {} index: {}",
                nums.len(),
                i
            ));
        };
        let index: usize = num
            .try_into()
            .or_else(|e| Err(format!("Num to index failed. error: {}", e)))?;
        let Some(&num) = nums.get(index) else {
            return Err(format!(
                "Index is out of range. nums length: {} index: {}",
                nums.len(),
                index
            ));
        };
        let Some(an) = ans.get_mut(i) else {
            return Err(format!(
                "Index is out of range. ans length: {} index: {}",
                ans.len(),
                i
            ));
        };
        *an = num;
    }
    Ok(ans)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let nums = [0, 2, 1, 5, 3, 4];
        let expected_output = [0, 1, 2, 4, 5, 3];
        let output = solve(&nums);
        assert_eq!(
            output.expect("Unexpected error has occurred"),
            expected_output
        );
    }

    #[test]
    fn example_2() {
        let nums = [5, 0, 1, 2, 3, 4];
        let expected_output = [4, 5, 0, 1, 2, 3];
        let output = solve(&nums);
        assert_eq!(
            output.expect("Unexpected error has occurred"),
            expected_output
        );
    }
}
