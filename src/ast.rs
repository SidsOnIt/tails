use indexmap::IndexMap;
use std::collections::HashMap;

#[non_exhaustive]
pub enum TextStyle {
    Regular,
    Bold,
    Italic,
    ULine,
    Strike,
    Link(String)
}

pub struct TextToken{
    style: TextStyle,
    value: String
}

pub struct TextBody(Vec<TextToken>)

#[non_exhaustive]
pub enum AlertStyle {
    Info,
    Success,
    Warning,
    Failure,
}

pub struct AlertToken {
    style: AlertStyle,
    body: TextBody,
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
    branches: Vec<List>,
}

#[non_exhaustive]
pub enum LeafStyle {
    File,
    Folder,
    Note,
}

#[non_exhaustive]
pub struct Leaf {
    style: LeafStyle,
    value: String,
}

pub struct Tree {
    name: String,
    leaves: Vec<String>,
    branches: Vec<Tree>,
}

pub struct Table {
    name: String,
    columns: IndexMap<String, Vec<String>>,
}

#[non_exhaustive]
pub enum DocumentToken {
    Title(TextBody),
    SubTitle(TextBody),
    Section(TextBody),
    Text(TextBody),
    SubText(TextBody),
    Quote(TextBody),
    Alert(AlertToken),
    LinkBtn(String),
    Code(String),
    Table(Table),
    Tree(Tree),
    List(List),
    YouTubeEmbed(String),
    VideoEmbed(String),
    Image(String), //covers gifs
}


//unsure if this should be non-exhaustive or not, will have to research the current discourse around the age signal laws more
pub enum Rating {
    UnderOr13, // 0 - 13
    Rng14_15,  // 14 - 15
    Rng16_17,  // 16 - 17
    OverOr18,  // 18+
}

pub struct DocumentMeta {
    pub title: Option<String>,
    pub subtitle: Option<String>,
    pub date: Option<String>,
    pub authorship: Option<Vec<String>>,
    pub tags: Option<Vec<String>>,
    pub rating: Option<Rating>,
    pub vars: Option<HashMap<String, String>>
}

pub struct Document {
    meta: DocumentMeta,
    data: String
}

impl Document {
    pub fn from_str(raw: &str) -> Self {
        todo!() // use graymatter to deserialize front matter
    }
    pub fn interpolated(&self) -> String {
        todo!() // use handlebars to get back interpolated
    }
}
// i need to pause here and consider if i want to continue with this draft
// i need to make sure i am not overdoing it
// this is just for tails and markdown
// 
