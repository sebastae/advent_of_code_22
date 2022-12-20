use std::collections::HashMap;

#[derive(Debug, PartialEq)]
enum FType {
    File,
    Dir,
}

#[derive(Debug)]
struct Path {
    path: Vec<String>,
}

impl Path {
    fn traverse(&mut self, path: &str) {
        if path.starts_with("/") {
            self.path.clear();
        }

        path.split('/').for_each(|part| {
            match part {
                "" => {}
                ".." => {
                    self.path.pop();
                }
                _ => {
                    self.path.push(String::from(part));
                }
            };
        })
    }

    fn new(path: &str) -> Path {
        let mut p = Path { path: vec![] };
        p.traverse(path);
        p
    }
}

#[derive(Debug)]
struct FSNode {
    size: usize,
    t: FType,
    children: HashMap<String, FSNode>,
}

impl FSNode {
    fn new(t: FType, size: usize) -> FSNode {
        FSNode {
            size,
            t,
            children: HashMap::new(),
        }
    }

    fn root() -> FSNode {
        FSNode::new(FType::Dir, 0)
    }

    fn get_size(&self) -> usize {
        match self.t {
            FType::File => self.size,
            FType::Dir => self
                .children
                .iter()
                .fold(0, |acc, (_, node)| acc + node.get_size()),
        }
    }

    fn insert(&mut self, name: &str, size: usize, t: FType) {
        self.children.insert(
            String::from(name),
            FSNode {
                size,
                t,
                children: HashMap::new(),
            },
        );
    }

    fn get_path_mut(&mut self, p: &Path, depth: usize) -> Option<&mut FSNode> {
        if depth == p.path.len() {
            Some(self)
        } else {
            match self.children.get_mut(p.path.get(depth).unwrap()) {
                Some(child) => child.get_path_mut(p, depth + 1),
                None => None,
            }
        }
    }

    fn build(input: &str) -> FSNode {
        let lines = input.split('\n');
        let mut path = Path::new("");
        let mut root = FSNode::root();

        // Build file tree
        lines.for_each(|line| {
            if line == "" {
                return;
            }

            if let Some(line) = line.trim().strip_prefix("$") {
                if line.trim().starts_with("cd") {
                    path.traverse(
                        line.trim()
                            .strip_prefix("cd")
                            .expect("stripped line")
                            .trim(),
                    );
                } else if line.trim().starts_with("ls") {
                    // Should get some non-command lines after this, no need to do anything I suppose
                } else {
                    panic!("Unknown command: {line}");
                }
            } else {
                // Should only occurr after ls
                let parts = line.trim().split(' ').collect::<Vec<&str>>();
                let size = parts.get(0).expect("file size or dir");
                let name = parts.get(1).expect("file or dir name");

                if size == &"dir" {
                    root.get_path_mut(&path, 0)
                        .expect("dir node")
                        .insert(name, 0, FType::Dir);
                } else {
                    root.get_path_mut(&path, 0).expect("dir node").insert(
                        name,
                        size.parse::<usize>().expect("file size"),
                        FType::File,
                    );
                }
            }
        });

        root
    }
}

fn sum_dirs_under_treshold(root: &FSNode, max: usize) -> usize {
    let mut sum = 0;

    root.children.iter().for_each(|(_, node)| {
        if node.t == FType::Dir {
            let size = node.get_size();
            if size <= max {
                sum += size;
            }
            sum += sum_dirs_under_treshold(node, max);
        }
    });
    
    sum
}

fn find_smallest_above_treshold(root: &FSNode, min: usize) -> Option<usize> {
    let mut dirs: Vec<usize> = vec![];

    let root_size = root.get_size();
    if root_size >= min {
        dirs.push(root_size);
    }

    root.children.iter().for_each(|(_, node)|{
        if node.t == FType::Dir {
            let size = find_smallest_above_treshold(node, min);
            if let Some(s) = size {
                dirs.push(s);
            }
        }
    });

    let min = dirs.iter().min();

    if let Some(num) = min {
        Some(num.to_owned())
    } else {
        None
    }
}

fn main() {
    let input = include_str!("./input.txt");
    let mut root = FSNode::build(input);

    let part_1 = sum_dirs_under_treshold(&mut root, 100000);
    println!("Sum files under 100000: {part_1}");

    let free_space = 70000000 - root.get_size();
    let min_to_delete = 30000000 - free_space;

    if let Some(num) = find_smallest_above_treshold(&root, min_to_delete){
        println!("Minimum directory above {min_to_delete}: {num}");
    } else {
        println!("No directory above {min_to_delete} found");
    }
}

#[cfg(test)]
mod tests {

    use crate::*;

    #[test]
    fn size_calc() {
        let mut root = FSNode::root();

        root.children
            .insert(String::from("a"), FSNode::new(FType::File, 10));

        assert_eq!(root.get_size(), 10);
    }

    #[test]
    fn insert_node() {
        let mut root = FSNode::root();

        root.insert("a", 1, FType::File);

        assert!(root.children.get("a").is_some());
    }

    #[test]
    fn path_traverse_down() {
        let mut path = Path::new("a");

        assert_eq!(path.path, ["a"]);

        path.traverse("b");
        assert_eq!(path.path, ["a", "b"]);
    }

    #[test]
    fn path_traverse_nested() {
        let mut path = Path::new("/a/b/c");

        assert_eq!(path.path, ["a", "b", "c"]);
    }

    fn path_traverse_up() {
        let mut path = Path::new("");
    }

    #[test]
    fn get_node_by_path() {
        let mut path = Path::new("/a/b/c");
        let mut root = FSNode::root();

        root.insert("a", 0, FType::Dir);
        root.children
            .get_mut("a")
            .unwrap()
            .insert("b", 0, FType::Dir);
        root.children
            .get_mut("a")
            .unwrap()
            .children
            .get_mut("b")
            .unwrap()
            .insert("c", 10, FType::File);

        assert_eq!(root.get_path_mut(&path, 0).unwrap().size, 10);
    }

    #[test]
    fn test_input() {
        let test_input = "$ cd /
            $ ls
            dir a
            14848514 b.txt
            8504156 c.dat
            dir d
            $ cd a
            $ ls
            dir e
            29116 f
            2557 g
            62596 h.lst
            $ cd e
            $ ls
            584 i
            $ cd ..
            $ cd ..
            $ cd d
            $ ls
            4060174 j
            8033020 d.log
            5626152 d.ext
            7214296 k"
            .trim();

        let mut root = FSNode::build(test_input);

        assert_eq!(sum_dirs_under_treshold(&mut root, 100000), 95437);
        assert_eq!(find_smallest_above_treshold(&root, 8381165), Some(24933642));

    }



}
