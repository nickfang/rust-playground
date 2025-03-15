use std::collections::HashMap;
use regex::Regex;

type SubSubComposite = String;

#[derive(Debug)]
struct SubComposite {
  sub_composite_value: String,
  sub_sub_composites: HashMap<String, SubSubComposite>,
}

impl SubComposite {
  fn new(value: String) -> Self {
    let mut sub_composite = SubComposite {
      sub_composite_value: String::new(),
      sub_sub_composites: HashMap::new(),
    };
    sub_composite.sub_composite_value = value;
    sub_composite
  }
}

#[derive(Debug)]
struct Composite {
  composite_value: String,
  sub_composites: HashMap<String, SubComposite>,
}

impl Composite {
  fn new(value: String) -> Self {
    let mut composite = Composite {
      composite_value: String::new(),
      sub_composites: HashMap::new(),
    };
    composite.composite_value = value;
    composite
  }
}

type HL7Message = HashMap<String, Composite>;

fn main() {
  let hl7Example2 = "MSH|^~\\&|Lab|Lab|Lab|Lab|202503061432||ORU^R01|202503061432|";
  let hl7Example =
    "MSH|^~|EPIC|EPICADT|iFW|SMSADT|199912271408|CHARRIS|ADT^A04|1817457|D|2.5|
PID||0493575^^^2^ID 1|454721||DOE^JOHN^^^^|DOE^JOHN^^^^|19480203|M||B|254 MYSTREET AVE^^MYTOWN^OH^44123^USA||(216)123-4567|||M|NON|400003403~1129086|
NK1||ROE^MARIE^^^^|SPO||(216)123-4567||EC|||||||||||||||||||||||||||
PV1||O|168 ~219~C~PMA^^^^^^^^^||||277^ALLEN MYLASTNAME^BONNIE^^^^|||||||||| ||2688684|||||||||||||||||||||||||199912271408||||||002376853";
  println!("{}", hl7ToJson(hl7Example2.to_string()));
}

fn handleEscapes(hl7: String) -> String {
  let re = Regex::new(r"\\([|^~&])").unwrap();

  let new_hl7 = re.replace_all(&hl7, "$1").to_string();

  new_hl7
}

#[test]
fn test_handle_escapes() {
  let mut hl7 = r"MSH|^~\\&|Lab|Lab|Lab|Lab|202503061432||ORU^R01|202503061432|".to_string();
  let mut new_hl7 = handleEscapes(hl7);
  println!("{}", new_hl7);
  assert_eq!(new_hl7, r"MSH|^~\&|Lab|Lab|Lab|Lab|202503061432||ORU^R01|202503061432|");
  hl7 = r"\|\^\&\~".to_string();
  new_hl7 = handleEscapes(hl7);
  println!("{}", new_hl7);
  assert_eq!(new_hl7, r"|^&~");
}

fn hl7ToJson(message: String) -> String {
  let mut message_map = HL7Message::new();

  let composites = message
    .split("|")
    .map(|s| s.to_string())
    .collect::<Vec<String>>();
  for (composite_index, composite) in composites.iter().enumerate() {
    message_map.insert(composite_index.clone().to_string(), Composite::new("".to_string()));
    if composite.is_empty() {
      continue;
    }
    let subcomposites = composite
      .split("^")
      .map(|s| s.to_string())
      .collect::<Vec<String>>();
    if subcomposites.len() == 1 {
      message_map.get_mut(&composite_index.to_string()).unwrap().composite_value =
        composite.to_string();
      continue;
    }
    for (subcomposite_index, subcomposite) in subcomposites.iter().enumerate() {
      message_map
        .get_mut(&composite_index.to_string())
        .unwrap()
        .sub_composites.insert(
          subcomposite_index.to_string(),
          SubComposite::new(subcomposite.to_string())
        );
      if subcomposite.is_empty() {
        continue;
      }
      let subsubcomposites = subcomposite
        .split("&")
        .map(|s| s.to_string())
        .collect::<Vec<String>>();
      if subsubcomposites.len() == 1 {
        message_map
          .get_mut(&composite_index.to_string())
          .unwrap()
          .sub_composites.get_mut(&subcomposite_index.to_string())
          .unwrap().sub_composite_value = subcomposite.to_string();
        continue;
      }
      for (subsubcomposite_index, subsubcomposite) in subsubcomposites.iter().enumerate() {
        message_map
          .get_mut(&composite_index.to_string())
          .unwrap()
          .sub_composites.get_mut(&subcomposite_index.to_string())
          .unwrap()
          .sub_sub_composites.insert(
            subsubcomposite_index.to_string(),
            subsubcomposite.to_string()
          );
      }
    }
  }

  // Create a formatted string to hold the result
  let mut result = String::new();
  result.push_str("{\n");

  // Collect and sort the keys
  let mut keys: Vec<&String> = message_map.keys().collect();
  keys.sort_by(|a, b| {
    let a_num = a.parse::<usize>().unwrap_or(0);
    let b_num = b.parse::<usize>().unwrap_or(0);
    a_num.cmp(&b_num)
  });

  // Debug print to verify sorting
  println!("Sorted keys: {:?}", keys);

  for key in keys {
    let composite = &message_map[key];
    result.push_str(&format!("  \"{}\": {{\n", key));
    result.push_str(&format!("    \"value\": \"{}\",\n", composite.composite_value));

    if !composite.sub_composites.is_empty() {
      result.push_str("    \"sub_composites\": {\n");

      let mut sub_keys: Vec<&String> = composite.sub_composites.keys().collect();
      sub_keys.sort_by(|a, b| {
        let a_num = a.parse::<usize>().unwrap_or(0);
        let b_num = b.parse::<usize>().unwrap_or(0);
        a_num.cmp(&b_num)
      });

      // Debug print to verify sub-sorting
      println!("Sorted sub_keys for {}: {:?}", key, sub_keys);

      for sub_key in sub_keys {
        let sub_composite = &composite.sub_composites[sub_key];
        result.push_str(&format!("      \"{}\": {{\n", sub_key));
        result.push_str(
          &format!("        \"value\": \"{}\",\n", sub_composite.sub_composite_value)
        );

        if !sub_composite.sub_sub_composites.is_empty() {
          result.push_str("        \"sub_sub_composites\": {\n");

          let mut sub_sub_keys: Vec<&String> = sub_composite.sub_sub_composites.keys().collect();
          sub_sub_keys.sort_by(|a, b| {
            let a_num = a.parse::<usize>().unwrap_or(0);
            let b_num = b.parse::<usize>().unwrap_or(0);
            a_num.cmp(&b_num)
          });

          // Debug print to verify sub-sub-sorting
          println!("Sorted sub_sub_keys for {}.{}: {:?}", key, sub_key, sub_sub_keys);

          for sub_sub_key in sub_sub_keys {
            let sub_sub_value = &sub_composite.sub_sub_composites[sub_sub_key];
            result.push_str(&format!("          \"{}\": \"{}\",\n", sub_sub_key, sub_sub_value));
          }

          result.push_str("        }\n");
        }

        result.push_str("      },\n");
      }

      result.push_str("    }\n");
    }

    result.push_str("  },\n");
  }

  // Remove trailing comma from the last item
  if result.ends_with(",\n") {
    result.truncate(result.len() - 2);
    result.push('\n');
  }

  result.push_str("}");
  result
}
