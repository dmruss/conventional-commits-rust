use crate::types::Token;
use crate::types::CommitAst;
// This function takes a commit message string and returns a vector of tokens
pub fn commit_string_parser(commit_msg: &str) -> Vec<Token> {
    let mut tokens: Vec<Token> = Vec::new();
    let mut commit_type = String::new();
    let mut scope = String::new();
    let mut breaking = false;
    let mut description = String::new();
    let mut footer = String::new();

    // Split the commit message into lines
    let lines: Vec<&str> = commit_msg.lines().collect();

    // Parse the first line for commit type, scope, and description
    if !lines.is_empty() {
        let first_line = lines[0];
        if let Some(colon_index) = first_line.find(':') {
            commit_type = first_line[..colon_index].to_string();
            description = first_line[colon_index + 1..].trim().to_string();
        } else {
            description = first_line.to_string();
        }

        // Check for breaking change indicator
        if commit_type.ends_with('!') {
            breaking = true;
            commit_type.pop(); // Remove the '!' from the commit type
        }

        // Check for scope in parentheses
        if let Some(start_index) = commit_type.find('(') {
            if let Some(end_index) = commit_type.find(')') {
                scope = commit_type[start_index + 1..end_index].to_string();
                commit_type = format!("{}{}", &commit_type[..start_index], &commit_type[end_index + 1..]);
            }
        }
    }

    // Parse the footer if it exists
    if lines.len() > 1 {
        footer = lines[1..].join("\n");
    }

    // Create tokens and add them to the vector
    tokens.push(Token::CommitType(commit_type));
    tokens.push(Token::Scope(scope));
    tokens.push(Token::Breaking(breaking));
    tokens.push(Token::Description(description));
    tokens.push(Token::Footer(footer));

    tokens
}

// This function takes a vector of tokens and returns a CommitAst struct
pub fn tokens_to_ast(tokens: Vec<Token>) -> CommitAst {
    let mut commit_type = Token::CommitType(String::new());
    let mut scope = Token::Scope(String::new());
    let mut breaking = Token::Breaking(false);
    let mut description = Token::Description(String::new());
    let mut footer = Token::Footer(String::new());
    for token in tokens {
        match token {
            Token::CommitType(t) => commit_type = Token::CommitType(t),
            Token::Scope(s) => scope = Token::Scope(s),
            Token::Breaking(b) => breaking = Token::Breaking(b),
            Token::Description(d) => description = Token::Description(d),
            Token::Footer(f) => footer = Token::Footer(f),
        }
    }
    CommitAst {
        commit_type,
        scope,
        breaking,
        description,
        footer,
    }
}   