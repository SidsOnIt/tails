use std::collections::HashMap;

pub enum Rating {
    UnderOr13,  // 0 - 13
    Rng14_15,   // 14 - 15
    Rng16_17,   // 16 - 17
    OverOr18,   // 18+
}

pub struct Meta {
    title: String,
    subtitle: String,
    date: String,
    authorship: Vec<String>,
    tags: Vec<String>,
    rating: Rating
}

pub enum TextStyle {
    bold,
    italic,
    uline,
    link(href)
}

pub struct Text {
    style: TextStyle,
    value: String
}

pub enum AlertStyle {
    Info,
    Success,
    Warning,
    Failure
}

pub struct Alert {
    style: AlertStyle,
    value: String
}

pub enum ListStyle {
    Plain,
    Abc,
    Num,
    Bullet,
    Arrow,
}

pub struct List {
    name: String,
    style: ListStyle,
    files: Vec<String>,
    folders: Vec<Box<List>>
}

pub enum Token {
    Title(Text),
    SubTitle(Text),
    Section(Text),
    Text(Text),
    SubText(Text),
    Quote(Text),
    Alert(Alert),
}

pub struct Vars(HashMap<String, String>);

pub struct Document {
    meta:Meta,
    tokens:Vec<Token>
}


extract_variables(String) -> (HashMap, String) //using grey_matter

inject_vars(HashMap, String) -> String //using handlebars

inject_files(String) -> String
    -> Depends on [extract_variable and inject_vars]

tokenize() -> Vec<
