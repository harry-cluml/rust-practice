use anyhow::Result;

/*
https://leetcode.com/problems/maximum-number-of-words-found-in-sentences/description/

2114. Maximum Number of Words Found in Sentences
Easy
Topics
Companies
Hint
A sentence is a list of words that are separated by a single space with no leading or trailing spaces.

You are given an array of strings sentences, where each sentences[i] represents a single sentence.

Return the maximum number of words that appear in a single sentence.



Example 1:

Input: sentences = ["alice and bob love leetcode", "i think so too", "this is great thanks very much"]
Output: 6
Explanation:
- The first sentence, "alice and bob love leetcode", has 5 words in total.
- The second sentence, "i think so too", has 4 words in total.
- The third sentence, "this is great thanks very much", has 6 words in total.
Thus, the maximum number of words in a single sentence comes from the third sentence, which has 6 words.
Example 2:

Input: sentences = ["please wait", "continue to fight", "continue to win"]
Output: 3
Explanation: It is possible that multiple sentences contain the same number of words.
In this example, the second and third sentences (underlined) have the same number of words.


Constraints:

1 <= sentences.length <= 100
1 <= sentences[i].length <= 100
sentences[i] consists only of lowercase English letters and ' ' only.
sentences[i] does not have leading or trailing spaces.
All the words in sentences[i] are separated by a single space.
*/

fn solution(input: &[&str]) -> Result<i32> {
    let mut max = 0i32;
    for &setence in input {
        let count: i32 = setence.split(' ').count().try_into()?;
        if count > max {
            max = count
        }
    }
    Ok(max)
}

#[test]
fn problem_2114_maximum_number_of_words_found_in_sentences_example_1() {
    let input = [
        "alice and bob love leetcode",
        "i think so too",
        "this is great thanks very much",
    ];
    let expected_output = 6;
    let output = solution(&input);
    assert_eq!(
        output.expect("It must exist in the current scope."),
        expected_output
    );
}

#[test]
fn problem_2114_maximum_number_of_words_found_in_sentences_example_2() {
    let input = ["please wait", "continue to fight", "continue to win"];
    let expected_output = 3;
    let output = solution(&input);
    assert_eq!(
        output.expect("It must exist in the current scope."),
        expected_output
    );
}
