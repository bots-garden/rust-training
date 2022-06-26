fn add(a: f64, b: f64) -> f64 {
  a + b
}

fn main() {
  let a: f64 = 40.0;
  let b: f64 = 2.0;
  println!("add {} & {} = {}", a, b, add(a, b));
}

/*
run `cargo test`
*/
#[test]
fn test_add() {
  assert_eq!(add(5.0, 5.0), 10.0);
  assert_ne!(add(40.0, 1.0), 42.0)

}
