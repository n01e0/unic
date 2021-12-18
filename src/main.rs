#[macro_use]
extern crate clap;
use std::io::Read;

fn main() {
    let yml = load_yaml!("cmd.yml");
    let args = clap::App::from_yaml(yml).get_matches();

    let mut v = if let Some(s) = args.value_of("content") {
         s.chars().collect::<Vec<_>>()
    } else if args.is_present("read_stdin") {
        let mut s = String::new();
        std::io::stdin().read_to_string(&mut s).expect("Can't read from stdin");
        s.chars().collect::<Vec<_>>()
    } else {
        Vec::new()
    };
    v.sort();
    v.dedup();
    for c in v {
        print!("{}", c);
    }
    println!()
}
