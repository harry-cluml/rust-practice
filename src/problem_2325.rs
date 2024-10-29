use std::collections::{HashMap, HashSet};

#[allow(dead_code)]
fn solve(key: &str, message: &str) -> String {
    let mut duplicate_key_set = HashSet::<char>::new();
    let mut key_map = HashMap::<char, char>::new();
    key.chars()
        .filter(|&x| {
            if x != ' ' && !duplicate_key_set.contains(&x) {
                duplicate_key_set.insert(x);
                true
            } else {
                false
            }
        })
        .zip('a'..='z')
        .for_each(|(key, value)| {
            key_map.insert(key, value);
        });
    message
        .chars()
        .filter_map(|x| {
            if x == ' ' {
                return Some(&' ');
            }
            key_map.get(&x)
        })
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let key = "the quick brown fox jumps over the lazy dog";
        let message = "vkbs bs t suepuv";
        let expected_output = "this is a secret";
        let output = solve(&key, &message);
        assert_eq!(output, expected_output);
    }
}
