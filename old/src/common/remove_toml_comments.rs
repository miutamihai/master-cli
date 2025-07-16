use regex::Regex;

pub fn remove_toml_comments(toml: String) -> String {
    let comment_re = Regex::new(r"^\s*#.*$").unwrap();
    let cleaned: Vec<_> = toml
        .lines()
        .filter(|line| !comment_re.is_match(line))
        .collect();
    cleaned.join("\n")
}
