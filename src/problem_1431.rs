#[allow(dead_code)]
fn solve(candies: &[i32], extra_candies: i32) -> Result<Vec<bool>, String> {
    let max = candies
        .iter()
        .max()
        .ok_or_else(|| "Max value must exist in candies".to_string())?;
    let output = candies
        .iter()
        .map(|&e| e + extra_candies >= *max)
        .collect::<Vec<bool>>();
    Ok(output)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let candies = [2, 3, 5, 1, 3];
        let extra_candies = 3;
        let expected_output = [true, true, true, false, true];
        let output = solve(&candies, extra_candies);
        assert_eq!(output.expect("Must solved"), expected_output);
    }

    #[test]
    fn example_2() {
        let candies = [4, 2, 1, 1, 2];
        let extra_candies = 1;
        let expected_output = [true, false, false, false, false];
        let output = solve(&candies, extra_candies);
        assert_eq!(output.expect("Must solved"), expected_output);
    }

    #[test]
    fn example_3() {
        let candies = [12, 1, 12];
        let extra_candies = 10;
        let expected_output = [true, false, true];
        let output = solve(&candies, extra_candies);
        assert_eq!(output.expect("Must solved"), expected_output);
    }
}
