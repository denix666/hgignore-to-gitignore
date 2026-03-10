use regex::Regex;

pub struct HgIgnoreConverter;

impl HgIgnoreConverter {
    pub fn new() -> Self {
        HgIgnoreConverter
    }

    /// Convert .hgignore format to .gitignore format
    pub fn convert(&self, content: &str) -> Result<String, String> {
        let mut gitignore_lines = Vec::new();
        let mut current_mode = "glob"; // hgignore default is glob

        for line in content.lines() {
            let trimmed = line.trim();

            // Skip empty lines and comments (but preserve them)
            if trimmed.is_empty() {
                gitignore_lines.push(String::new());
                continue;
            }

            // Handle syntax directives
            if trimmed.starts_with("syntax:") {
                let syntax = trimmed.strip_prefix("syntax:").unwrap().trim();
                current_mode = match syntax {
                    "glob" => "glob",
                    "regexp" => "regexp",
                    "relre" => "regexp",
                    _ => {
                        return Err(format!("Unknown syntax: {}", syntax));
                    }
                };
                // Don't output syntax line in gitignore
                continue;
            }

            // Skip lines starting with # (comments)
            if trimmed.starts_with('#') {
                gitignore_lines.push(line.to_string());
                continue;
            }

            // Convert the pattern based on current mode
            let converted = match current_mode {
                "glob" => self.convert_glob_pattern(trimmed),
                "regexp" => self.convert_regexp_pattern(trimmed)?,
                _ => trimmed.to_string(),
            };

            if !converted.is_empty() {
                gitignore_lines.push(converted);
            }
        }

        Ok(gitignore_lines.join("\n"))
    }

    /// Convert glob patterns from hgignore to gitignore
    fn convert_glob_pattern(&self, pattern: &str) -> String {
        if pattern.is_empty() {
            return String::new();
        }

        // Handle patterns that start with re: (regexp inline)
        if pattern.starts_with("re:") {
            if let Ok(regexp_pattern) = self.convert_regexp_pattern(&pattern[3..]) {
                return regexp_pattern;
            }
            return pattern.to_string();
        }

        // Handle patterns that start with glob: (explicit glob)
        if pattern.starts_with("glob:") {
            return pattern[5..].to_string();
        }

        // hgignore glob patterns are similar to gitignore, no conversion needed
        pattern.to_string()
    }

    /// Convert regexp patterns from hgignore to gitignore
    fn convert_regexp_pattern(&self, pattern: &str) -> Result<String, String> {
        if pattern.is_empty() {
            return Ok(String::new());
        }

        // Remove leading ^ and trailing $ if present (hgignore uses them)
        let mut clean_pattern = pattern.to_string();
        if clean_pattern.starts_with('^') {
            clean_pattern = clean_pattern[1..].to_string();
        }
        if clean_pattern.ends_with('$') {
            clean_pattern = clean_pattern[..clean_pattern.len() - 1].to_string();
        }

        // Basic validation that it's a valid regex
        if Regex::new(&clean_pattern).is_err() {
            return Err(format!("Invalid regex pattern: {}", pattern));
        }

        // For now, return the cleaned pattern
        // Note: Some regexp patterns might not work perfectly in gitignore
        // but gitignore with --global-no-re flag doesn't support regex
        // So we convert what we can
        Ok(clean_pattern)
    }
}

impl Default for HgIgnoreConverter {
    fn default() -> Self {
        Self::new()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simple_glob_pattern() {
        let converter = HgIgnoreConverter::new();
        let result = converter.convert("*.o").unwrap();
        assert_eq!(result, "*.o");
    }

    #[test]
    fn test_glob_pattern_with_directory() {
        let converter = HgIgnoreConverter::new();
        let result = converter.convert("build/").unwrap();
        assert_eq!(result, "build/");
    }

    #[test]
    fn test_syntax_directive() {
        let converter = HgIgnoreConverter::new();
        let content = "syntax: glob\n*.o\n*.pyc";
        let result = converter.convert(content).unwrap();
        assert!(!result.contains("syntax:"));
        assert!(result.contains("*.o"));
        assert!(result.contains("*.pyc"));
    }

    #[test]
    fn test_comments_preserved() {
        let converter = HgIgnoreConverter::new();
        let content = "# This is a comment\n*.o";
        let result = converter.convert(content).unwrap();
        assert!(result.contains("# This is a comment"));
    }

    #[test]
    fn test_empty_lines_preserved() {
        let converter = HgIgnoreConverter::new();
        let content = "*.o\n\n*.pyc";
        let result = converter.convert(content).unwrap();
        let lines: Vec<&str> = result.lines().collect();
        assert_eq!(lines.len(), 3);
        assert_eq!(lines[1], "");
    }

    #[test]
    fn test_glob_prefix() {
        let converter = HgIgnoreConverter::new();
        let result = converter.convert("glob:*.o").unwrap();
        assert_eq!(result, "*.o");
    }

    #[test]
    fn test_regexp_with_anchors() {
        let converter = HgIgnoreConverter::new();
        let result = converter.convert("re:^build/.*").unwrap();
        assert_eq!(result, "build/.*");
    }

    #[test]
    fn test_missing_hgignore() {
        let converter = HgIgnoreConverter::new();
        let result = converter.convert("");
        assert!(result.is_ok());
    }
}