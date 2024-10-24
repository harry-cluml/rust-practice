use anyhow::{bail, Result};

/*
https://leetcode.com/problems/sorting-the-sentence/description/

1859. Sorting the Sentence
Easy
Topics
Companies
Hint
A sentence is a list of words that are separated by a single space with no leading or trailing spaces. Each word consists of lowercase and uppercase English letters.

A sentence can be shuffled by appending the 1-indexed word position to each word then rearranging the words in the sentence.

For example, the sentence "This is a sentence" can be shuffled as "sentence4 a3 is2 This1" or "is2 sentence4 This1 a3".
Given a shuffled sentence s containing no more than 9 words, reconstruct and return the original sentence.



Example 1:

Input: s = "is2 sentence4 This1 a3"
Output: "This is a sentence"
Explanation: Sort the words in s to their original positions "This1 is2 a3 sentence4", then remove the numbers.
Example 2:

Input: s = "Myself2 Me1 I4 and3"
Output: "Me Myself and I"
Explanation: Sort the words in s to their original positions "Me1 Myself2 and3 I4", then remove the numbers.


Constraints:

2 <= s.length <= 200
s consists of lowercase and uppercase English letters, spaces, and digits from 1 to 9.
The number of words in s is between 1 and 9.
The words in s are separated by a single space.
s contains no leading or trailing spaces.
 */

fn solution(input: &str) -> Result<String> {
    let words: Vec<&str> = input.split_ascii_whitespace().collect();
    let mut results = vec!["".to_string(); words.len()];
    for word in &words {
        let Some(byte) = word.as_bytes().last() else {
            bail!("Last element must exist in the current scope");
        };
        let order: usize = (byte - b'0').try_into()?;
        let Some(result_index) = order.checked_add_signed(-1) else {
            bail!("order -1 should not cause overflow");
        };
        let Some(result) = results.get_mut(result_index) else {
            bail!("Result index must be accessible");
        };
        *result = word[0..word.len() - 1].to_string();
    }
    Ok(results.join(" "))
}

#[test]
fn problem_1859_sorting_the_sentence_example_1() {
    let input = "is2 sentence4 This1 a3";
    let expected_output = "This is a sentence";
    let output = solution(&input);
    assert_eq!(
        output.expect("It must be a successful output"),
        expected_output
    );
}

#[test]
fn problem_1859_sorting_the_sentence_example_2() {
    let input = "Myself2 Me1 I4 and3";
    let expected_output = "Me Myself and I";
    let output = solution(&input);
    assert_eq!(
        output.expect("It must be a successful output"),
        expected_output
    );
}
