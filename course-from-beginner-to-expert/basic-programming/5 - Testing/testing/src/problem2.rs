// Problem 2: Add a suitable attribute so that the test passes.

fn find_max(nums: &[i32]) -> i32 {
  if nums.is_empty() {
    panic!("Empty number list");
  }
  let mut max = nums[0];
  for &num in nums {
    if num > max {
      max = num;
    }
  }
  max
}

#[cfg(test)]
mod tests {
  use super::*;

  #[test]
  #[should_panic]
  fn panics_on_empty_list() {
    let nums = [];
    let _max = find_max(&nums);
  }
}
