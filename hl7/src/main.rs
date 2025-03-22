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

fn finalize_segment(segment: Segment) -> Segment {
  let sorted_field_keys: Vec<String> = sort_numeric_strings(segment.keys().cloned().collect());

  for field_key in sorted_field_keys {
    let field_value = segment.get(&field_key).unwrap();
    match field_value {
      Field::Value(v) => println!("{}: {}", field_key, v),
      Field::Values(v) => println!("{}: {:?}", field_key, v),
      Field::Components(components) => {
        let sorted_component_keys: Vec<String> = sort_numeric_strings(
          components.keys().cloned().collect()
        );
        for component_key in sorted_component_keys {
          let component_value = components.get(&component_key).unwrap();
          match component_value {
            Component::Value(v) => println!("  {}: {}", component_key, v),
            Component::Values(v) => println!("  {}: {:?}", component_key, v),
            Component::SubComponents(sub_components) => {
              let sorted_sub_component_keys: Vec<String> = sort_numeric_strings(
                sub_components.keys().cloned().collect()
              );
              for sub_component_key in sorted_sub_component_keys {
                let sub_component_value = sub_components.get(&sub_component_key).unwrap();

                match sub_component_value {
                  SubComponent::Value(v) => println!("    {}: {}", sub_component_key, v),
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
  let test_segment =
    r"MSH|^~\&|Sender\|App|Sender\^Facility|Receiver\&App|ReceiverFacility|202503160201||ORU^R01|MSG00001|P|2.5
PID|1||PatientID123\^ABC^^^Hospital\|ID||Patient\^Name\^With\^Carets||19700101|M|||123 Main St\^Any\|town\^TX^78704
OBR|1||Lab\&Order\^123|||||||||||||||||||||||
OBX|1|ST|ObservationID|This observation value contains all escapes: \| and \^ and \&|||||";
  let test_segments = test_segment.split("\n").collect::<Vec<&str>>();
  for segment in test_segments {
    println!("{}", segment_to_json(segment.to_string()));
  }
}

fn segment_to_json(message: String) -> String {
  let mut segment = Segment::new();

  // First split on field separator (|)
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
fn test_handle_field_separators() {
  let segment = r"MSH|^~\&|Lab|Lab|Lab|Lab|202503061432||ORU^R01|202503061432|".to_string();
  let new_segment = handle_escapes(segment, '|');
  println!("{}", new_segment);
  assert_eq!(new_segment, r"MSH|^~\&|Lab|Lab|Lab|Lab|202503061432||ORU^R01|202503061432|");

  // Test individual separators
  let pipe_test = r"\|".to_string();
  assert_eq!(handle_escapes(pipe_test, '|'), "|");

  let caret_test = r"\^".to_string();
  assert_eq!(handle_escapes(caret_test, '^'), "^");

  let amp_test = r"\&".to_string();
  assert_eq!(handle_escapes(amp_test, '&'), "&");
}

#[test]
fn test_escaped_field_separators() {
  let segment =
    r"MSH|^~\&|Lab\|More|Lab|Lab\|Extra|Lab|202503061432||ORU^R01|202503061432|".to_string();
  let json = segment_to_json(segment);
  assert!(json.contains("Lab|More")); // Check if escaped field separator was properly handled
  assert!(json.contains("Lab|Extra")); // Check another escaped field separator

  // Test more complex escaping scenarios
  let complex_segment =
    r"MSH|^~\&|Lab\|More\^Test|Field\|With\^Multiple\|Escapes|Simple\^With\^Carets|Field\&With\&Ampersands".to_string();
  let json_complex = segment_to_json(complex_segment);
  assert!(json_complex.contains("Lab|More^Test")); // Check escaped field separator with component separator
  assert!(json_complex.contains("Field|With^Multiple|Escapes")); // Check multiple escapes
  assert!(json_complex.contains("Simple^With^Carets")); // Check component separator handling
  assert!(json_complex.contains("Field&With&Ampersands")); // Check subcomponent separator handling
}

#[test]
fn test_ampersand_escaping() {
  let test_str = r"Field\&With\&Ampersands".to_string();
  let components = test_str
    .split("&")
    .map(|s| s.to_string())
    .collect::<Vec<String>>();
  println!("Initial split: {:?}", components);

  let processed = handle_escaped_separators(components, '&');
  println!("After processing: {:?}", processed);

  assert_eq!(processed.len(), 1);
  assert_eq!(processed[0], "Field&With&Ampersands");
}
