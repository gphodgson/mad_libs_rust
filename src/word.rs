use std::io;
use crate::lexical::Lexical;
use crate::tense::Tense;

#[derive(PartialEq, Debug)]
pub struct Word{
    pub content: String,
    pub lexical_category: Lexical,
    pub tense: Tense
}

impl Word {
    pub fn new(lexical_category: Lexical, tense: Tense) -> Self{
        Self{
            content: String::from(""),
            lexical_category,
            tense
        }
    }

    /*
        Example codes: `{noun}`, `{verb|present}`
    */
    pub fn from_code(code: &str) -> Self{
        let code = code.replace("{", "");
        let code = code.replace("}", "");

        let elements = code.split("|");
        let collection: Vec<&str> = elements.collect();

        let lexical_str = collection[0];
        let mut tense_str = "void";

        if collection.len() > 1{
            tense_str = collection[1];
        }

        Self::new(Lexical::from_str(lexical_str), Tense::from_str(tense_str))
    }

    pub fn get_code(&self)-> String{
        if self.tense != Tense::VOID{
            return String::from(format!("{{{}|{}}}", self.lexical_category.as_str(), self.tense.as_str()));
        }

         String::from(format!("{{{}}}", self.lexical_category.as_str()))
    }

    fn ask_message(&self) -> String{
        if self.tense != Tense::VOID{
            return format!("please give a {}({} tense)", self.lexical_category.as_str(), self.tense.as_str());
        }

        format!("please give a {}", self.lexical_category.as_str())
    }

    /*
        Ask for the word. take input and assign it to the `content` field.
     */
    pub fn ask(&mut self){
        println!("{}", self.ask_message());
        io::stdin().read_line(&mut self.content).expect("failed to read line");

        //remove end line
        self.content = self.content.replace("\n", "");
    }
}

#[cfg(test)]
mod tests{
    use crate::word::{Lexical, Tense, Word};
    #[test]
    fn test_new(){
        let actual = Word::new(Lexical::VERB, Tense::PAST);
        let expected = Word{
            content: String::from(""),
            lexical_category: Lexical::VERB,
            tense: Tense::PAST
        };

        assert_eq!(actual, expected);
    }
    #[test]
    fn test_from_code_verb_past(){
        let actual = Word::from_code("{verb|past}");
        let expected = Word::new(Lexical::VERB, Tense::PAST);

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_from_code_verb_present(){
        let actual = Word::from_code("{verb|present}");
        let expected = Word::new(Lexical::VERB, Tense::PRESENT);

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_from_code_verb_future(){
        let actual = Word::from_code("{verb|future}");
        let expected = Word::new(Lexical::VERB, Tense::FUTURE);

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_from_code_noun(){
        let actual = Word::from_code("{noun}");
        let expected = Word::new(Lexical::NOUN, Tense::VOID);

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_from_code_adjective(){
        let actual = Word::from_code("{adjective}");
        let expected = Word::new(Lexical::ADJECTIVE, Tense::VOID);

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_from_code_adverb(){
        let actual = Word::from_code("{adverb}");
        let expected = Word::new(Lexical::ADVERB, Tense::VOID);

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_ask_message_noun(){
        let sut = Word::new(Lexical::NOUN, Tense::VOID);
        let actual = sut.ask_message();
        let expected = String::from("please give a noun");

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_ask_message_verb_past(){
        let sut = Word::new(Lexical::VERB, Tense::PAST);
        let actual = sut.ask_message();
        let expected = String::from("please give a verb(past tense)");

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_ask_message_verb_present(){
        let sut = Word::new(Lexical::VERB, Tense::PRESENT);
        let actual = sut.ask_message();
        let expected = String::from("please give a verb(present tense)");

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_ask_message_adverb(){
        let sut = Word::new(Lexical::ADVERB, Tense::VOID);
        let actual = sut.ask_message();
        let expected = String::from("please give a adverb");

        assert_eq!(actual, expected)
    }
    #[test]
    fn test_get_code_noun(){
        let sut = Word::new(Lexical::NOUN, Tense::VOID);
        let expected = String::from("{noun}");

        assert_eq!(expected, sut.get_code());
    }
    #[test]
    fn test_get_code_verb(){
        let sut = Word::new(Lexical::VERB, Tense::PRESENT);
        let expected = String::from("{verb|present}");

        assert_eq!(expected, sut.get_code());
    }
}