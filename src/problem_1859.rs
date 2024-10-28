#[allow(dead_code)]
fn solve(s: &str) -> String {
    let mut splited: Vec<&str> = s.split(' ').collect();
    debug_assert!(splited.iter().all(|x| !x.is_empty()));
    splited.sort_unstable_by_key(|x| x.chars().last());
    splited
        .iter()
        .map(|x| &x[..x.len() - 1])
        .collect::<Vec<&str>>()
        .join(" ")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let s = "is2 sentence4 This1 a3";
        let expected_output = "This is a sentence";
        let output = solve(s);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let s = "Myself2 Me1 I4 and3";
        let expected_output = "Me Myself and I";
        let output = solve(&s);
        assert_eq!(output, expected_output);
    }
}
