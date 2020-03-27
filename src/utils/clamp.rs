pub fn clamp<T: Ord>(x: T, min: T, max: T) -> T {
  assert!(max > min);

  if x < min {
    min
  } else if x > max {
    max
  } else {
    x
  }
}
