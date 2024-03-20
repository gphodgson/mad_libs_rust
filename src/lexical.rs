#[derive(PartialEq, Debug)]
pub enum Lexical{
    VERB,
    ADVERB,
    NOUN,
    ADJECTIVE
}

impl Lexical {
    pub fn as_str(&self)-> &str{
        match self {
            Lexical::VERB => "verb",
            Lexical::ADVERB => "adverb",
            Lexical::NOUN => "noun",
            Lexical::ADJECTIVE => "adjective"
        }
    }

    pub fn from_str(string: &str) -> Self{
        match string {
            "verb" => Lexical::VERB,
            "adverb" => Lexical::ADVERB,
            "noun" => Lexical::NOUN,
            "adjective" => Lexical::ADJECTIVE,
            _ => Lexical::NOUN
        }
    }
}