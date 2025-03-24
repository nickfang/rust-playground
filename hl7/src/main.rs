use std::collections::HashMap;
use regex::Regex;

#[derive(Debug)]
enum SubComponent {
  Value(String),
  Values(Vec<String>),
}

impl SubComponent {
  fn new_value(value: String) -> Self {
    SubComponent::Value(value)
  }

  fn new_values(values: Vec<String>) -> Self {
    SubComponent::Values(values)
  }
}

#[derive(Debug)]
enum Component {
  Value(String),
  Values(Vec<String>),
  SubComponents(HashMap<String, SubComponent>),
}

impl Component {
  fn new_value(value: String) -> Self {
    Component::Value(value)
  }

  fn new_values(values: Vec<String>) -> Self {
    Component::Values(values)
  }

  fn new_sub_components(components: HashMap<String, SubComponent>) -> Self {
    Component::SubComponents(components)
  }
}

#[derive(Debug)]
enum Field {
  Value(String),
  Values(Vec<String>),
  Components(HashMap<String, Component>),
}

impl Field {
  fn new_value(value: String) -> Self {
    Field::Value(value)
  }

  fn new_values(values: Vec<String>) -> Self {
    Field::Values(values)
  }

  fn new_components(components: HashMap<String, Component>) -> Self {
    Field::Components(components)
  }
}

type Segment = HashMap<String, Field>;

fn handle_escaped_separators(mut elements: Vec<String>, separator: char) -> Vec<String> {
  let mut i = 0;
  while i < elements.len() {
    if elements[i].ends_with(r"\") && i < elements.len() {
      elements[i].pop();
      elements[i].push(separator);
      let next_element = elements.remove(i + 1);
      elements[i].push_str(&next_element);
    } else {
      i += 1;
    }
  }
  elements
}

fn split_by_separator(string: String, separator: char) -> Vec<String> {
  let mut result = Vec::new();
  result = string
    .split(separator)
    .map(|s| s.to_string())
    .collect();
  result = handle_escaped_separators(result, separator);
  result
}

fn sort_numeric_strings(strings: Vec<String>) -> Vec<String> {
  let mut sorted_strings = strings.clone();
  sorted_strings.sort_by(|a, b| {
    let a_num = a.parse::<usize>().unwrap_or(0);
    let b_num = b.parse::<usize>().unwrap_or(0);
    a_num.cmp(&b_num)
  });
  sorted_strings
}

fn handle_escaped_characters(string: String) -> String {
  let re = Regex::new(r"\\([|^&~\\])").unwrap();
  let result = re.replace_all(&string, "$1").to_string();
  result
}

fn parse_repeat(string: String) -> Vec<String> {
  let split_string = split_by_separator(string, '~');
  split_string
    .iter()
    .map(|s| handle_escaped_characters(s.to_string()))
    .collect()
}

