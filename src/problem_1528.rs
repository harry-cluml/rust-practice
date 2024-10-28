use core::str;

#[allow(dead_code)]
fn solve(s: &str, indices: &[usize]) -> String {
    let mut pairs = s.chars().zip(indices).collect::<Vec<(char, &usize)>>();
    pairs.sort_unstable_by_key(|(_, &index)| index);
    pairs.iter().map(|(ch, _)| ch).collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "codeleet";
        let indices = [4, 5, 6, 7, 0, 2, 1, 3];
        let expected_output = "leetcode";
        let output = solve(&s, &indices);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let s = "abc";
        let indices = [0, 1, 2];
        let expected_output = "abc";
        let output = solve(&s, &indices);
        assert_eq!(output, expected_output);
    }
}
