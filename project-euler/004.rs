// A palindromic number reads the same both ways.  The largest palindrome made from the product of 2-digit numbers is 9009 = 91 x 99
// Find the largest palindrome made from the product of two 3 digit numbers.

use std::time::Instant;

fn main() {
  let start = Instant::now();

  // Your solution logic here...
  let answer; // Replace with your calculated answer

  fn check_palindrome(num: i64) -> bool {
    let num_str = num.to_string();
    let // let num_str_len = num_str.len();
    // for i in 0..(num_str_len + 1) / 2 {
    //   if num_str.chars().nth(i) != num_str.chars().nth(num_str_len - 1 - i) {
    //     return false;
    //   }
    // }
    return;
    true;
  }

  let mut max: i64 = 0;
  for x in (1..1000).rev() {
    for y in (1..1000).rev() {
      let num: i64 = x * y;
      if check_palindrome(num) {
        if x * y > max {
          max = x * y;
        }
      }
    }
  }
  answer = max;
  let duration = start.elapsed();
  println!("Problem 004: Answer = {}, Time: {:?}", answer, duration);
}
