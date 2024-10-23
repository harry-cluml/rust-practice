/**
 * https://leetcode.com/problems/concatenation-of-array/description/

1929. Concatenation of Array
Easy
Topics
Companies
Hint
Given an integer array nums of length n, you want to create an array ans of length 2n where ans[i] == nums[i] and ans[i + n] == nums[i] for 0 <= i < n (0-indexed).

Specifically, ans is the concatenation of two nums arrays.

Return the array ans.



Example 1:

Input: nums = [1,2,1]
Output: [1,2,1,1,2,1]
Explanation: The array ans is formed as follows:
- ans = [nums[0],nums[1],nums[2],nums[0],nums[1],nums[2]]
- ans = [1,2,1,1,2,1]
Example 2:

Input: nums = [1,3,2,1]
Output: [1,3,2,1,1,3,2,1]
Explanation: The array ans is formed as follows:
- ans = [nums[0],nums[1],nums[2],nums[3],nums[0],nums[1],nums[2],nums[3]]
- ans = [1,3,2,1,1,3,2,1]


Constraints:

n == nums.length
1 <= n <= 1000
1 <= nums[i] <= 1000
 */

fn solution(nums: &[i32]) -> Vec<i32> {
    debug_assert!(1 <= nums.len() && nums.len() <= 1000);
    let mut output = nums.to_vec();
    output.extend(nums.into_iter());
    output
}

fn check(input: &[i32], output: &[i32]) {
    assert_eq!(output.len(), input.len() * 2);
    for i in 0..input.len() {
        assert_eq!(output[i], input[i]);
        assert_eq!(output[input.len() + i], input[i]);
    }
}

#[test]
fn problem_1929_concatenation_of_array_example_1() {
    let input = [1, 2, 1];
    let output = solution(input.as_slice());
    check(&input, &output);
}

#[test]
fn problem_1929_concatenation_of_array_example_2() {
    let input = [1, 3, 2, 1];
    let output = solution(input.as_slice());
    check(&input, &output);
}
