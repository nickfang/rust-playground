mod hl7ToJSON {
  pub fn hl7ToJSON(hl7: String) -> String {
    let json = serde_json::to_string(&hl7).unwrap();
    json
  }
}
