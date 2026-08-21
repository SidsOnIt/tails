// this is going to be the most unstable api starting out, since the end intent is to distill all primitive variants into their meaning
// as a better fitting word is found the api will change until 1.0.
//
pub enum Intents {
    Type, //class, struct, enum
    Var,
    Math, // + - / * % =
    Condition, // while if for each
    String,
    Num,
    Verbs, //SELECT, INSERT, UPDATE, DELETE, CREATE, ALTER, DROP, TRUNCATE, COMMIT, ROLLBACK, GET, POST, PUT, PATCH, DELETE, HEAD, OPTIONS
    CallBracket, //( ),
    ArrayBracket, // [ ]
    MapBracket, // { }
    Terminator, // : , ; . :: << >>

}

#[derive(Logos, Clone, Debug, PartialEq)]
pub enum CodeToken {
    // Explicitly match whitespace/indentation to preserve spacing
    #[regex(r"[ \t]+")]
    Whitespace,

    // Explicitly match newlines to preserve line breaks
    #[regex(r"\r?\n")]
    Newline,

    #[token("select", ignore(case))]
    #[token("insert", ignore(case))]
    #[token("update", ignore(case))]
    #[token("delete", ignore(case))]
    #[token("create", ignore(case))]
    #[token("alter", ignore(case))]
    #[token("drop", ignore(case))]
    #[token("trunicate", ignore(case))]
    #[token("commit", ignore(case))]
    #[token("rollback", ignore(case))]
    #[token("get", ignore(case))]
    #[token("post", ignore(case))]
    #[token("put", ignore(case))]
    #[token("patch", ignore(case))]
    #[token("head", ignore(case))]
    Verb,

    #[token("let")]
    #[token("fn")]
    #[token("return")]
    #[token("mut")]
    #[token("pub")]
    Keyword,

    #[regex("[a-zA-Z_][a-zA-Z0-9_]*")]
    Identifier,

    #[regex("[0-9]+")]
    Number,

    #[token("=")]
    #[token("+")]
    #[token("-")]
    #[token("*")]
    #[token("/")]

    #[token("==")]
    #[token("!=")]
    #[token(">")]
    #[token("<")]
    #[token(">=")]
    #[token("<=")]
    Comparitor,

    #[token("|")]
    #[token("&")]
    #[token("||")]
    #[token("&&")]
    Operator,

    #[token(":")]
    #[token("=>")]
    #[token("=")]
    #[token("as")]
    #[token("alias")]
    Assignment,

    #[token(".")]
    #[token("::")]
    #[token("<<")]
    #[token(">>")]
    #[token("(")]
    #[token(")")]
    Call,

    #[token("}")]
    #[token("{")]
    Map,

    #[token("[")]
    #[token("]")]
    #[token(",")]
    Array
}

//!!! implement an as alias, so if someone uses a file type that is not included that is similar to another lang they can do:
/// ~ code.surreal.SQL:, likewise overtime some of these may become auto aliased if the matching is solid.
