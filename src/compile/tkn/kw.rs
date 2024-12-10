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

#[derive(PartialEq,Clone,Copy)]
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
