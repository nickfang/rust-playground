// Problem 1: Complete the test functions so that they pass successfully.

pub fn is_divisible_by_three(num: i32) -> bool {
  num % 3 == 0
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn is_true_when_divisible_by_three() {
    assert!(is_divisible_by_three(9));
  }

  #[test]
  fn is_false_when_not_divisible_by_three() {
    assert!(!is_divisible_by_three(8));
  }
}
