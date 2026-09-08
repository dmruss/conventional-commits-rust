use crate::types::Version;
use crate::types::CommitAst;
use crate::types::Token;
// Semantic Versioning functions
// get most recent git tag version, if none, return 0.0.0
pub fn get_most_recent_tag() -> String {
    let output = std::process::Command::new("git")
        .args(&["describe", "--tags", "--abbrev=0"])
        .output()
        .expect("Failed to execute git command");
    if output.status.success() {
        let tag = String::from_utf8_lossy(&output.stdout);
        tag.trim().to_string()
    } else {
        "0.0.0".to_string()
    }
}

// get all commits since most recent tag
pub fn get_commits_since_tag(tag: &str) -> Vec<String> {
    let output = std::process::Command::new("git")
        .args(&["log", "--pretty=format:%s", &format!("{}..HEAD", tag)])
        .output()
        .expect("Failed to execute git command");
    if output.status.success() {
        let commits = String::from_utf8_lossy(&output.stdout);
        commits.lines().map(|s| s.to_string()).collect()
    } else {
        Vec::new()
    }
}

// parse version string into Version struct
pub fn parse_version(version_str: &str) -> Version {
    let parts: Vec<&str> = version_str.split('.').collect();
    let major = parts.get(0).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    let minor = parts.get(1).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    let patch = parts.get(2).and_then(|s| s.parse::<u32>().ok()).unwrap_or(0);
    Version { major, minor, patch }
}   

// increment version based on commit messages, implement on Version struct
impl Version {
    pub fn increment(&mut self, commits: Vec<CommitAst>) {
        let mut major_increment = false;
        let mut minor_increment = false;
        let mut patch_increment = false;

        for commit in commits {
            match commit.commit_type {
                Token::CommitType(ref t) if t == "feat" => minor_increment = true,
                Token::CommitType(ref t) if t == "fix" => patch_increment = true,
                Token::Breaking(true) => major_increment = true,
                _ => {}
            }
        }

        if major_increment {
            self.major += 1;
            self.minor = 0;
            self.patch = 0;
        } else if minor_increment {
            self.minor += 1;
            self.patch = 0;
        } else if patch_increment {
            self.patch += 1;
        }
    }

    pub fn to_string(&self) -> String {
        format!("{}.{}.{}", self.major, self.minor, self.patch)
    }
}

// take in next version and create a git tag for it
pub fn create_git_tag(version: &Version) {
    let tag_name = format!("v{}", version.to_string());
    let output = std::process::Command::new("git")
        .args(&["tag", &tag_name])
        .output()
        .expect("Failed to execute git command");
    if !output.status.success() {
        eprintln!("Failed to create git tag: {}", String::from_utf8_lossy(&output.stderr));
    }
}