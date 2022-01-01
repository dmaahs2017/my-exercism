#[derive(Debug)]
pub enum CalculatorInput {
    Add,
    Subtract,
    Multiply,
    Divide,
    Value(i32),
}

pub fn evaluate(inputs: &[CalculatorInput]) -> Option<i32> {
    use CalculatorInput::*;
    let mut stack = vec![];
    for input in inputs {
        match input {
            Add => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(x + y);
            },
            Subtract => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y - x);
            },
            Multiply => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(x * y);
            },
            Divide => {
                let (x, y) = (stack.pop()?, stack.pop()?);
                stack.push(y / x);
            },
            Value(x) => stack.push(*x)
        }
    }

    if stack.len() > 1 {
        return None;
    }

    stack.pop()
}
