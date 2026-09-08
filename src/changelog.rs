// Changelog generation functions
use std::io::Write;
// generate changelog from commits and version
pub fn generate_changelog(commits: Vec<String>, version: &str) -> String {
    let mut changelog = String::new();
    changelog.push_str(&format!("## {}\n\n", version));
    for commit in commits {
        changelog.push_str(&format!("- {}\n", commit));
    }
    changelog
}   

// output changelog to file
pub fn output_changelog(changelog: &str) {
    let file = std::fs::File::create("CHANGELOG.md").unwrap();
    let mut writer = std::io::BufWriter::new(file);
    writer.write_all(changelog.as_bytes()).unwrap();
}   