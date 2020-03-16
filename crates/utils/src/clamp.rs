pub fn clamp<T: PartialOrd>(x: T, min: T, max: T) -> T {
  assert!(min < max);

  if x < min {
    min
  } else if x > max {
    max
  } else {
    x
  }
}
