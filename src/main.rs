//include parser.rs from this dir
mod parser;
mod types;
mod semver;
mod changelog;

// This is the main entry point for the conventional-commits crate
// It will parse the commit messages and return the next version number

fn main() {
    // get most recent tag
    let most_recent_tag = semver::get_most_recent_tag();
    println!("Most recent tag: {}", most_recent_tag);

    // get commits since most recent tag
    let commits = semver::get_commits_since_tag(&most_recent_tag);
    println!("Commits since most recent tag:");
    for commit in &commits {
        println!("{}", commit);
    }

    // parse commits into ASTs
    let mut commit_asts: Vec<types::CommitAst> = Vec::new();
    for commit in &commits {
        let tokens = parser::commit_string_parser(commit);
        let ast = parser::tokens_to_ast(tokens);
        commit_asts.push(ast);
    }

    // print ASTs
    for ast in &commit_asts {
        ast.print_ast();
    }

    // parse most recent tag into Version struct
    let mut version = semver::parse_version(&most_recent_tag);
    println!("Current version: {}.{}.{}", version.major, version.minor, version.patch);

    // increment version based on commits
    version.increment(commit_asts);
    println!("Next version: {}", version.to_string());

    // generate changelog from commits and version
    let changelog = changelog::generate_changelog(commits, &version.to_string());
    println!("Changelog:\n{}", changelog);
    changelog::output_changelog(&changelog);

    // create git tag for next version
    let tag_name = format!("v{}", version.to_string());
    semver::create_git_tag(&version);
}
