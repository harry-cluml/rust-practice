/**
https://leetcode.com/problems/running-sum-of-1d-array/description/

1480. Running Sum of 1d Array
Easy
Topics
Companies
Hint
Given an array nums. We define a running sum of an array as runningSum[i] = sum(nums[0]…nums[i]).

Return the running sum of nums.



Example 1:

Input: nums = [1,2,3,4]
Output: [1,3,6,10]
Explanation: Running sum is obtained as follows: [1, 1+2, 1+2+3, 1+2+3+4].
Example 2:

Input: nums = [1,1,1,1,1]
Output: [1,2,3,4,5]
Explanation: Running sum is obtained as follows: [1, 1+1, 1+1+1, 1+1+1+1, 1+1+1+1+1].
Example 3:

Input: nums = [3,1,2,10,1]
Output: [3,4,6,16,17]


Constraints:

1 <= nums.length <= 1000
-10^6 <= nums[i] <= 10^6
 */

fn solution(input: &[i32]) -> Vec<i32> {
    let mut output = vec![0; input.len()];
    for out_i in 0..output.len() {
        let value = input[0..=out_i].iter().cloned().fold(0, |acc, e| acc + e);
        output[out_i] = value;
    }
    output
}

#[test]
fn problem_1480_running_sum_of_1d_array_example_1() {
    let input = [1, 2, 3, 4];
    let expected_output = [1, 3, 6, 10];
    let output = solution(&input);
    assert_eq!(output, expected_output);
}

#[test]
fn problem_1480_running_sum_of_1d_array_example_2() {
    let input = [1, 1, 1, 1, 1];
    let expected_output = [1, 2, 3, 4, 5];
    let output = solution(&input);
    assert_eq!(output, expected_output);
}

#[test]
fn problem_1480_running_sum_of_1d_array_example_3() {
    let input = [1, 1, 1, 1, 1];
    let expected_output = [1, 2, 3, 4, 5];
    let output = solution(&input);
    assert_eq!(output, expected_output);
}
