#[allow(dead_code)]
fn solve(num: i32) -> i32 {
    let n0 = num / 1000;
    let n1 = (num % 1000) / 100;
    let n2 = (num % 100) / 10;
    let n3 = num % 10;
    let mut ns = [n0, n1, n2, n3];
    ns.sort_unstable();
    ns[0] * 10 + ns[2] + ns[1] * 10 + ns[3]
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let num = 2932;
        let expected_output = 52;
        let output = solve(num);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let num = 4009;
        let expected_output = 13;
        let output = solve(num);
        assert_eq!(output, expected_output);
    }
}
