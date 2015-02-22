// vi: sw=2 ts=2 
#![feature(env)]
#![feature(core)]
use std::env;

fn main() {
  let args: Vec<String> = env::args().map(|arg| arg.to_string()).collect();
  let first_arg: &str = args[1].as_slice();
  let expanded: &str = expand(first_arg);
  println!("{:?}", expanded);
}

fn expand(cidr: &str) -> &str {
  let bits: &str = cidr.split('/').next().expect("Invalid CIDR");
  for i in bits.split('.') {
    println!("{:?}", i)
  }
  return bits;
}

#[test]
fn slash_zero_returns_same_ip() {
  let res = expand("10.1.2.3/0");
  assert!(res == "10.1.2.3");
}
