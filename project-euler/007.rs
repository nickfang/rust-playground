// By listing the first six prime numbers: 2, 3, 5, 7, 11, 13, we can see that the 6th prime is 13.
// What is the 10001st prime number?

use std::time::Instant;

fn main() {
  let start = Instant::now();

  // Your solution logic here...
  let mut answer = 0; // Replace with your calculated answer
  let mut primes = vec![2, 3, 5, 7, 11, 13];
  let mut num = 15;
  while primes.len() < 10001 {
    let mut is_prime = true;
    for prime in &primes {
      if num % prime == 0 {
        is_prime = false;
        break;
      }
    }
    if is_prime {
      primes.push(num);
      answer = num;
    }
    num += 2;
  }
  let duration = start.elapsed();
  println!("Problem 007: Answer = {}, Time: {:?}", answer, duration);
  // 104743
}
