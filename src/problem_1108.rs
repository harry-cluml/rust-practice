#[allow(dead_code)]
fn solve(address: &str) -> String {
    address.replace('.', "[.]")
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let address = "1.1.1.1";
        let expected_output = "1[.]1[.]1[.]1";
        let output = solve(&address);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let address = "255.100.50.0";
        let expected_output = "255[.]100[.]50[.]0";
        let output = solve(&address);
        assert_eq!(output, expected_output);
    }
}
