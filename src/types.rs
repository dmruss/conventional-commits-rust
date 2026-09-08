#[derive(Debug)]
pub enum Token {
    CommitType(String),
    Scope(String),
    Breaking(bool),
    Description(String),
    Footer(String)
}

pub(crate) struct CommitAst {
    pub commit_type: Token,
    pub scope: Token,
    pub breaking: Token,
    pub description: Token,
    pub footer: Token
}

pub struct Version {
    pub major: u32,
    pub minor: u32,
    pub patch: u32
}

impl CommitAst {
    pub fn print_ast(&self) {
        println!("Commit Type: {:?}", self.commit_type);
        println!("Scope: {:?}", self.scope);
        println!("Breaking: {:?}", self.breaking);
        println!("Description: {:?}", self.description);
        println!("Footer: {:?}", self.footer);
    }
}