/*
https://leetcode.com/problems/subtract-the-product-and-sum-of-digits-of-an-integer/description/

1281. Subtract the Product and Sum of Digits of an Integer
Easy
Topics
Companies
Hint
Given an integer number n, return the difference between the product of its digits and the sum of its digits.


Example 1:

Input: n = 234
Output: 15
Explanation:
Product of digits = 2 * 3 * 4 = 24
Sum of digits = 2 + 3 + 4 = 9
Result = 24 - 9 = 15
Example 2:

Input: n = 4421
Output: 21
Explanation:
Product of digits = 4 * 4 * 2 * 1 = 32
Sum of digits = 4 + 4 + 2 + 1 = 11
Result = 32 - 11 = 21


Constraints:

1 <= n <= 10^5
*/

fn solution(input: i32) -> i32 {
    let mut product = 1;
    let mut sum = 0;
    let mut it = input;
    while it != 0 {
        let num = it % 10;
        it /= 10;
        product *= num;
        sum += num;
    }
    product - sum
}

#[test]
fn problem_1281_subtract_the_product_and_sum_of_digits_of_an_integer_example_1() {
    let input = 234;
    let expected_output = 15;
    let output = solution(input);
    assert_eq!(output, expected_output);
}

#[test]
fn problem_1281_subtract_the_product_and_sum_of_digits_of_an_integer_example_2() {
    let input = 4421;
    let expected_output = 21;
    let output = solution(input);
    assert_eq!(output, expected_output);
}
