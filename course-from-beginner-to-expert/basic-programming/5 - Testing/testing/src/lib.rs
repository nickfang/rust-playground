mod shapes {
  pub struct Circle {
    radius: f32,
  }
  impl Circle {
    pub fn new(radius: f32) -> Circle {
      Circle { radius }
    }
    pub fn new_1(radius: f32) -> Result<Circle, String> {
      if radius >= 0.0 {
        Ok(Circle { radius })
      } else {
        Err(String::from("Radius cannot be negative"))
      }
    }
    pub fn new_2(radius: f32) -> Circle {
      match radius {
        -10.0..=0.0 => panic!("Radius cannot be negative"),
        ..=-10.0 => panic!("Radius is lesser then -10.0"),
        _ => Circle { radius },
      }
    }
    pub fn contains(&self, other: &Circle) -> bool {
      self.radius > other.radius
    }
    pub fn area(&self) -> f32 {
      std::f32::consts::PI * self.radius * self.radius
    }
  }
}

#[cfg(test)]
mod tests {
  use super::*;
  #[test]
  fn larger_circle_should_contain_smaller() {
    let larger = shapes::Circle::new(5.0);
    let smaller = shapes::Circle::new(2.0);
    assert_eq!(larger.contains(&smaller), true, "Custom failure message");
    assert_ne!(larger.contains(&smaller), false);
    assert!(larger.contains(&smaller));
  }
  #[test]
  fn smaller_circle_should_not_contain_larger() {
    let larger = shapes::Circle::new(5.0);
    let smaller = shapes::Circle::new(2.0);
    assert_eq!(!smaller.contains(&larger), true);
  }
  #[test]
  fn should_not_create_circle() -> Result<(), String> {
    let some_circle = shapes::Circle::new_1(1.0)?;
    Ok(())
  }
  #[test]
  #[should_panic(expected = "is lesser then -10.0")]
  fn should_not_create_and_painc() {
    let some_circle = shapes::Circle::new_2(-11.0);
  }
}
