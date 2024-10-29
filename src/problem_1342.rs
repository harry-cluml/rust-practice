use std::iter::successors;

#[allow(dead_code)]
fn solve(num: i32) -> usize {
    let step = successors(Some(num), |&x| match x {
        0 => None,
        x if x % 2 == 0 => Some(x / 2),
        _ => Some(x - 1),
    })
    .count();
    step - 1
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let num = 14;
        let expected_output = 6;
        let output = solve(num);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let num = 8;
        let expected_output = 4;
        let output = solve(num);
        assert_eq!(output, expected_output);
    }
}
