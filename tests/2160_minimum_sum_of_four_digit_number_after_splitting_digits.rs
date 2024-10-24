/**
https://leetcode.com/problems/minimum-sum-of-four-digit-number-after-splitting-digits/description/

2160. Minimum Sum of Four Digit Number After Splitting Digits
Easy
Topics
Companies
Hint
You are given a positive integer num consisting of exactly four digits. Split num into two new integers new1 and new2 by using the digits found in num. Leading zeros are allowed in new1 and new2, and all the digits found in num must be used.

For example, given num = 2932, you have the following digits: two 2's, one 9 and one 3. Some of the possible pairs [new1, new2] are [22, 93], [23, 92], [223, 9] and [2, 329].
Return the minimum possible sum of new1 and new2.



Example 1:

Input: num = 2932
Output: 52
Explanation: Some possible pairs [new1, new2] are [29, 23], [223, 9], etc.
The minimum sum can be obtained by the pair [29, 23]: 29 + 23 = 52.
Example 2:

Input: num = 4009
Output: 13
Explanation: Some possible pairs [new1, new2] are [0, 49], [490, 0], etc.
The minimum sum can be obtained by the pair [4, 9]: 4 + 9 = 13.


Constraints:

1000 <= num <= 9999
 */

fn solution(input: i32) -> i32 {
    let n1 = input / 1000;
    let n2 = input % 1000 / 100;
    let n3 = input % 100 / 10;
    let n4 = input % 10;
    let mut ns = [n1, n2, n3, n4];
    ns.sort();
    ns[0] * 10 + ns[2] + ns[1] * 10 + ns[3]
}

#[test]
fn problem_2160_minimum_sum_of_four_digit_number_after_splitting_digits_example_1() {
    let input = 2932;
    let expected_output = 52;
    let output = solution(input);
    assert_eq!(output, expected_output);
}

#[test]
fn problem_2160_minimum_sum_of_four_digit_number_after_splitting_digits_example_2() {
    let input = 4009;
    let expected_output = 13;
    let output = solution(input);
    assert_eq!(output, expected_output);
}
