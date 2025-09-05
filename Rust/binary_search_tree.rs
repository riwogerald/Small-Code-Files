use std::cmp::Ordering;
use std::fmt::Display;

#[derive(Debug, Clone)]
struct TreeNode<T> {
    value: T,
    left: Option<Box<TreeNode<T>>>,
    right: Option<Box<TreeNode<T>>>,
}

impl<T> TreeNode<T> {
    fn new(value: T) -> Self {
        TreeNode {
            value,
            left: None,
            right: None,
        }
    }
}

#[derive(Debug)]
struct BinarySearchTree<T> {
    root: Option<Box<TreeNode<T>>>,
    size: usize,
}

impl<T: Ord + Clone + Display> BinarySearchTree<T> {
    fn new() -> Self {
        BinarySearchTree {
            root: None,
            size: 0,
        }
    }

    fn insert(&mut self, value: T) {
        self.root = Self::insert_recursive(self.root.take(), value);
        self.size += 1;
    }

    fn insert_recursive(node: Option<Box<TreeNode<T>>>, value: T) -> Option<Box<TreeNode<T>>> {
        match node {
            None => Some(Box::new(TreeNode::new(value))),
            Some(mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => {
                        node.left = Self::insert_recursive(node.left.take(), value);
                    }
                    Ordering::Greater => {
                        node.right = Self::insert_recursive(node.right.take(), value);
                    }
                    Ordering::Equal => {
                        // Value already exists, don't insert duplicate
                        return Some(node);
                    }
                }
                Some(node)
            }
        }
    }

    fn search(&self, value: &T) -> bool {
        Self::search_recursive(&self.root, value)
    }

    fn search_recursive(node: &Option<Box<TreeNode<T>>>, value: &T) -> bool {
        match node {
            None => false,
            Some(node) => {
                match value.cmp(&node.value) {
                    Ordering::Equal => true,
                    Ordering::Less => Self::search_recursive(&node.left, value),
                    Ordering::Greater => Self::search_recursive(&node.right, value),
                }
            }
        }
    }

    fn delete(&mut self, value: &T) -> bool {
        let (new_root, deleted) = Self::delete_recursive(self.root.take(), value);
        self.root = new_root;
        if deleted {
            self.size -= 1;
        }
        deleted
    }

    fn delete_recursive(node: Option<Box<TreeNode<T>>>, value: &T) -> (Option<Box<TreeNode<T>>>, bool) {
        match node {
            None => (None, false),
            Some(mut node) => {
                match value.cmp(&node.value) {
                    Ordering::Less => {
                        let (new_left, deleted) = Self::delete_recursive(node.left.take(), value);
                        node.left = new_left;
                        (Some(node), deleted)
                    }
                    Ordering::Greater => {
                        let (new_right, deleted) = Self::delete_recursive(node.right.take(), value);
                        node.right = new_right;
                        (Some(node), deleted)
                    }
                    Ordering::Equal => {
                        // Node to delete found
                        match (node.left.take(), node.right.take()) {
                            (None, None) => (None, true), // Leaf node
                            (Some(left), None) => (Some(left), true), // Only left child
                            (None, Some(right)) => (Some(right), true), // Only right child
                            (Some(left), Some(right)) => {
                                // Two children: replace with inorder successor
                                let (min_value, new_right) = Self::extract_min(Some(right));
                                let mut new_node = TreeNode::new(min_value);
                                new_node.left = Some(left);
                                new_node.right = new_right;
                                (Some(Box::new(new_node)), true)
                            }
                        }
                    }
                }
            }
        }
    }

    fn extract_min(node: Option<Box<TreeNode<T>>>) -> (T, Option<Box<TreeNode<T>>>) {
        match node {
            Some(mut node) => {
                if node.left.is_none() {
                    (node.value, node.right.take())
                } else {
                    let (min_value, new_left) = Self::extract_min(node.left.take());
                    node.left = new_left;
                    (min_value, Some(node))
                }
            }
            None => panic!("Cannot extract min from empty tree"),
        }
    }

    fn inorder_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        Self::inorder_recursive(&self.root, &mut result);
        result
    }

    fn inorder_recursive(node: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            Self::inorder_recursive(&node.left, result);
            result.push(node.value.clone());
            Self::inorder_recursive(&node.right, result);
        }
    }

    fn preorder_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        Self::preorder_recursive(&self.root, &mut result);
        result
    }

    fn preorder_recursive(node: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            result.push(node.value.clone());
            Self::preorder_recursive(&node.left, result);
            Self::preorder_recursive(&node.right, result);
        }
    }

    fn postorder_traversal(&self) -> Vec<T> {
        let mut result = Vec::new();
        Self::postorder_recursive(&self.root, &mut result);
        result
    }

    fn postorder_recursive(node: &Option<Box<TreeNode<T>>>, result: &mut Vec<T>) {
        if let Some(node) = node {
            Self::postorder_recursive(&node.left, result);
            Self::postorder_recursive(&node.right, result);
            result.push(node.value.clone());
        }
    }

    fn height(&self) -> usize {
        Self::height_recursive(&self.root)
    }

    fn height_recursive(node: &Option<Box<TreeNode<T>>>) -> usize {
        match node {
            None => 0,
            Some(node) => {
                let left_height = Self::height_recursive(&node.left);
                let right_height = Self::height_recursive(&node.right);
                1 + left_height.max(right_height)
            }
        }
    }

    fn size(&self) -> usize {
        self.size
    }

    fn is_empty(&self) -> bool {
        self.root.is_none()
    }

    fn find_min(&self) -> Option<&T> {
        Self::find_min_recursive(&self.root)
    }

    fn find_min_recursive(node: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        match node {
            None => None,
            Some(node) => {
                if node.left.is_none() {
                    Some(&node.value)
                } else {
                    Self::find_min_recursive(&node.left)
                }
            }
        }
    }

    fn find_max(&self) -> Option<&T> {
        Self::find_max_recursive(&self.root)
    }

    fn find_max_recursive(node: &Option<Box<TreeNode<T>>>) -> Option<&T> {
        match node {
            None => None,
            Some(node) => {
                if node.right.is_none() {
                    Some(&node.value)
                } else {
                    Self::find_max_recursive(&node.right)
                }
            }
        }
    }

    fn display_tree(&self) {
        if self.is_empty() {
            println!("Tree is empty");
            return;
        }
        
        println!("Tree structure:");
        Self::display_recursive(&self.root, 0, "Root: ");
    }

    fn display_recursive(node: &Option<Box<TreeNode<T>>>, depth: usize, prefix: &str) {
        if let Some(node) = node {
            println!("{}{}{}", "  ".repeat(depth), prefix, node.value);
            
            if node.left.is_some() || node.right.is_some() {
                Self::display_recursive(&node.left, depth + 1, "L--- ");
                Self::display_recursive(&node.right, depth + 1, "R--- ");
            }
        }
    }
}

