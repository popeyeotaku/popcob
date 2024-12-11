pub static KW_TAB:[&str; 13] = [
    "PROCEDURE",
    "DIVISION",
    "DISPLAY",
    "UPON",
    "DATA",
    "ENVIRONMENT",
    "CONFIGURATION",
    "SECTION",
    "SOURCE-COMPUTER",
    "OBJECT-COMPUTER",
    "SPECIAL",
    "NAMES",
    "IS",
];

#[derive(PartialEq,Clone,Copy,Debug)]
pub enum Kw {
    Procedure,
    Division,
    Display,
    Upon,
    Data,
    Environment,
    Configuration,
    Section,
    SourceComputer,
    ObjectComputer,
    Special,
    Names,
    Is,
}

impl Kw {
    pub fn findkw(s:&str) -> Option<Self> {
        match s {
			"PROCEDURE" => Some(Self::Procedure),
			"DIVISION" => Some(Self::Division),
			"DISPLAY" => Some(Self::Display),
			"UPON" => Some(Self::Upon),
			"DATA" => Some(Self::Data),
			"ENVIRONMENT" => Some(Self::Environment),
			"CONFIGURATION" => Some(Self::Configuration),
			"SECTION" => Some(Self::Section),
			"SOURCE-COMPUTER" => Some(Self::SourceComputer),
			"OBJECT-COMPUTER" => Some(Self::ObjectComputer),
			"SPECIAL" => Some(Self::Special),
			"NAMES" => Some(Self::Names),
			"IS" => Some(Self::Is),

            _ => None
        }
    }
}