//! Terminal colors and a small tree renderer for status and device output.

use std::io::IsTerminal;

#[derive(Debug, Clone, Copy)]
pub struct Styles {
    pub enabled: bool,
}

impl Styles {
    pub fn detect() -> Self {
        Self {
            enabled: std::io::stdout().is_terminal() && std::env::var_os("NO_COLOR").is_none(),
        }
    }

    fn paint(self, code: &str, text: &str) -> String {
        if self.enabled {
            format!("\x1b[{code}m{text}\x1b[0m")
        } else {
            text.to_string()
        }
    }

    pub fn green(self, text: impl AsRef<str>) -> String {
        self.paint("32", text.as_ref())
    }

    pub fn red(self, text: impl AsRef<str>) -> String {
        self.paint("31", text.as_ref())
    }

    pub fn yellow(self, text: impl AsRef<str>) -> String {
        self.paint("33", text.as_ref())
    }

    pub fn cyan(self, text: impl AsRef<str>) -> String {
        self.paint("36", text.as_ref())
    }

    pub fn dim(self, text: impl AsRef<str>) -> String {
        self.paint("2", text.as_ref())
    }

    pub fn bold(self, text: impl AsRef<str>) -> String {
        self.paint("1", text.as_ref())
    }
}

#[derive(Debug, Clone)]
pub struct Node {
    pub text: String,
    pub children: Vec<Node>,
}

impl Node {
    pub fn new(text: impl Into<String>) -> Self {
        Self {
            text: text.into(),
            children: Vec::new(),
        }
    }

    pub fn child(mut self, child: Node) -> Self {
        self.children.push(child);
        self
    }
}

/// Renders the root label followed by indented branches.
pub fn render_tree(root: &Node) -> String {
    let mut lines = vec![root.text.clone()];
    for (index, child) in root.children.iter().enumerate() {
        render_node(child, "", index + 1 == root.children.len(), &mut lines);
    }
    lines.join("\n")
}

fn render_node(node: &Node, prefix: &str, last: bool, lines: &mut Vec<String>) {
    let branch = if last { "└─" } else { "├─" };
    lines.push(format!("{prefix}{branch} {}", node.text));
    let next_prefix = format!("{prefix}{}", if last { "   " } else { "│  " });
    for (index, child) in node.children.iter().enumerate() {
        render_node(child, &next_prefix, index + 1 == node.children.len(), lines);
    }
}

pub fn format_speed(bytes_per_second: u64) -> String {
    const UNITS: [&str; 4] = ["B/s", "KiB/s", "MiB/s", "GiB/s"];
    let mut value = bytes_per_second as f64;
    let mut unit = 0;
    while value >= 1024.0 && unit + 1 < UNITS.len() {
        value /= 1024.0;
        unit += 1;
    }
    if unit == 0 {
        format!("{bytes_per_second} B/s")
    } else {
        format!("{value:.1} {}", UNITS[unit])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn tree_uses_box_drawing_branches() {
        let tree = Node::new("Rayload")
            .child(Node::new("Device").child(Node::new("NAS")))
            .child(Node::new("Engine"));
        assert_eq!(
            render_tree(&tree),
            "Rayload\n├─ Device\n│  └─ NAS\n└─ Engine"
        );
    }

    #[test]
    fn speed_is_human_readable() {
        assert_eq!(format_speed(512), "512 B/s");
        assert_eq!(format_speed(1536), "1.5 KiB/s");
    }
}
