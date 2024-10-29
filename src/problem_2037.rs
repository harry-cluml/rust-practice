#[allow(dead_code)]
fn solve(seats: &mut [i32], students: &mut [i32]) -> i32 {
    seats.sort_unstable();
    students.sort_unstable();
    seats
        .iter()
        .zip(students)
        .fold(0, |acc, (&seat, &mut student)| acc + (seat - student).abs())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let mut seats = [3, 1, 5];
        let mut students = [2, 7, 4];
        let expected_output = 4;
        let output = solve(&mut seats, &mut students);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let mut seats = [4, 1, 5, 9];
        let mut students = [1, 3, 2, 6];
        let expected_output = 7;
        let output = solve(&mut seats, &mut students);
        assert_eq!(output, expected_output);
    }
}
