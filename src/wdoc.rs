//! Documentation Generator (WDoc) - Widya Documentation Generator
//! Generates API documentation from source code comments

use std::collections::HashMap;
use std::fs::{self, File};
use std::io::{Write, Read};
use std::path::{Path, PathBuf};
use serde::{Deserialize, Serialize};

/// WDoc configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct WdocConfig {
    pub input_dir: PathBuf,
    pub output_dir: PathBuf,
    pub theme: String,
    pub include_private: bool,
    pub generate_examples: bool,
}

/// Documentation item
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct DocItem {
    pub kind: String,
    pub name: String,
    pub path: String,
    pub description: String,
    pub signatures: Vec<String>,
    pub examples: Vec<String>,
    pub attributes: HashMap<String, Vec<String>>,
    pub line_start: usize,
    pub line_end: usize,
}

/// WDoc documentation generator
pub struct Wdoc {
    config: WdocConfig,
    items: Vec<DocItem>,
}

impl Wdoc {
    /// Create new WDoc instance
    pub fn new(config: WdocConfig) -> Self {
        Self {
            config,
            items: Vec::new(),
        }
    }

    /// Load documentation from source files
    pub fn load(&mut self, source_path: &Path) -> Result<(), String> {
        let mut content = String::new();
        File::open(source_path)
            .map_err(|e| format!("Failed to open file: {}", e))?
            .read_to_string(&mut content)
            .map_err(|e| format!("Failed to read file: {}", e))?;

        self.parse_comments(&content, source_path);
        
        Ok(())
    }

    /// Parse documentation comments from content
    fn parse_comments(&mut self, content: &str, path: &Path) {
        let lines: Vec<&str> = content.lines().collect();
        let mut i = 0;

        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("///") || line.starts_with("/**") {
                if let Some(item) = self.parse_doc_block(&lines, i, path) {
                    self.items.push(item);
                }
            }
            i += 1;
        }
    }

    /// Parse documentation block
    fn parse_doc_block(&self, lines: &[&str], start: usize, path: &Path) -> Option<DocItem> {
        let mut description = String::new();
        let mut i = start;
        
        while i < lines.len() {
            let line = lines[i].trim();
            if line.starts_with("///") {
                let content = line.strip_prefix("///").unwrap_or(line).trim();
                description.push_str(content);
                description.push(' ');
            } else {
                break;
            }
            i += 1;
        }

        Some(DocItem {
            kind: "Function".to_string(),
            name: "function_name".to_string(),
            path: path.to_str()?.to_string(),
            description: description.trim().to_string(),
            signatures: vec!["function_name()".to_string()],
            examples: Vec::new(),
            attributes: HashMap::new(),
            line_start: start,
            line_end: i,
        })
    }

    /// Generate HTML documentation
    pub fn generate_html(&self, output_dir: &Path) -> Result<(), String> {
        fs::create_dir_all(output_dir)
            .map_err(|e| format!("Failed to create output directory: {}", e))?;

        let index_html = self.generate_index();
        let mut file = File::create(output_dir.join("index.html"))
            .map_err(|e| format!("Failed to create index.html: {}", e))?;
        file.write_all(index_html.as_bytes())
            .map_err(|e| format!("Failed to write index.html: {}", e))?;

        Ok(())
    }

    /// Generate HTML index page
    fn generate_index(&self) -> String {
        let mut html = String::from(
            "<!DOCTYPE html>
<html>
<head>
    <title>Widya Documentation</title>
    <style>
        body { font-family: sans-serif; max-width: 900px; margin: 0 auto; padding: 20px; }
        h1 { color: #6200ee; }
        .item { margin: 20px 0; padding: 10px; background: #f5f5f5; border-radius: 5px; }
        .name { font-weight: bold; color: #6200ee; }
    </style>
</head>
<body>
    <h1>Widya Documentation</h1>
"
        );

        for item in &self.items {
            html.push_str(&format!(
                "<div class=\"item\">
                    <span class=\"name\">{}</span>: {}<br>
                </div>",
                item.name, item.description
            ));
        }

        html.push_str("</body></html>");
        html
    }

    /// Generate JSON documentation
    pub fn generate_json(&self) -> String {
        serde_json::to_string_pretty(&self.items).unwrap_or_default()
    }

    /// Get all documented items
    pub fn items(&self) -> &[DocItem] {
        &self.items
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_wdoc_generate() {
        let config = WdocConfig {
            input_dir: PathBuf::from("/tmp/test"),
            output_dir: PathBuf::from("/tmp/docs"),
            theme: "default".to_string(),
            include_private: false,
            generate_examples: true,
        };
        
        let wdoc = Wdoc::new(config);
        let json = wdoc.generate_json();
        assert!(json.contains("["));
    }
}
