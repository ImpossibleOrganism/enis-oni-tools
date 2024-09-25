use std::env;

use oni_tools;

fn main() {
    let args: Vec<String> = env::args().collect();

    // TODO: Handle incorrect number of arguments.
    oni_tools::read(args[1].clone());
}
