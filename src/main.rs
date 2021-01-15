mod arg_parsing;

fn main() {
    let prog_args = arg_parsing::parse_prog_args();

    println!("Hello, world!");
}
