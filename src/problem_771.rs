use std::collections::HashSet;

#[allow(dead_code)]
fn solve(jewels: &str, stones: &str) -> i32 {
    let jewel_set: HashSet<char> = jewels.chars().collect();
    stones.chars().fold(
        0,
        |acc, x| {
            if jewel_set.contains(&x) {
                acc + 1
            } else {
                acc
            }
        },
    )
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let jewels = "aA";
        let stones = "aAAbbbb";
        let expected_output = 3;
        let output = solve(&jewels, &stones);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let jewels = "z";
        let stones = "ZZ";
        let expected_output = 0;
        let output = solve(&jewels, &stones);
        assert_eq!(output, expected_output);
    }
}
