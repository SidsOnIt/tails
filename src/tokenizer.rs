use logos::Logos;

#[derive(Logos, Debug, PartialEq)]
#[logos(skip r"[ \t\n\f]+")]
pub enum Token<'a> {
    // Captures everything between 'title:' and ';;' directly into a slice
    #[regex(r"title:[ \t]*([^;]+);;", |lex| lex.slice())]
    Title(&'a str),

    #[regex(r"subtitle:[ \t]*([^;]+);;", |lex| lex.slice())]
    SubTitle(&'a str),

    #[regex(r"section:[ \t]*([^;]+);;", |lex| lex.slice())]
    Section(&'a str),

    #[regex(r"subtext:[ \t]*([^;]+);;", |lex| lex.slice())]
    SubText(&'a str),

    #[regex(r"quote:[ \t]*([^;]+);;", |lex| lex.slice())]
    Quote(&'a str),

    #[regex(r"alert\.info:[ \t]*([^;]+);;", |lex| lex.slice())]
    AlertInfo(&'a str),

    // Code blocks capturing multi-line content up to ';;'
    #[regex(r"code\.rust:[ \t]*([\s\S]*?);;", |lex| lex.slice())]
    CodeRust(&'a str),

    // Standalone dividers
    #[token("line.s")]
    LineSmall,

    #[token("line.m")]
    LineMedium,

    #[token("line.l")]
    LineLarge,
}
