use std::iter::successors;

#[allow(dead_code)]
fn solve(n: i32) -> i32 {
    let (prod, sum) = successors(Some(n), |&x| if x > 0 { Some(x / 10) } else { None })
        .filter_map(|x| if x > 0 { Some(x % 10) } else { None })
        .fold(
            (Ok::<i32, String>(1), Ok::<i32, String>(0)),
            |(prod, sum), x| {
                let p = prod.expect("n always 1 <= n <= 10^5, prod will not overflow");
                let s = sum.expect("n always 1 <= n <= 10^5, sum will not overflow");
                (Ok(p * x), Ok(s + x))
            },
        );
    let p = prod.expect("n always 1 <= n <= 10^5, prod will not overflow");
    let s = sum.expect("n always 1 <= n <= 10^5, sum will not overflow");
    p - s
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n = 234;
        let expected_output = 15;
        let output = solve(n);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let n = 4421;
        let expected_output = 21;
        let output = solve(n);
        assert_eq!(output, expected_output);
    }
}
