mod parser_utils;
use parser_utils::Parser;

fn parse_arithmetic(s: String) {
    println!("{}", s);
    let mut p = Parser::new(&s);
    let tree = p.start();
    if let Some(node) = tree {
        println!("{}\n", node.borrow().to_string());
    } else {
        println!("Failed to parse the input");
    }
}

fn main() {
    parse_arithmetic(String::from("!A -> B | A & C"));
    parse_arithmetic(String::from("!A -> (B | A) & C"));
}
