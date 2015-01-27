// vi: sw=2 ts=2 
use std::os;

fn main() {
  let args = os::args();
  let first_arg: &str = args[1].as_slice();
  let expanded: &str = expand(first_arg);
  println!("{:?}", expanded);
}

fn expand(cidr: &str) -> &str {
  let foo: &str = cidr.split('/').next().expect("Invalid CIDR");
  return foo;
}

#[test]
fn slash_zero_returns_same_ip() {
  let res = expand("10.1.2.3/0");
  assert!(res == "10.1.2.3");
}
