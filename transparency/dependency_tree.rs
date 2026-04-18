use std::fmt;

#[derive(Debug)]
pub struct DependencyNode {
    pub name: String,
    pub children: Vec<DependencyNode>,
}

impl DependencyNode {
    pub fn new(name: &str) -> Self {
        Self {
            name: name.to_string(),
            children: vec![],
        }
    }

    pub fn add_child(&mut self, child: DependencyNode) {
        self.children.push(child);
    }

    pub fn print(&self, indent: usize) {
        let padding = " ".repeat(indent);
        println!("{}- {}", padding, self.name);

        for child in &self.children {
            child.print(indent + 2);
        }
    }
}