fn main() {
    println!("🌳 Binary Search Tree Implementation");
    println!("===================================");

    let mut bst = BinarySearchTree::new();

    loop {
        println!("\nOptions:");
        println!("1. Insert value");
        println!("2. Search value");
        println!("3. Delete value");
        println!("4. Display tree");
        println!("5. Show traversals");
        println!("6. Show statistics");
        println!("7. Demo with sample data");
        println!("8. Exit");
        print!("Choose an option (1-8): ");
        std::io::Write::flush(&mut std::io::stdout()).unwrap();

        let mut input = String::new();
        std::io::stdin().read_line(&mut input).expect("Failed to read input");
        
        match input.trim() {
            "1" => {
                print!("Enter value to insert: ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut value_input = String::new();
                std::io::stdin().read_line(&mut value_input).unwrap();
                
                if let Ok(value) = value_input.trim().parse::<i32>() {
                    bst.insert(value);
                    println!("✓ Inserted {}", value);
                } else {
                    println!("Invalid number!");
                }
            }
            "2" => {
                print!("Enter value to search: ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut value_input = String::new();
                std::io::stdin().read_line(&mut value_input).unwrap();
                
                if let Ok(value) = value_input.trim().parse::<i32>() {
                    if bst.search(&value) {
                        println!("✓ Found {}", value);
                    } else {
                        println!("✗ {} not found", value);
                    }
                } else {
                    println!("Invalid number!");
                }
            }
            "3" => {
                print!("Enter value to delete: ");
                std::io::Write::flush(&mut std::io::stdout()).unwrap();
                let mut value_input = String::new();
                std::io::stdin().read_line(&mut value_input).unwrap();
                
                if let Ok(value) = value_input.trim().parse::<i32>() {
                    if bst.delete(&value) {
                        println!("✓ Deleted {}", value);
                    } else {
                        println!("✗ {} not found", value);
                    }
                } else {
                    println!("Invalid number!");
                }
            }
            "4" => {
                bst.display_tree();
            }
            "5" => {
                if bst.is_empty() {
                    println!("Tree is empty!");
                } else {
                    println!("Inorder (sorted): {:?}", bst.inorder_traversal());
                    println!("Preorder: {:?}", bst.preorder_traversal());
                    println!("Postorder: {:?}", bst.postorder_traversal());
                }
            }
            "6" => {
                println!("Tree Statistics:");
                println!("  Size: {} nodes", bst.size());
                println!("  Height: {} levels", bst.height());
                if let Some(min) = bst.find_min() {
                    println!("  Minimum value: {}", min);
                }
                if let Some(max) = bst.find_max() {
                    println!("  Maximum value: {}", max);
                }
            }
            "7" => {
                println!("Loading sample data...");
                let sample_values = vec![50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45];
                for value in sample_values {
                    bst.insert(value);
                }
                println!("✓ Inserted sample values: [50, 30, 70, 20, 40, 60, 80, 10, 25, 35, 45]");
                bst.display_tree();
            }
            "8" => {
                println!("Goodbye!");
                break;
            }
            _ => {
                println!("Invalid choice! Please enter 1-8.");
            }
        }
    }
}