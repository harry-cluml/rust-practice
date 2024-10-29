#[derive(PartialEq, Eq, Clone, Debug)]
struct ListNode {
    val: i32,
    next: Option<Box<ListNode>>,
}

fn traverse(node: Option<Box<ListNode>>, mut value: i32) -> i32 {
    let Some(node) = node else {
        return value;
    };
    value <<= 1;
    value += node.val;
    traverse(node.next, value)
}

#[allow(dead_code)]
fn solve(head: &Option<Box<ListNode>>) -> i32 {
    let mut output = 0;
    let mut current = head;
    while let Some(node) = current {
        output <<= 1;
        output += node.val;
        current = &node.next;
    }
    output
}

#[allow(dead_code)]
fn solve_rec(head: Option<Box<ListNode>>) -> i32 {
    traverse(head, 0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn example_1() {
        let head: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));
        let expected_output = 5;
        let output = solve(&head);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2() {
        let head: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: None }));
        let expected_output = 0;
        let output = solve(&head);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_1_rec() {
        let head: Option<Box<ListNode>> = Some(Box::new(ListNode {
            val: 1,
            next: Some(Box::new(ListNode {
                val: 0,
                next: Some(Box::new(ListNode { val: 1, next: None })),
            })),
        }));
        let expected_output = 5;
        let output = solve_rec(head);
        assert_eq!(output, expected_output);
    }

    #[test]
    fn example_2_rec() {
        let head: Option<Box<ListNode>> = Some(Box::new(ListNode { val: 0, next: None }));
        let expected_output = 0;
        let output = solve_rec(head);
        assert_eq!(output, expected_output);
    }
}
