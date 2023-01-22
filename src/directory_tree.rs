use super::Entry;

#[derive(Debug, Default, Clone)]
pub struct DirectoryItem {
    idx: usize,
    val: String,
    children: Vec<usize>,
}

impl DirectoryItem {
    fn new(idx: usize, val: String) -> Self {
        Self {
            idx,
            val,
            children: vec![],
        }
    }
}

#[derive(Debug, Default, Clone)]
pub struct DirectoryTree {
    arena: Vec<DirectoryItem>,
}

impl DirectoryTree {
    fn node(&mut self, val: String) -> usize {
        //first see if it exists
        for node in &self.arena {
            if node.val == val {
                return node.idx;
            }
        }
        // Otherwise, add new node
        let idx = self.arena.len();
        self.arena.push(DirectoryItem::new(idx, val));
        idx
    }

    pub fn from_entries(entries: &Vec<Entry>, root_name: &str) -> Self {
        let mut paths = entries
            .iter()
            .map(|x| {
                x.path
                    .as_path()
                    .components()
                    .map(|x| x.as_os_str().to_str().unwrap().replace(".md", ""))
                    .filter(|x| x != "404")
                    .collect::<Vec<_>>()
            })
            .collect::<Vec<Vec<String>>>();

        paths.sort();

        let mut tree: DirectoryTree = DirectoryTree::default();
        let directory = tree.node(root_name.to_string());
        let mut base = directory;

        for path_segments in paths {
            for (idx, segment) in path_segments.iter().enumerate() {
                // always reset the base to the root on the first segment
                if idx == 0 {
                    base = directory;
                }

                // fetch (and create if missing)
                let node = tree.node(segment.clone());

                // add nodes
                if !tree.arena[base].children.contains(&node) {
                    tree.arena[base].children.push(node);
                }
                // switch base
                base = node;
            }
        }

        return tree;
    }

    pub fn to_string(&self) -> String {
        fn traverse(tree: &DirectoryTree, item: &DirectoryItem, res: &mut String, depth: u8) {
            if item.children.len() > 0 {
                res.push_str(
                    format!(
                        "<details style=\"--depth: {}\"><summary>{}</summary>\n",
                        depth, item.val
                    )
                    .as_str(),
                );

                for child in &item.children {
                    traverse(tree, &tree.arena[*child], res, depth + 1)
                }

                res.push_str("</details>\n");
            } else {
                res.push_str(format!("\n* [[{}]]\n", item.val).as_str());
            }
        }

        // own the string here, so we can add to it
        let mut formatted = String::new();
        // update the string recrusively
        traverse(&self, &self.arena[0], &mut formatted, 0);

        return formatted;
    }

    pub fn format_directory_page(&self, existing_contents: &String) -> String {
        fn traverse(tree: &DirectoryTree, item: &DirectoryItem, res: &mut String, depth: u8) {
            let indent = "  ".repeat(depth.into());

            if item.children.len() > 0 {
                res.push_str(format!("{}* {}\n", indent, item.val,).as_str());

                for child in &item.children {
                    traverse(tree, &tree.arena[*child], res, depth + 1)
                }
            } else {
                res.push_str(format!("{}* [[{}]]\n", indent, item.val).as_str());
            }
        }

        // own the string here, so we can add to it
        let mut formatted = String::new();
        // update the string recrusively
        traverse(self, &self.arena[0], &mut formatted, 0);

        return format!("{}\n{}", existing_contents, formatted);
    }
}

#[cfg(test)]
mod tests {
    use super::super::collect_entries;
    use super::*;

    #[test]
    fn test_directory() {
        let entries = collect_entries("content/entries").unwrap();
        let directory = DirectoryTree::from_entries(&entries, "pages");
        assert!(directory.arena.len() > 10);
    }
}
