#[allow(dead_code)]
fn solve(sentences: &[&str]) -> Result<usize, String> {
    sentences
        .iter()
        .map(|&x| x.split(' ').count())
        .max()
        .ok_or_else(|| "Must exist max value".to_string())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let sentences = [
            "alice and bob love leetcode",
            "i think so too",
            "this is great thanks very much",
        ];
        let expected_output = 6;
        let output = solve(&sentences);
        assert_eq!(output.unwrap(), expected_output);
    }

    #[test]
    fn example_2() {
        let sentences = ["please wait", "continue to fight", "continue to win"];
        let expected_output = 3;
        let output = solve(&sentences);
        assert_eq!(output.unwrap(), expected_output);
    }
}
