/**
https://leetcode.com/problems/defanging-an-ip-address/description/

1108. Defanging an IP Address
Easy
Topics
Companies
Given a valid (IPv4) IP address, return a defanged version of that IP address.

A defanged IP address replaces every period "." with "[.]".



Example 1:

Input: address = "1.1.1.1"
Output: "1[.]1[.]1[.]1"
Example 2:

Input: address = "255.100.50.0"
Output: "255[.]100[.]50[.]0"


Constraints:

The given address is a valid IPv4 address.
 */

fn solution(input: &str) -> String {
    let mut output = "".to_string();
    for ch in input.chars() {
        if ch == '.' {
            output.push_str("[.]");
        } else {
            output.push(ch);
        }
    }
    output
}

#[test]
fn problem_1108_defanging_an_ip_address_example_1() {
    let input = "1.1.1.1";
    let expected_output = "1[.]1[.]1[.]1";
    let output = solution(&input);
    assert_eq!(output, expected_output);
}

#[test]
fn problem_1108_defanging_an_ip_address_example_2() {
    let input = "255.100.50.0";
    let expected_output = "255[.]100[.]50[.]0";
    let output = solution(&input);
    assert_eq!(output, expected_output);
}
