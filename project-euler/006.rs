// The sum of the squares of the first ten natural numbers is 1^2 + 2^2 + ... + 10^2 = 385,
//
// The square of the sum of the first ten natural numbers is (1 + 2 + ... + 10)^2 = 55^2 = 3025,
//
// Hence the difference between the sum of the squares of the first ten natural numbers and the square of the sum is 3025 - 385 = 2640.
//
// Find the difference between the sum of the squares of the first one hundred natural numbers and the square of the sum.

use std::time::Instant;

fn main() {
  let start = Instant::now();

  let answer;
  let mut sum_of_squares = 0;
  let mut square_of_sums = 0;
  for x in 1..101 {
    sum_of_squares += x * x;
    square_of_sums += x;
  }
  square_of_sums = square_of_sums * square_of_sums;
  answer = square_of_sums - sum_of_squares;

  let duration = start.elapsed();
  println!("Problem 006: Answer = {}, Time: {:?}", answer, duration);
  // 25164150
}
