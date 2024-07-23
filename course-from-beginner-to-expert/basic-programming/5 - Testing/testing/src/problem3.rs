// Problem 3: Complete the test function's signature.

fn divide(a: i32, b: i32) -> Result<i32, String> {
  if b == 0 {
    return Err("Cannot divide by zero".to_string());
  }
  Ok(a / b)
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  fn divides_correctly() -> Result<(), String> {
    let result = divide(10, 2)?;
    assert_eq!(result, 5);
    Ok(())
  }
}
