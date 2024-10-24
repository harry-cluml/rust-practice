# Rust Practice

## Project Structure

Each problem is implemented as a module.

For example, [Leetcode problem
2235](https://leetcode.com/problems/add-two-integers/description/)'s solution
and examples is in `src/problem_2235.rs`.

The solution to each problem is implemented with the name `fn solve(...)`.
and examples are implemented under the tests module in each problem module with
names like `fn example_1()`, `fn example_2()`.

For example, example 1 of problem 2235 is as follows:

```rust
// location: src/problem_2235.rs
// The solution implementation is here.

#[test]
fn example_1() {
    let num1 = 12;
    let num2 = 5;
    let expected_output = 17;
    let output = solve(num1, num2);
    assert_eq!(output, expected_output);
}

// Another examples are here.
```

## How to Run Examples?

### Run All Examples

All examples are implemented as test cases. so, you just run the tests.

```sh
cargo test
```

### Run Only Specific Problem's Examples

For example, if you want to run an example for **problem 2235**,

```sh
cargo test problem_2235
```
