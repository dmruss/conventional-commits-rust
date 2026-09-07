#[derive(Debug)]
pub enum Token {
    CommitType(String),
    Scope(String),
    Breaking(bool),
    Description(String),
    Footer(String)
}

struct CommitAst {
    commit_type: Token,
    scope: Token,
    breaking: Token,
    description: Token,
    footer: Token
}

struct Version {
    major: u32,
    minor: u32,
    patch: u32
}