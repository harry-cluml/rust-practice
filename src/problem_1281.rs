use std::iter::successors;

fn solve(n: i32) -> Result<i32, String> {
    let (prod, sum) = successors(Some(n), |&x| if x > 0 { Some(x / 10) } else { None })
        .filter_map(|x| if x > 0 { Some(x % 10) } else { None })
        .fold((Ok(1), Ok(0)), |(prod, sum), x| {
            (
                prod.and_then(|p: i32| {
                    p.checked_mul(x)
                        .ok_or_else(|| format!("Overflow occurred. prod: {} x: {}", p, x))
                }),
                sum.and_then(|s: i32| {
                    s.checked_add(x)
                        .ok_or_else(|| format!("Overflow occurred. sum: {} x: {}", s, x))
                }),
            )
        });
    match (prod, sum) {
        (Ok(p), Ok(s)) => p
            .checked_sub(s)
            .ok_or_else(|| format!("Overflow occurred. prod: {} sum: {}", p, s)),
        (Ok(_), Err(s)) => Err(s),
        (Err(p), Ok(_)) => Err(p),
        (Err(p), Err(s)) => Err(format!("{}\n{}", p, s)),
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let n = 234;
        let expected_output = 15;
        let output = solve(n);
        assert_eq!(output.expect("Unexpected error occurred"), expected_output);
    }

    #[test]
    fn example_2() {
        let n = 4421;
        let expected_output = 21;
        let output = solve(n);
        assert_eq!(output.expect("Unexpected error occurred"), expected_output);
    }
}
