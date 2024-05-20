// The prime factors of 13195 are 5, 7, 13 and 29
// What is the largest prime factor of the number 600851475143

use std::time::Instant;

fn main() {
  let start = Instant::now();

  // Your solution logic here...
  let mut answer = 0; // Replace with your calculated answer

  let mut number = 600851475143;

  let end = (number as f64).sqrt() as i64;

  for x in 3..end {
    if number % x == 0 {
      number = number / x;
      answer = x;
    }
  }

  let duration = start.elapsed();
  println!("Problem 003: Answer = {}, Time: {:?}", answer, duration);
  // 6857
}
