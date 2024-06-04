// 2520 is the smallest number that can be divided by each of the numbers from 1 to 10 without any remainder.
// What is the smallest positive number that is evenly divisible by all of the numbers from 1 to 20?

use std::time::Instant;

fn main() {
  let start = Instant::now();

  // Your solution logic here...
  let divisors: [i64; 11] = [20, 19, 18, 17, 16, 15, 14, 13, 12, 11, 7];
  let mut answer = 20;
  loop {
    let mut divisible: bool = true;

    for i in 0..11 {
      if answer % divisors[i] != 0 {
        divisible = false;
      }
    }
    if divisible {
      break;
    }
    answer += 2;
  }
  let duration = start.elapsed();
  println!("Problem 005: Answer = {}, Time: {:?}", answer, duration);
}
