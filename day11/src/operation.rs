#[derive(Debug, Clone)]
pub struct Operation(OperationType, OperatorType, OperationType);

#[derive(Debug, Clone)]
enum OperationType {
    Old,
    New(usize),
}

#[derive(Debug, Clone)]
enum OperatorType {
    Addition,
    Multiplication,
}

use OperationType::*;
use OperatorType::*;

impl Operation {
    pub fn get_worry_level(&self, item: usize) -> usize {
        match self {
            Operation(Old, Addition, Old) => item + item,
            Operation(Old, Multiplication, Old) => item * item,
            Operation(Old, Addition, New(num)) => item + num,
            Operation(Old, Multiplication, New(num)) => item * num,
            Operation(New(num), Addition, Old) => num + item,
            Operation(New(num), Multiplication, Old) => num * item,
            Operation(New(num1), Addition, New(num2)) => num1 + num2,
            Operation(New(num1), Multiplication, New(num2)) => num1 * num2,
        }
    }
}

pub fn get_operation(operation: &[&str]) -> Operation {
    match operation {
        ["old", "+", "old"] => Operation(Old, Addition, Old),
        ["old", "*", "old"] => Operation(Old, Multiplication, Old),
        ["old", "+", num] => Operation(Old, Addition, New(num.parse::<usize>().unwrap())),
        ["old", "*", num] => Operation(Old, Multiplication, New(num.parse::<usize>().unwrap())),
        [num, "+", "old"] => Operation(New(num.parse::<usize>().unwrap()), Addition, Old),
        [num, "*", "old"] => Operation(New(num.parse::<usize>().unwrap()), Multiplication, Old),
        [num1, "+", num2] => Operation(
            New(num1.parse::<usize>().unwrap()),
            Addition,
            New(num2.parse::<usize>().unwrap()),
        ),
        [num1, "*", num2] => Operation(
            New(num1.parse::<usize>().unwrap()),
            Multiplication,
            New(num2.parse::<usize>().unwrap()),
        ),
        _ => panic!("wrong operation"),
    }
}
