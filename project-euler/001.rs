// If we list all the natural numbers below 10 that are multiples of 3 or 5, we get 3,5,6 and 9. The sum of these multiples is 23.

// Find the sum of all the multiples of 3 or 5 below 1000.

fn main() {
  use std::time::Instant;
  let start = Instant::now();

  // Your solution logic here...
  let mut answer = 0; // Replace with your calculated answer
  for x in 1..1000 {
    if x % 3 == 0 || x % 5 == 0 {
      answer = answer + x
    }
  }

  let duration = start.elapsed();
  println!("Problem 001: Answer = {}, Time: {:?}", answer, duration);
}