fn finalize_segment(mut segment: Segment) -> Segment {
  let sorted_field_keys: Vec<String> = sort_numeric_strings(segment.keys().cloned().collect());

  for field_key in sorted_field_keys {
    let field_value = segment.get_mut(&field_key).unwrap();
    match field_value {
      Field::Value(v) => {
        if v.is_empty() {
          println!("{}: {}", field_key, v);
          continue;
        }
        let values = parse_repeat(v.clone());
        if values.len() > 1 {
          let final_values: Vec<String> = values
            .iter()
            .map(|v| handle_escaped_characters(v.clone()))
            .collect();
          *field_value = Field::new_values(final_values.clone());
          println!("{}: {:?}", field_key, final_values);
          continue;
        } else {
          let final_value = handle_escaped_characters(v.clone());
          println!("{}: {}", field_key, final_value);
          *field_value = Field::new_value(final_value);
        }
      }
      Field::Values(v) => println!("{}: {:?}", field_key, v),
      Field::Components(components) => {
        println!("{}:", field_key);
        let sorted_component_keys: Vec<String> = sort_numeric_strings(
          components.keys().cloned().collect()
        );
        for component_key in sorted_component_keys {
          let component_value = components.get_mut(&component_key).unwrap();
          match component_value {
            Component::Value(v) => {
              if v.is_empty() {
                println!("  {}: {}", component_key, v);
                continue;
              }
              let values = parse_repeat(v.clone());
              if values.len() > 1 {
                let final_values: Vec<String> = values
                  .iter()
                  .map(|v| handle_escaped_characters(v.clone()))
                  .collect();
                *component_value = Component::new_values(final_values.clone());
                println!("  {}: {:?}", component_key, final_values);
              } else {
                let final_value = handle_escaped_characters(v.clone());
                println!("  {}: {}", component_key, final_value);
                *component_value = Component::new_value(final_value);
              }
            }
            Component::Values(v) => println!("  {}: {:?}", component_key, v),
            Component::SubComponents(sub_components) => {
              println!("  {}:", component_key);
              let sorted_sub_component_keys: Vec<String> = sort_numeric_strings(
                sub_components.keys().cloned().collect()
              );
              for sub_component_key in sorted_sub_component_keys {
                let sub_component_value = sub_components.get_mut(&sub_component_key).unwrap();

                match sub_component_value {
                  SubComponent::Value(v) => {
                    if v.is_empty() {
                      println!("    {}: {}", sub_component_key, v);
                      continue;
                    }
                    let values = parse_repeat(v.clone());
                    if values.len() > 1 {
                      let final_values: Vec<String> = values
                        .iter()
                        .map(|v| handle_escaped_characters(v.clone()))
                        .collect();
                      *sub_component_value = SubComponent::new_values(final_values.clone());
                      println!("    {}: {:?}", sub_component_key, values);
                    } else {
                      let final_value = handle_escaped_characters(v.clone());
                      println!("    {}: {}", sub_component_key, final_value);
                      *sub_component_value = SubComponent::new_value(final_value);
                    }
                  }
                  SubComponent::Values(v) => println!("    {}: {:?}", sub_component_key, v),
                }
              }
            }
          }
        }
      }
    }
  }
  segment
}

fn main() {
  let msh_segment = r"MSH|^~\&|Lab|Lab|Lab|Lab|202503061432||ORU^R01|202503061432|";
  let example_segments =
    "MSH|^~|EPIC|EPICADT|iFW|SMSADT|199912271408|CHARRIS|ADT^A04|1817457|D|2.5|
PID||0493575^^^2^ID 1|454721||DOE^JOHN^^^^|DOE^JOHN^^^^|19480203|M||B|254 MYSTREET AVE^^MYTOWN^OH^44123^USA||(216)123-4567|||M|NON|400003403~1129086|
NK1||ROE^MARIE^^^^|SPO||(216)123-4567||EC|||||||||||||||||||||||||||
PV1||O|168 ~219~C~PMA^^^^^^^^^||||277^ALLEN MYLASTNAME^BONNIE^^^^|||||||||| ||2688684|||||||||||||||||||||||||199912271408||||||002376853";
  let test_segments =
    r"MSH|^~\&|Sender\|App|Sender\^Facility|Receiver\&App|ReceiverFacility|202503160201||ORU^R01|MSG00001|P|2.5
PID|1||PatientID123\^ABC^^^Hospital\|ID||Patient\^Name\^With\^Carets||19700101|M|||123 Main St\^Any\|town\^TX^78704
OBR|1||Lab\&Order\^123|||||||||||||||||||||||
OBX|1|ST|ObservationID|This observation value contains all escapes: \| and \^ and \&|||||";
  let test = r"MSH|^~\&||&One^Two^Three^&^Three^Two^One&^^^||";
  let test_segments = test.split("\n").collect::<Vec<&str>>();
  for segment in test_segments {
    println!("{}", segment_to_json(segment.to_string()));
  }
}

