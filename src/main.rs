//include parser.rs from this dir
mod parser;
mod types;

fn main() {
    //take in a commit message string, run through parser, and print out the resulting tokens
    let commit_msg = "feat(parser): add new parser for commit messages\n\nBREAKING CHANGE: The old parser is no longer supported\n\nThis commit adds a new parser for commit messages that supports the conventional commits specification. The old parser is no longer supported and will be removed in a future release.";
    let tokens = parser::commit_string_parser(commit_msg);
    for token in tokens {
        println!("{:?}", token);
    }

}
