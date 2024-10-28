#[allow(dead_code)]
fn solve(encoded: &[i32], first: i32) -> Option<Vec<i32>> {
    encoded.iter().try_fold(vec![first], |mut acc, x| {
        let last = acc.last()?;
        acc.push(last ^ x);
        Some(acc)
    })
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let encoded = [1, 2, 3];
        let first = 1;
        let expected_output = [1, 0, 2, 1];
        let output = solve(&encoded, first);
        assert_eq!(output.unwrap(), expected_output);
    }

    #[test]
    fn example_2() {
        let encoded = [6, 2, 7, 3];
        let first = 4;
        let expected_output = [4, 2, 0, 7, 4];
        let output = solve(&encoded, first);
        assert_eq!(output.unwrap(), expected_output);
    }
}
