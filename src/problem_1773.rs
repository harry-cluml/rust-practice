#[allow(dead_code)]
fn solve(items: &[Vec<&str>], rule_key: &str, rule_value: &str) -> usize {
    items
        .iter()
        .filter_map(|x| {
            let &types = x.first()?;
            let &color = x.get(1)?;
            let &name = x.last()?;
            let matches = match rule_key {
                "type" => types == rule_value,
                "color" => color == rule_value,
                "name" => name == rule_value,
                _ => return None,
            };
            if matches {
                Some(())
            } else {
                None
            }
        })
        .count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let items = vec![
            vec!["phone", "blue", "pixel"],
            vec!["computer", "silver", "lenovo"],
            vec!["phone", "gold", "iphone"],
        ];
        let rule_key = "color";
        let rule_value = "silver";
        let expected_output = 1;
        let output = solve(&items, rule_key, rule_value);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let items = vec![
            vec!["phone", "blue", "pixel"],
            vec!["computer", "silver", "phone"],
            vec!["phone", "gold", "iphone"],
        ];
        let rule_key = "type";
        let rule_value = "phone";
        let expected_output = 2;
        let output = solve(&items, rule_key, rule_value);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_1_original_signature() {
        let items: Vec<Vec<String>> = vec![
            vec!["phone".to_string(), "blue".to_string(), "pixel".to_string()],
            vec![
                "computer".to_string(),
                "silver".to_string(),
                "lenovo".to_string(),
            ],
            vec![
                "phone".to_string(),
                "gold".to_string(),
                "iphone".to_string(),
            ],
        ];
        let rule_key = "color";
        let rule_value = "silver";
        let expected_output = 1;
        let items: Vec<Vec<&str>> = items
            .iter()
            .map(|x| x.iter().map(|y| y.as_str()).collect())
            .collect();
        let output = solve(&items, rule_key, rule_value);
        assert_eq!(output, expected_output);
    }
}
