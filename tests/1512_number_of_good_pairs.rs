use std::collections::HashMap;

/**
https://leetcode.com/problems/number-of-good-pairs/description/

1512. Number of Good Pairs
Easy
Topics
Companies
Hint
Given an array of integers nums, return the number of good pairs.

A pair (i, j) is called good if nums[i] == nums[j] and i < j.



Example 1:

Input: nums = [1,2,3,1,1,3]
Output: 4
Explanation: There are 4 good pairs (0,3), (0,4), (3,4), (2,5) 0-indexed.
Example 2:

Input: nums = [1,1,1,1]
Output: 6
Explanation: Each pair in the array are good.
Example 3:

Input: nums = [1,2,3]
Output: 0


Constraints:

1 <= nums.length <= 100
1 <= nums[i] <= 100
 */

fn solution(input: &[i32]) -> i32 {
    let mut count_map = HashMap::<i32, i32>::new();
    for &n in input {
        let value = count_map.entry(n).or_insert_with(|| 0);
        *value += 1;
    }
    let mut output = 0;
    for &value in count_map.values() {
        let mut count = value;
        while count > 1 {
            count -= 1;
            output += count;
        }
    }
    output
}

#[test]
fn problem_1512_number_of_good_pairs_example_1() {
    let input = [1, 2, 3, 1, 1, 3];
    let expected_output = 4;
    let output = solution(&input);
    assert_eq!(output, expected_output);
}

#[test]
fn problem_1512_number_of_good_pairs_example_2() {
    let input = [1, 1, 1, 1];
    let expected_output = 6;
    let output = solution(&input);
    assert_eq!(output, expected_output);
}

#[test]
fn problem_1512_number_of_good_pairs_example_3() {
    let input = [1, 2, 3];
    let expected_output = 0;
    let output = solution(&input);
    assert_eq!(output, expected_output);
}
