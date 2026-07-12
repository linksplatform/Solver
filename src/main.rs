use std::fmt;

const INPUT_COUNT: usize = 2;
const FUNCTION_COUNT: usize = 1 << INPUT_COUNT;

#[derive(Clone, Debug, Eq, PartialEq)]
enum Expression {
  Input,
  Nand(Box<Expression>, Box<Expression>),
}

impl Expression {
  fn nand(left: &Self, right: &Self) -> Self {
    Self::Nand(Box::new(left.clone()), Box::new(right.clone()))
  }
}

impl fmt::Display for Expression {
  fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
    match self {
      Self::Input => formatter.write_str("x"),
      Self::Nand(left, right) => write!(formatter, "({left} ↑ {right})"),
    }
  }
}

#[derive(Clone, Debug, Eq, PartialEq)]
struct Function {
  outputs: [bool; INPUT_COUNT],
  expression: Expression,
  nand_count: usize,
}

fn nand(left: bool, right: bool) -> bool {
  !(left && right)
}

fn combine(left: &Function, right: &Function) -> Function {
  Function {
    outputs: [
      nand(left.outputs[0], right.outputs[0]),
      nand(left.outputs[1], right.outputs[1]),
    ],
    expression: Expression::nand(&left.expression, &right.expression),
    nand_count: left.nand_count + right.nand_count + 1,
  }
}

fn truth_table_index(outputs: [bool; INPUT_COUNT]) -> usize {
  usize::from(outputs[0]) | (usize::from(outputs[1]) << 1)
}

/// Exhaustively combines known expressions with NAND until every truth table is found.
///
/// Candidates are considered in increasing NAND-gate count, so the first expression
/// stored for each truth table is also one of its smallest NAND implementations.
fn generate_all_functions() -> [Function; FUNCTION_COUNT] {
  let input = Function {
    outputs: [false, true],
    expression: Expression::Input,
    nand_count: 0,
  };
  let mut functions: [Option<Function>; FUNCTION_COUNT] = std::array::from_fn(|_| None);
  let input_index = truth_table_index(input.outputs);
  functions[input_index] = Some(input);

  for nand_count in 1.. {
    let known: Vec<Function> = functions.iter().flatten().cloned().collect();
    let mut discovered = Vec::new();

    for left in &known {
      for right in &known {
        if left.nand_count + right.nand_count + 1 != nand_count {
          continue;
        }
        let candidate = combine(left, right);
        let index = truth_table_index(candidate.outputs);
        if functions[index].is_none()
          && !discovered
            .iter()
            .any(|function: &Function| function.outputs == candidate.outputs)
        {
          discovered.push(candidate);
        }
      }
    }

    for function in discovered {
      let index = truth_table_index(function.outputs);
      functions[index] = Some(function);
    }
    if functions.iter().all(Option::is_some) {
      break;
    }
  }

  functions.map(Option::unwrap)
}

fn bit(value: bool) -> u8 {
  u8::from(value)
}

fn main() {
  let functions = generate_all_functions();

  println!("All {FUNCTION_COUNT} possible functions of one Boolean argument:");
  println!("F | f(0) | f(1) | NAND gates | expression");
  println!("--|------|------|------------|-----------");
  for (index, function) in functions.iter().enumerate() {
    println!(
      "{index} |   {}  |   {}  |      {}     | {}",
      bit(function.outputs[0]),
      bit(function.outputs[1]),
      function.nand_count,
      function.expression,
    );
  }
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn nand_has_the_expected_truth_table() {
    assert!(nand(false, false));
    assert!(nand(false, true));
    assert!(nand(true, false));
    assert!(!nand(true, true));
  }

  #[test]
  fn generates_every_possible_function_exactly_once() {
    let functions = generate_all_functions();
    let outputs: Vec<_> = functions.iter().map(|function| function.outputs).collect();

    assert_eq!(
      outputs,
      vec![[false, false], [true, false], [false, true], [true, true]]
    );
  }

  #[test]
  fn finds_smallest_nand_implementations() {
    let functions = generate_all_functions();
    let gate_counts: Vec<_> = functions
      .iter()
      .map(|function| function.nand_count)
      .collect();

    assert_eq!(gate_counts, vec![5, 1, 0, 2]);
    assert_eq!(
      functions[0].expression.to_string(),
      "(((x ↑ x) ↑ x) ↑ ((x ↑ x) ↑ x))"
    );
    assert_eq!(functions[1].expression.to_string(), "(x ↑ x)");
    assert_eq!(functions[2].expression.to_string(), "x");
    assert_eq!(functions[3].expression.to_string(), "((x ↑ x) ↑ x)");
  }
}
