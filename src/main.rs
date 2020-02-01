extern crate rpn;
use std::io::{self, Write};

fn main() {
    println!("Reverse Polish Notation.");
    println!("Type quit to exit");
    let mut stack: Vec<OperationElt> = Vec<OperationElt>::new();

    loop {
        print!("> ");
        io::stdout().flush().expect("flushing failed");
        let mut input = String::new();

        io::stdin()
            .read_line(&mut input)
            .expect("failed to read line");

        if input.trim() == "quit" {
            break;
        }
        let result = rpn::evaluate(&mut stack, &input);
        match result {
            Err(err) => println!("Error: {}", err),
            Ok(res) => println!("{:?}", res),
        }
    }
}
