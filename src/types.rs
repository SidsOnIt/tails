use indexmap::IndexMap;

#[non_exhaustive]
pub enum TextStyle {
    Regular,
    Bold,
    Italic,
    ULine,
    Strike,
    Link(String),
}

pub struct TextToken {
    pub style: TextStyle,
    pub value: String,
}

pub struct TextBody(pub Vec<TextToken>);

#[non_exhaustive]
pub enum AlertStyle {
    Info,
    Success,
    Warning,
    Failure,
}

pub struct AlertToken {
    pub style: AlertStyle,
    pub body: TextBody,
}

#[non_exhaustive]
pub enum ListStyle {
    Plain,
    Abc,
    Num,
    Bullet,
    Arrow,
    Task,
    Other,
}

pub struct List {
    pub name: String,
    pub style: ListStyle,
    pub leaves: Vec<String>,
    pub branches: Vec<List>,
}

#[non_exhaustive]
pub enum LeafStyle {
    File,
    Folder,
    Note,
}

pub struct Leaf {
    pub style: LeafStyle,
    pub value: String,
}

pub struct Tree {
    pub name: String,
    pub leaves: Vec<String>,
    pub branches: Vec<Tree>,
}

pub struct Table {
    pub name: String,
    pub columns: IndexMap<String, Vec<String>>,
}

pub struct ExpandableBlock {
    pub title: String,
    pub content: String,
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
    Expandable(ExpandableBlock),
    YouTubeEmbed(String),
    VideoEmbed(String),
    Image(String),
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub enum Rating {
    UnderOr13,
    Rng14_15,
    Rng16_17,
    OverOr18,
}

impl Rating {
    pub fn parse(s: &str) -> Option<Self> {
        match s.trim().to_uppercase().as_str() {
            "UNDEROR13" | "13-" | "G" | "PG" | "PG-13" | "PG13" => Some(Rating::UnderOr13),
            "RNG14_15" | "14-15" | "14_15" | "M" | "MA15+" => Some(Rating::Rng14_15),
            "RNG16_17" | "16-17" | "16_17" | "R" | "16+" => Some(Rating::Rng16_17),
            "OVEROR18" | "18+" | "NC-17" | "NC17" | "ADULT" => Some(Rating::OverOr18),
            _ => None,
        }
    }
}
