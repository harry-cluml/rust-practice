/**
 * https://leetcode.com/problems/add-two-integers/description/

Given two integers num1 and num2, return the sum of the two integers.


Example 1:

Input: num1 = 12, num2 = 5
Output: 17
Explanation: num1 is 12, num2 is 5, and their sum is 12 + 5 = 17, so 17 is returned.
Example 2:

Input: num1 = -10, num2 = 4
Output: -6
Explanation: num1 + num2 = -6, so -6 is returned.


Constraints:

-100 <= num1, num2 <= 100
 */

fn solution(num1: i32, num2: i32) -> i32 {
    debug_assert!(-100 <= num1 && num1 <= 100);
    debug_assert!(-100 <= num2 && num2 <= 100);
    num1 + num2
}

#[test]
fn problem_2235_add_two_integers_example_1() {
    assert_eq!(solution(12, 5), 17);
}

#[test]
fn problem_2235_add_two_integers_example_2() {
    assert_eq!(solution(-10, 4), -6);
}
