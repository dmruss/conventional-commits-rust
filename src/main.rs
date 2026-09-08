//include parser.rs from this dir
mod parser;
mod types;

fn main() {
    //take in a commit message string, run through parser, and print out the resulting tokens
    let commit_msg = "feat(parser): add new parser for commit messages";
    let tokens = parser::commit_string_parser(commit_msg);
    let ast = parser::tokens_to_ast(tokens);
    ast.print_ast();
    // for token in tokens {
    //     println!("{:?}", token);
    // }

}
