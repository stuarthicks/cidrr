pub static VERSION: &'static str = "cidrr version 0.1.0 by Stuart Hicks \
                                    (https://github.com/stuarthicks/cidrr)";

pub static USAGE: &'static str = "
Usage: cidrr <cidr>
       cidrr (-h | --help)
       cidrr (-v | --version)
";

#[derive(Debug, RustcDecodable)]
pub struct Args {
    pub arg_cidr: String,
    pub flag_h: bool,
    pub flag_help: bool,
    pub flag_v: bool,
    pub flag_version: bool,
}