fn segment_to_json(message: String) -> String {
  let mut segment = Segment::new();

  let fields = split_by_separator(message, '|');
  for (field_index, field) in fields.iter().enumerate() {
    if field.is_empty() {
      segment.insert(field_index.to_string(), Field::new_value("".to_string()));
      continue;
    }

    let components = split_by_separator(field.to_string(), '^');
    if components.len() == 1 {
      segment.insert(field_index.to_string(), Field::new_value(field.to_string()));
      continue;
    }
    let mut processed_components = HashMap::new();

    for (component_index, component) in components.into_iter().enumerate() {
      if component.is_empty() {
        processed_components.insert(
          component_index.to_string(),
          Component::new_value("".to_string())
        );
        continue;
      }

      let sub_components = split_by_separator(component.clone(), '&');
      if sub_components.len() == 1 {
        processed_components.insert(
          component_index.to_string(),
          Component::new_value(component.to_string())
        );
      } else {
        let mut processed_sub_components = HashMap::new();
        for (sub_index, sub_component) in sub_components.into_iter().enumerate() {
          processed_sub_components.insert(
            sub_index.to_string(),
            SubComponent::new_value(sub_component)
          );
        }
        processed_components.insert(
          component_index.to_string(),
          Component::new_sub_components(processed_sub_components)
        );
      }
    }

    segment.insert(field_index.to_string(), Field::new_components(processed_components));
  }

  finalize_segment(segment);
  return "".to_string();
}

#[test]
fn test_segment_to_json() {
  let message = r"MSH|^~\&|Lab\^Lab\&Lab\~Lab|202503061432|||ORU^R01|202503061432|";
  let json = segment_to_json(message.to_string());
  println!("{}", json);
}

#[test]
fn test_handle_escaped_separators() {
  let test_str = r"Field|With\|Escapes|and|without".to_string();
  let components = test_str
    .split("|")
    .map(|s| s.to_string())
    .collect::<Vec<String>>();
  let escaped_components = handle_escaped_separators(components, '|');
  assert_eq!(escaped_components, vec!["Field", "With|Escapes", "and", "without"]);
}

#[test]
fn test_split_by_separator() {
  let test_str1 =
    r"MSH|^~\&|Sender\|App|Sender\^Facility|Receiver\&App|ReceiverFacility|202503160201||ORU^R01|MSG00001|P|2.5";

  let components1 = split_by_separator(test_str1.to_string(), '|');
  assert_eq!(
    components1,
    vec![
      "MSH",
      "^~\\&",
      "Sender|App",
      "Sender\\^Facility",
      "Receiver\\&App",
      "ReceiverFacility",
      "202503160201",
      "",
      "ORU^R01",
      "MSG00001",
      "P",
      "2.5"
    ]
  );
  let test_str2 =
    r"PID|1||PatientID123\^ABC^^^Hospital\|ID||Patient\^Name\^With\^Carets||19700101|M|||123 Main St\^Any\|town\^TX^78704";
  let components2 = split_by_separator(test_str2.to_string(), '|');
  assert_eq!(
    components2,
    vec![
      "PID",
      "1",
      "",
      "PatientID123\\^ABC^^^Hospital|ID",
      "",
      "Patient\\^Name\\^With\\^Carets",
      "",
      "19700101",
      "M",
      "",
      "",
      "123 Main St\\^Any|town\\^TX^78704"
    ]
  );
  let fields = split_by_separator(components2[3].to_string(), '^');
  assert_eq!(fields, vec!["PatientID123^ABC", "", "", "Hospital|ID"]);
  let test_str3 = r"Patient\^Name\^With\^Carets";
  let components3 = split_by_separator(test_str3.to_string(), '^');
  assert_eq!(components3, vec!["Patient^Name^With^Carets"]);

  let test_str3 = r"OBR|1||Lab\&Order\^123|||||||||||||||||||||||";
  let test_str4 =
    r"OBX|1|ST|ObservationID|This observation value contains all escapes: \| and \^ and \&|||||";
}
