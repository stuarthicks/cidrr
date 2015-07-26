// vi: sw=4 ts=4
extern crate docopt;

use std::env;

mod args;
use args::Arguments;

#[allow(dead_code)]
fn main() {
    let cidr_args = Arguments::parse(env::args());
    let expanded: Vec<String> = all_with_prefix(cidr_args.base_ip, cidr_args.fixed_bits);
    for ip in expanded.iter() {
        println!["{}", ip];
    }
}

/// Given an ip and a number of fixed bits, calculate possible ip addresses.
#[allow(unused_variables)]
pub fn all_with_prefix(base: String, fixed: u8) -> Vec<String> {
    let fixed_prefix: String = ip_to_binary(base.clone()).chars().take(fixed as usize).collect();
    let range_to_calculate: usize = 32 - fixed_prefix.len();
    if range_to_calculate == 0 {
        return vec![base.clone()];
    }
    let mut max: Vec<String> = Vec::with_capacity(range_to_calculate);
    for i in 0..range_to_calculate {
        max.push("1".to_string());
    }
    let max_num = u32::from_str_radix(&(max.clone().connect("")), 2).unwrap();
    let mut set: Vec<String> = Vec::new();
    for i in 0..max_num {
        let suffix: String = format!["{:b}", i as u32];
        let this_ip: String = fixed_prefix.clone() + &suffix;
        set.push(binary_to_ip(this_ip));
    }
    return set;
}

/// Convert a conventional string representation of an ip address to it's binary representation.
pub fn ip_to_binary(ip: String) -> String {
    let mut binary: Vec<String> = Vec::new();
    for i in ip.split('.') {
        let encoded: u8 = u8::from_str_radix(i, 10).unwrap();
        binary.push(format!["{:08b}", encoded]);
    }
    return binary.connect("");
}

/// Convert a binary representation of an ip address to conventional human-readable one.
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
    fn it_converts_ip_string_to_binary() {
        assert_eq!["00001010000000010000001000000011", ip_to_binary("10.1.2.3".to_string())];
        assert_eq!["01111111000000000000000100000001", ip_to_binary("127.0.1.1".to_string())];
        assert_eq!["11000000101010000000000000000001", ip_to_binary("192.168.0.1".to_string())];
        assert_eq!["10101001111111101010100111111110", ip_to_binary("169.254.169.254".to_string())];
    }

    #[test]
    fn it_converts_binary_to_ip_string() {
        assert_eq!["10.1.2.3", binary_to_ip("00001010000000010000001000000011".to_string())];
        assert_eq!["127.0.1.1", binary_to_ip("01111111000000000000000100000001".to_string())];
        assert_eq!["192.168.0.1", binary_to_ip("11000000101010000000000000000001".to_string())];
        assert_eq!["169.254.169.254", binary_to_ip("10101001111111101010100111111110".to_string())];
    }

    #[test]
    fn it_lists_same_ip_if_fixed_prefix_is_32() {
        let expected: Vec<String> = vec!["1.2.3.4".to_string()];
        assert_eq![expected, all_with_prefix("1.2.3.4".to_string(), 32 as u8)]
    }

    #[test]
    fn it_lists_some_ip_addresses() {
        let expected: Vec<String> = vec![
            "192.168.0.0".to_string(),
            "192.168.0.1".to_string(),
            "192.168.0.2".to_string()
        ];
        assert_eq![expected, all_with_prefix("192.168.0.0".to_string(), 30 as u8)]
    }
}
