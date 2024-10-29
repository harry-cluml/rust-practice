use std::iter::successors;

#[allow(dead_code)]
fn solve(n: i32) -> Option<i32> {
    let (_, matches) = successors(Some((n, 0)), |&(teams, matches)| match teams {
        0 | 1 => None,
        x if x % 2 == 0 => Some(((x / 2), matches + x / 2)),
        x => Some(((x / 2 + 1), matches + x / 2)),
    })
    .last()?;
    Some(matches)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n = 7;
        let expected_output = 6;
        let output = solve(n);
        assert_eq!(output.unwrap(), expected_output);
    }

    #[test]
    fn example_2() {
        let n = 14;
        let expected_output = 13;
        let output = solve(n);
        assert_eq!(output.unwrap(), expected_output);
    }
}
