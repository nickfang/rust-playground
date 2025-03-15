#[cfg(test)]
mod tests {
  use std::process::Command;
  use super::*;

  #[test]
  fn test_main_output() {
    // Run the main program and capture its output
    let output = Command::new("cargo").arg("run").output().expect("Failed to execute command");

    // Convert the output bytes to string
    let output_str = String::from_utf8(output.stdout).expect("Invalid UTF-8 output");

    // Assert that the output matches expected
    assert_eq!(output_str.trim(), "Hello, world!");
  }

  #[test]
  fn test_hl7_parsing() {
    let hl7_input = "MSH|^~\\&|Lab|Lab|Lab|Lab|202503061432||ORU^R01|202503061432|";
    let result = hl7ToJson(hl7_input.to_string());

    // The result should contain the MSH segment
    assert!(result.contains("\"0\": {"));
    assert!(result.contains("\"value\": \"MSH\""));

    // Check for the ORU^R01 parsing
    assert!(result.contains("\"8\": {"));
    assert!(result.contains("\"value\": \"ORU^R01\""));
    assert!(result.contains("\"0\": \"ORU\""));
    assert!(result.contains("\"1\": \"R01\""));
  }
}
