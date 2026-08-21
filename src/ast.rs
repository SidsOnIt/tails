use std::collections::HashMap;
use indexmap::IndexMap;

//unsure if this should be non-exhaustive or not, will have to research the current discourse around the age signal laws more
pub enum Rating {
    UnderOr13,  // 0 - 13
    Rng14_15,   // 14 - 15
    Rng16_17,   // 16 - 17
    OverOr18,   // 18+
}

#[non_exhaustive]
pub struct Meta {
    title: String,
    subtitle: String,
    date: String,
    authorship: Vec<String>,
    tags: Vec<String>,
    rating: Rating,
}

#[non_exhaustive]
pub enum TextStyle {
    Bold,
    Italic,
    ULine,
    Strike,
    Link(String),
}

pub struct Text {
    style: TextStyle,
    value: String,
}

pub struct TextSpan(Vec<Text>);

#[non_exhaustive]
pub enum AlertStyle {
    Info,
    Success,
    Warning,
    Failure,
}

pub struct Alert {
    style: AlertStyle,
    value: String,
}

#[non_exhaustive]
pub enum ListStyle {
    Plain,
    Abc,
    Num,
    Bullet,
    Arrow,
    Task,
}

pub struct List {
    name: String,
    style: ListStyle,
    leaves: Vec<String>,
    branches: Vec<Box<List>>,
}

#[non_exhaustive]
pub enum TreeStyle {
    File,
    Folder,
}

pub struct Tree {
    name: String,
    style: TreeStyle,
    leaves: Vec<String>,
    branches: Vec<Box<Tree>>,
}

pub struct Table {
    name: String,
    columns: IndexMap<String, Vec<String>>,
}

#[non_exhaustive]
pub enum Token {
    Title(TextSpan),
    SubTitle(TextSpan),
    Section(TextSpan),
    Text(TextSpan),
    SubText(TextSpan),
    Quote(TextSpan),
    Alert(Alert),
    LinkBtn(String),
    Code(String),
    Table(Table),
    Tree(Tree),
    List(List),
    YouTubeEmbed(String),
    VideoEmbed(String),
    Image(String), //also covers gifs
    //injects are not going to be tokenized, they are going to be pre-agregated pre-tokenization
}

pub struct Document {
    meta:Meta,
    tokens:Vec<Token>
}

pub struct Vars(HashMap<String, String>);
