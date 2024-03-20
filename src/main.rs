mod word;
mod lexical;
mod tense;
mod lib_parser;

use crate::lib_parser::LibParser;

fn main() {
    let mut parser = LibParser::new(String::from("once upon a {noun}, there was a {noun} that liked to {verb|present} {adverb}. Weirdly, he was {adjective} when he {verb|past}."));

    parser.parse();
    parser.ask();

    println!("{}", parser.get_output());
}
