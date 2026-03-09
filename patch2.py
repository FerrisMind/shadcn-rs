import sys
from pathlib import Path

# Path to the tree viewer demo
main_rs = "H:/Nova-Code/tp/shadcn-rs/crates/iced-shadcn/examples/tree-viewer-demo/main.rs"

with open(main_rs, "r", encoding="utf-8") as f:
    code = f.read()

default_start = code.find("impl Default for Example {")
update_start = code.find("    fn update(&mut self, message: Message) -> Task<Message> {")

# We will modify the default function to build a nested structure
# and modify the update function to correctly filter visible nodes based on expansion

new_methods = """impl Default for Example {
    fn default() -> Self {
        let mut all_nodes = Vec::new();

        // Let's create a nested structure similar to tree-view
        // src/
        all_nodes.push(FlatNode::folder("src", "/src", "src", 0, true, FolderState::Loaded));
        
        // src/components
        all_nodes.push(FlatNode::folder("components", "/src/components", "components", 1, true, FolderState::Loaded));
        all_nodes.push(FlatNode::folder("ui", "/src/components/ui", "ui", 2, false, FolderState::Loaded));
        all_nodes.push(FlatNode::file("button.rs", "/src/components/ui/button.rs", "button.rs", 3));
        all_nodes.push(FlatNode::file("tree_viewer.rs", "/src/components/tree_viewer.rs", "tree_viewer.rs", 2));
        
        // src/lib.rs
        all_nodes.push(FlatNode::file("lib.rs", "/src/lib.rs", "lib.rs", 1));

        // Let's add thousands of nested generated files to show virtualization
        all_nodes.push(FlatNode::folder("big_folder", "/big_folder", "big_folder (10,000 files)", 0, false, FolderState::Loaded));
        for i in 0..10_000 {
            all_nodes.push(FlatNode::file(
                format!("file_{i}"),
                format!("/big_folder/file_{i}.rs"),
                format!("file_{i}.rs"),
                1,
            ));
        }

        let mut example = Self {
            theme: Theme::dark(),
            state: TreeViewerState {
                nodes: vec![],
                selected_path: None,
            },
            all_nodes,
        };
        
        example.update_visible_nodes();
        example
    }
}

#[derive(Debug, Clone)]
enum Message {
    Toggle(String),
    Select(String),
    Load(String),
}

impl Example {
    fn update_visible_nodes(&mut self) {
        // A simple algorithm to filter all_nodes into state.nodes based on what is expanded.
        // It skips children of collapsed folders.
        let mut visible = Vec::new();
        let mut skip_depth = None;

        for node in &self.all_nodes {
            if let Some(depth) = skip_depth {
                if node.depth > depth {
                    continue; // Skip because a parent is collapsed
                } else {
                    skip_depth = None; // Back to a visible level
                }
            }

            visible.push(node.clone());

            if node.is_folder && !node.is_expanded {
                skip_depth = Some(node.depth);
            }
        }

        self.state.nodes = visible;
    }

    fn update(&mut self, message: Message) -> Task<Message> {
        match message {
            Message::Toggle(path) => {
                // Find node and toggle it
                if let Some(node) = self.all_nodes.iter_mut().find(|n| n.path == path) {
                    node.is_expanded = !node.is_expanded;
                }
                
                // Re-evaluate visibility
                self.update_visible_nodes();
            }
            Message::Select(path) => {
                self.state.select(&path);
            }
            Message::Load(path) => {
                println!("Loading folder: {}", path);
                // Simulate loading
            }
        }
        Task::none()
    }
"""

view_start = code.find("    fn view(&self) -> Element<'_, Message> {")

code = code[:default_start] + new_methods + code[view_start:]

# Change "100,000 Nodes" to "10,000 Nodes"
code = code.replace("100,000 Nodes", "10,005 Nodes")

with open(main_rs, "w", encoding="utf-8") as f:
    f.write(code)

print("Demo script patched.")
