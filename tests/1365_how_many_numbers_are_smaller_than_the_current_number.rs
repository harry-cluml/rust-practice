use anyhow::{bail, Result};

/*
https://leetcode.com/problems/how-many-numbers-are-smaller-than-the-current-number/description/

1365. How Many Numbers Are Smaller Than the Current Number
Easy
Topics
Companies
Hint
Given the array nums, for each nums[i] find out how many numbers in the array are smaller than it. That is, for each nums[i] you have to count the number of valid j's such that j != i and nums[j] < nums[i].

Return the answer in an array.



Example 1:

Input: nums = [8,1,2,2,3]
Output: [4,0,1,1,3]
Explanation:
For nums[0]=8 there exist four smaller numbers than it (1, 2, 2 and 3).
For nums[1]=1 does not exist any smaller number than it.
For nums[2]=2 there exist one smaller number than it (1).
For nums[3]=2 there exist one smaller number than it (1).
For nums[4]=3 there exist three smaller numbers than it (1, 2 and 2).
Example 2:

Input: nums = [6,5,4,8]
Output: [2,1,0,3]
Example 3:

Input: nums = [7,7,7,7]
Output: [0,0,0,0]


Constraints:

2 <= nums.length <= 500
0 <= nums[i] <= 100
*/

fn solution(input: &[i32]) -> Result<Vec<i32>> {
    let mut output = vec![0i32; input.len()];
    let mut sorted_input: Vec<i32> = input.iter().cloned().collect();
    sorted_input.sort();
    for i in 0..input.len() {
        let Some(found) = sorted_input.iter().position(|&x| input[i] == x) else {
            bail!("It must exist in the current scope");
        };
        let casted: i32 = found.try_into()?;
        output[i] = casted;
    }
    Ok(output)
}

#[test]
fn problem_1365_how_many_numbers_are_smaller_than_the_current_number_example_1() {
    let input = [8, 1, 2, 2, 3];
    let expected_output = [4, 0, 1, 1, 3];
    let output = solution(&input);
    assert_eq!(output.expect("Must validated"), expected_output);
}

#[test]
fn problem_1365_how_many_numbers_are_smaller_than_the_current_number_example_2() {
    let input = [6, 5, 4, 8];
    let expected_output = [2, 1, 0, 3];
    let output = solution(&input);
    assert_eq!(output.expect("Must validated"), expected_output);
}

#[test]
fn problem_1365_how_many_numbers_are_smaller_than_the_current_number_example_3() {
    let input = [7, 7, 7, 7];
    let expected_output = [0, 0, 0, 0];
    let output = solution(&input);
    assert_eq!(output.expect("Must validated"), expected_output);
}
