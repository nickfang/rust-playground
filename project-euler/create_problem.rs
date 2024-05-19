use std::env;
use std::fs;

fn main() {
    // Get problem number from the command line argument
    let args: Vec<String> = env::args().collect();
    if args.len() != 2 {
        println!("Usage: generate_euler <problem_number>");
        return;
    }
    let problem_number = &args[1];

    // Create file name
    let file_name = format!("{}.rs", problem_number);

    // Template content
    let template = format!(
        r#"
use std::time::Instant;

fn main() {{
  let start = Instant::now();

  // Your solution logic here...
  let answer = 42; // Replace with your calculated answer

  let duration = start.elapsed();
  println!("Problem {}: Answer = {{}}, Time: {{:?}}", answer, duration);
}}
"#,
        problem_number
    );

    // Write the template to the file
    fs::write(&file_name, template).expect("Unable to write file");

    println!("Created template: {}", file_name);
}
