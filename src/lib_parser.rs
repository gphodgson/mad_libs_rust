use crate::word::Word;

pub struct LibParser{
    pub input:String,
    pub words:Vec<Word>
}

impl LibParser {
    pub fn new(input:String)->Self{
        LibParser{
            input,
            words: Vec::new(),
        }
    }

    pub fn get_output(&self)-> String{
        let mut output:String = self.input.clone();

        for word in &self.words{
            output = output.replacen(&*word.get_code(), &*word.content, 1);
        }

        output
    }

    pub fn ask(&mut self){
        for word in &mut self.words{
            word.ask();
        }
    }

    pub fn parse(&mut self){
        //iterate over input
        let bytes = self.input.as_bytes();

        let mut in_code:bool = false;
        let mut code_start: usize = 0;

        for(i, &character) in bytes.iter().enumerate(){
            if in_code != true{
                if character == b'{' {
                    in_code = true;
                    code_start = i;
                }
            }else{
                if character == b'{' {
                    code_start = i;
                } else if character == b'}'{
                    in_code = false;
                    let code:&str = &self.input[code_start..i];
                    self.words.push(Word::from_code(code));
                }
            }

        }
    }
}

#[cfg(test)]
mod tests{
    use crate::lexical::Lexical;
    use crate::lib_parser::LibParser;
    use crate::tense::Tense;
    use crate::word::Word;

    #[test]
    fn test_parse_happy_path(){
        let mut sut:LibParser = LibParser::new(String::from("{noun} {noun} {verb|past} {adjective} {adverb}"));
        sut.parse();

        let mut expected:Vec<Word> = Vec::new();
        expected.push(Word::new(Lexical::NOUN, Tense::VOID));
        expected.push(Word::new(Lexical::NOUN, Tense::VOID));
        expected.push(Word::new(Lexical::VERB, Tense::PAST));
        expected.push(Word::new(Lexical::ADJECTIVE, Tense::VOID));
        expected.push(Word::new(Lexical::ADVERB, Tense::VOID));

        assert_eq!(expected, sut.words);
    }
}