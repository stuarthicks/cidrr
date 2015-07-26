// vi: sw=4 ts=4
use std::env;

fn main() {
    let args: Vec<String> = env::args().map(|arg| arg.to_string()).collect();
    let first_arg: &str = &args[1];
    let expanded: String = expand(first_arg);
    println!("result: {:?}", expanded);
}

pub fn expand(cidr: &str) -> String {
    let ip: &str = cidr.split('/').next().expect("Invalid CIDR");
    let binary: String = ip_to_binary(ip.to_string());
    println!("{:?}", binary);
    return ip.to_string();
}

pub fn ip_to_binary(ip: String) -> String {
    let mut binary: Vec<String> = Vec::new();
    for i in ip.split('.') {
        let encoded: u8 = u8::from_str_radix(i, 10).ok().unwrap();
        binary.push(format!("{:08b}", encoded));
    }
    return binary.connect("");
}

pub fn binary_to_ip(binary: String) -> String {
    let bin: &str = &binary;
    let bin_codepoints: Vec<_> = bin.chars().collect();
    let mut output: Vec<String> = Vec::new();
    for section in bin_codepoints.chunks(8) {
        let bit: String = section.iter().cloned().collect::<String>();
        output.push(format!["{}", u8::from_str_radix(&bit, 2).unwrap()]);
    }
    return output.connect(".");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn slash_zero_returns_same_ip() {
        let res = expand("10.1.2.3/0");
        assert_eq!("10.1.2.3", res);
    }

    #[test]
    fn it_converts_ip_string_to_binary() {
        assert_eq!("00001010000000010000001000000011", ip_to_binary("10.1.2.3".to_string()));
        assert_eq!("01111111000000000000000100000001", ip_to_binary("127.0.1.1".to_string()));
        assert_eq!("11000000101010000000000000000001", ip_to_binary("192.168.0.1".to_string()));
        assert_eq!("10101001111111101010100111111110", ip_to_binary("169.254.169.254".to_string()));
    }

    #[test]
    fn it_converts_binary_to_ip_string() {
        assert_eq!("10.1.2.3", binary_to_ip("00001010000000010000001000000011".to_string()));
        assert_eq!("127.0.1.1", binary_to_ip("01111111000000000000000100000001".to_string()));
        assert_eq!("192.168.0.1", binary_to_ip("11000000101010000000000000000001".to_string()));
        assert_eq!("169.254.169.254", binary_to_ip("10101001111111101010100111111110".to_string()));
    }
}
