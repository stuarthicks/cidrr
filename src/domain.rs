use args;
use args::Args;

pub struct Cidr {
    pub base_ip: String,
    pub fixed_bits: u8
}

impl Cidr {
    pub fn from_args(args : args::Args) -> Cidr {
        let cidr: String = args.cidr;
        let cidr_parts = cidr.split('/');
        let ip: String = cidr_parts.clone().nth(0).unwrap().to_string();
        let fixed_bits: &str = cidr_parts.clone().nth(1).unwrap();
        let parsed_fixed_bits: u8 = u8::from_str_radix(fixed_bits, 10).unwrap();
        return Cidr {
            base_ip: ip,
            fixed_bits: parsed_fixed_bits
        }
    }

}