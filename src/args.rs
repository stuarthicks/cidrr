use docopt;
use std::process::exit;

static VERSION: &'static str =
    "cidrr version 0.1.0 by Stuart Hicks (https://github.com/stuarthicks/cidrr)";

static USAGE: &'static str = "
Usage: cidrr <CIDR>
       cidrr (-h | --help)
       cidrr (-v | --version)
";

#[derive(Debug)]
pub struct Arguments {
    pub base_ip: String,
    pub fixed_bits: u8
}

impl Arguments {

    fn from_docopt(args : docopt::ArgvMap) -> Arguments {
        let cidr: String = args.get_str(&"CIDR").to_string();
        let cidr_parts = cidr.split('/');
        let ip: String = cidr_parts.clone().nth(0).unwrap().to_string();
        let fixed_bits: &str = cidr_parts.clone().nth(1).unwrap();
        let parsed_fixed_bits: u8 = u8::from_str_radix(fixed_bits, 10).unwrap();
        return Arguments {
            base_ip: ip,
            fixed_bits: parsed_fixed_bits
        }
    }

    pub fn parse<I, S>(argv : I) -> Arguments where I: Iterator<Item=S>, S: Into<String> {
        let args = docopt::Docopt::new(USAGE).unwrap().argv(argv).parse().unwrap_or_else(|e| e.exit());

        if args.get_bool(&"--version") || args.get_bool(&"-v") {
            println!("{}", &VERSION);
            exit(0);
        }

        if args.get_bool(&"--help") || args.get_bool(&"-h") {
            println!("{}", &USAGE);
            exit(0);
        }

        return Arguments::from_docopt(args);
    }

}
