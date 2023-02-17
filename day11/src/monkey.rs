use crate::operation::Operation;
use std::collections::VecDeque;

#[derive(Clone)]
pub struct Monkey {
    pub id: usize,
    pub starting_items: VecDeque<usize>,
    pub operation: Option<Operation>,
    pub divisible: usize,
    pub monkey_test: (usize, usize),
    pub times_inspected: usize,
}

impl std::fmt::Display for Monkey {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        writeln!(f, "Monkey {{")?;
        writeln!(f, "    id: {}", self.id)?;
        writeln!(f, "    items: {:?}", self.starting_items)?;
        writeln!(f, "    operation: {:?}", self.operation.as_ref().unwrap())?;
        writeln!(f, "    divisible: {}", self.divisible)?;
        writeln!(
            f,
            "    Monkey test: (true: {} false: {})",
            self.monkey_test.0, self.monkey_test.1
        )?;
        writeln!(f, "}}")
    }
}
