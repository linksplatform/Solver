// All 1-bit boolean function generator
// This program generates and implements all possible functions with 1-bit sized argument using NAND gates

/// Performs a NAND operation on two boolean inputs.
///
/// # Arguments
///
/// * `a` - A boolean input.
/// * `b` - A boolean input.
///
/// # Returns
///
/// * A boolean output representing the NAND operation on the inputs.
fn nand(a: bool, b: bool) -> bool {
  !(a && b)
}


/// Evaluates a function with the given truth table for a 1-bit input
fn evaluate_function(input: bool, truth_table: &[bool]) -> bool {
  truth_table[if input { 1 } else { 0 }]
}

/// Implements constant 0 using NAND gates: x AND (NOT x) = false always
fn constant_zero_nand(x: bool) -> bool {
  let not_x = nand(x, x);          // NOT x
  let result = nand(x, not_x);     // x NAND (NOT x) = NOT(x AND NOT x) = NOT(false) = true
  nand(result, result)             // NOT(true) = false
}

/// Implements identity using NAND gates: NOT(NOT x) = x
fn identity_nand(x: bool) -> bool {
  let not_x = nand(x, x);          // NOT x
  nand(not_x, not_x)               // NOT(NOT x) = x
}

/// Implements NOT using NAND gates: x NAND x = NOT x
fn not_nand(x: bool) -> bool {
  nand(x, x)
}

/// Implements constant 1 using NAND gates: x NAND (NOT x) = true always
fn constant_one_nand(x: bool) -> bool {
  let not_x = nand(x, x);          // NOT x
  nand(x, not_x)                   // x NAND (NOT x) = NOT(x AND NOT x) = NOT(false) = true
}

fn main() {
  println!("Generating all possible functions with 1-bit sized argument");
  println!("==========================================================");
  
  // For a single boolean input, there are exactly 4 possible functions
  let functions = [
    ("Constant 0", vec![false, false]),    // Always returns false
    ("Identity",   vec![false, true]),     // Returns input value  
    ("NOT",        vec![true, false]),     // Returns !input
    ("Constant 1", vec![true, true]),      // Always returns true
  ];
  
  println!("Truth table for all possible 1-bit functions:");
  println!("Input | F0 | F1 | F2 | F3");
  println!("------|----|----|----|----|");
  println!("  0   | {} | {} | {} | {} |", 
    if functions[0].1[0] { 1 } else { 0 },
    if functions[1].1[0] { 1 } else { 0 },
    if functions[2].1[0] { 1 } else { 0 },
    if functions[3].1[0] { 1 } else { 0 }
  );
  println!("  1   | {} | {} | {} | {} |", 
    if functions[0].1[1] { 1 } else { 0 },
    if functions[1].1[1] { 1 } else { 0 },
    if functions[2].1[1] { 1 } else { 0 },
    if functions[3].1[1] { 1 } else { 0 }
  );
  println!();
  
  for (i, (name, truth_table)) in functions.iter().enumerate() {
    println!("Function F{}: {} -> Truth table: {:?}", i, name, truth_table);
  }
  
  println!();
  println!("Implementing each function using NAND gates:");
  println!("============================================");
  
  let nand_implementations = [
    ("F0 (Constant 0)", "((x ↑ (x ↑ x)) ↑ (x ↑ (x ↑ x)))", constant_zero_nand as fn(bool) -> bool),
    ("F1 (Identity)", "(x ↑ x) ↑ (x ↑ x)", identity_nand as fn(bool) -> bool),
    ("F2 (NOT)", "x ↑ x", not_nand as fn(bool) -> bool), 
    ("F3 (Constant 1)", "x ↑ (x ↑ x)", constant_one_nand as fn(bool) -> bool),
  ];
  
  for (i, (name, expression, implementation)) in nand_implementations.iter().enumerate() {
    println!("{}: {}", name, expression);
    println!("   Verification:");
    
    let mut correct = true;
    for input in [false, true] {
      let expected = evaluate_function(input, &functions[i].1);
      let actual = implementation(input);
      let input_str = if input { 1 } else { 0 };
      let expected_str = if expected { 1 } else { 0 };
      let actual_str = if actual { 1 } else { 0 };
      
      if expected == actual {
        println!("     Input: {} -> Expected: {} -> Actual: {} ✓", input_str, expected_str, actual_str);
      } else {
        println!("     Input: {} -> Expected: {} -> Actual: {} ✗", input_str, expected_str, actual_str);
        correct = false;
      }
    }
    
    if correct {
      println!("   ✓ Implementation is correct!");
    } else {
      println!("   ✗ Implementation has errors!");
    }
    println!();
  }
  
  println!("Summary:");
  println!("========");
  println!("All 4 possible functions with 1-bit argument have been generated and implemented using NAND gates.");
  println!("These represent the complete set of boolean functions for a single input variable.");
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn test_nand() {
    assert_eq!(nand(false, false), true);
    assert_eq!(nand(false, true), true);
    assert_eq!(nand(true, false), true);
    assert_eq!(nand(true, true), false);
  }

  #[test]
  fn test_constant_zero() {
    assert_eq!(constant_zero_nand(false), false);
    assert_eq!(constant_zero_nand(true), false);
  }

  #[test]
  fn test_identity() {
    assert_eq!(identity_nand(false), false);
    assert_eq!(identity_nand(true), true);
  }

  #[test]
  fn test_not() {
    assert_eq!(not_nand(false), true);
    assert_eq!(not_nand(true), false);
  }

  #[test]
  fn test_constant_one() {
    assert_eq!(constant_one_nand(false), true);
    assert_eq!(constant_one_nand(true), true);
  }

  #[test]
  fn test_all_functions() {
    let functions = [
      ("Constant 0", vec![false, false]),
      ("Identity",   vec![false, true]),
      ("NOT",        vec![true, false]),
      ("Constant 1", vec![true, true]),
    ];

    let implementations = [
      constant_zero_nand as fn(bool) -> bool,
      identity_nand as fn(bool) -> bool,
      not_nand as fn(bool) -> bool,
      constant_one_nand as fn(bool) -> bool,
    ];

    for (i, (name, truth_table)) in functions.iter().enumerate() {
      for input in [false, true] {
        let expected = evaluate_function(input, truth_table);
        let actual = implementations[i](input);
        assert_eq!(actual, expected, 
          "Function {} failed for input {}: expected {}, got {}", 
          name, input, expected, actual);
      }
    }
  }
}
