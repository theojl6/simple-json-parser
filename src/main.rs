use ::simple_json_parser::{run_file, run_prompt};
use std::env;
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() == 1 {
        run_prompt();
    } else {
        run_file("tests/step1/valid.json");
    }
}
