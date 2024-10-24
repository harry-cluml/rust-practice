/*
https://leetcode.com/problems/jewels-and-stones/description/

771. Jewels and Stones
Easy
Topics
Companies
Hint
You're given strings jewels representing the types of stones that are jewels, and stones representing the stones you have. Each character in stones is a type of stone you have. You want to know how many of the stones you have are also jewels.

Letters are case sensitive, so "a" is considered a different type of stone from "A".



Example 1:

Input: jewels = "aA", stones = "aAAbbbb"
Output: 3
Example 2:

Input: jewels = "z", stones = "ZZ"
Output: 0


Constraints:

1 <= jewels.length, stones.length <= 50
jewels and stones consist of only English letters.
All the characters of jewels are unique.
*/

fn solution(jewels: &str, stones: &str) -> i32 {
    let mut sorted_jewels: Vec<char> = jewels.chars().collect();
    sorted_jewels.sort();
    let mut sorted_stones: Vec<char> = stones.chars().collect();
    sorted_stones.sort();
    let mut output = 0;
    let mut stone_i = 0;
    for jewel in sorted_jewels {
        loop {
            loop {
                if stone_i >= sorted_stones.len() {
                    break;
                }

                if sorted_stones[stone_i] < jewel {
                    stone_i += 1;
                } else {
                    break;
                }
            }

            if stone_i >= sorted_stones.len() {
                break;
            }

            if sorted_stones[stone_i] == jewel {
                output += 1;
                stone_i += 1;
            } else {
                break;
            }
        }
    }
    output
}

#[test]
fn problem_771_jewels_and_stones_example_1() {
    let jewels = "aA";
    let stones = "aAAbbbb";
    let expected_output = 3;
    let output = solution(&jewels, &stones);
    assert_eq!(output, expected_output);
}

#[test]
fn problem_771_jewels_and_stones_example_2() {
    let jewels = "z";
    let stones = "ZZ";
    let expected_output = 0;
    let output = solution(&jewels, &stones);
    assert_eq!(output, expected_output);
}
