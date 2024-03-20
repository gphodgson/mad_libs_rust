#[derive(PartialEq, Debug)]
pub enum Tense{
    PAST,
    PRESENT,
    FUTURE,
    VOID
}

impl Tense {
   pub fn as_str(&self) -> &str {
       match self {
           Tense::PAST => "past",
           Tense::PRESENT => "present",
           Tense::FUTURE => "future",
           Tense::VOID => "!!!!"
       }
   }

    pub fn from_str(string: &str) -> Self{
        match string {
            "past" => Tense::PAST,
            "present" => Tense::PRESENT,
            "future" => Tense::FUTURE,
            "void" => Tense::VOID,
            _ => Tense::VOID
        }
    }
}