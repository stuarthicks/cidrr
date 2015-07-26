# Cidrr [![Build Status](https://travis-ci.org/stuarthicks/cidrr.svg)](https://travis-ci.org/stuarthicks/cidrr) [![Coverage Status](https://coveralls.io/repos/stuarthicks/cidrr/badge.svg?branch=master&service=github)](https://coveralls.io/github/stuarthicks/cidrr?branch=master)
---

CLI utility for converting between CIDR blocks (eg, 10.0.0.0/8) and lists of IP address covered by that block. Might be useful to someone (maybe even myself), but exists mainly as a project for learning the basics of rust.

````
Usage: cidrr <CIDR>
       cidrr (-h | --help)
       cidrr (-v | --version)
````

Example:
````
$ ./target/debug/cidrr 192.168.1.0/24
192.168.1.0
192.168.1.1
192.168.1.2
192.168.1.3
192.168.1.4
192.168.1.5
192.168.1.6
192.168.1.7
192.168.1.8
192.168.1.9
...
[truncated output]
````

## Dependencies
* Rust ([1.1.0 Stable](http://www.rust-lang.org/install.html)).
* Docopt ([crates.io](https://crates.io/crates/docopt))

## TODO
* Take list of IPs on stdin and calculate minimum set of cidr blocks to cover all IPs.
* Writer nicer looking rust.
* Figure out how to test docopt parsing.

## License

[Apache 2.0](http://www.apache.org/licenses/LICENSE-2.0)
