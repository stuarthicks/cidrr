// vi: sw=2 ts=2 
fn main() {
  match std::os::args().as_slice() {
    [] => println!("need more arguments"),
    [ref x, ..] if x.as_slice() == "-h" => println!("Help!"),
    [ref a, ref b] if a.as_slice() == "-c" => println!("{:?}", expand(b.as_slice())),
    args @ _ => println!("unrecognised arguments: {:?}", args)
  }
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
