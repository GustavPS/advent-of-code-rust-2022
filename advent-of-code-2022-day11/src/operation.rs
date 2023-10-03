#[derive(Debug)]
pub enum Modifier {
    Value(usize),
    OldValue
}

#[derive(Debug)]
#[derive(Copy)]
#[derive(Clone)]
pub enum MathOperation {
    Add,
    Subtract,
    Multiply,
    Divide,
    Modulo
}

impl MathOperation {
    pub fn apply(&self, a: usize, b: usize) -> usize {
        match self {
            MathOperation::Add => a + b,
            MathOperation::Divide => a / b,
            MathOperation::Multiply => a * b,
            MathOperation::Subtract => a - b,
            MathOperation::Modulo => a % b
        }
    }
}

#[derive(Debug)]
pub struct Operation {
    math_operation: MathOperation,
    modifier: Modifier
}

impl Operation {
    pub fn new(operation: MathOperation, modifier: Modifier) -> Self {
        Self {
            math_operation: operation,
            modifier
        }
    }

    pub fn run(&self, old_value: usize) -> usize {
        match self.modifier {
            Modifier::Value(modifier) => self.math_operation.apply(old_value, modifier),
            Modifier::OldValue => self.math_operation.apply(old_value, old_value)
        }
    }
}

#[derive(Debug)]
pub struct TestOperation {
    operation: Operation,
    expected_value: u32,
    monkey_id_if_true: u32,
    monkey_id_if_false: u32
}

impl TestOperation {
    pub fn new(operation: Operation, expected_value: u32, if_true: u32, if_false: u32) -> Self {
        Self {
            operation,
            expected_value,
            monkey_id_if_true: if_true,
            monkey_id_if_false: if_false
        }
    }

    pub fn test(&self, value: usize) -> u32 {
        let result = self.operation.run(value);

        if result == self.expected_value as usize {
            return self.monkey_id_if_true;
        }
        self.monkey_id_if_false
    }
}