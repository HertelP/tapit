use std::cell::RefCell;
use std::rc::Rc;

const LEAF_LEN: usize = 12;

pub struct Node {
    left: Option<Rc<RefCell<Node>>>,
    right: Option<Rc<RefCell<Node>>>,
    parent: Option<Rc<RefCell<Node>>>,
    len: usize,
    content: Option<Vec<char>>,
}
impl Node {
    pub fn init() -> Self {
        Node {
            left: None,
            right: None,
            parent: None,
            len: 0,
            content: Some(Vec::with_capacity(LEAF_LEN)),
        }
    }
    pub fn len(&self) -> usize {
        self.len
    }
    pub fn add_content(&mut self, c: char) {
        self.len += 1;
        let mut content = self.content.clone().unwrap_or(Vec::with_capacity(LEAF_LEN));
        content.push(c);
        self.content = Some(content);
        if let Some(p) = self.parent.clone() {
            p.borrow_mut().add_len(1);
        }
    }
    pub fn add_len(&mut self, len: usize) {
        self.len += len;
        if let Some(p) = self.parent.clone() {
            p.borrow_mut().add_len(len);
        }
    }
    pub fn mirror(&self) -> Node {
        let new_node = Node {
            left: self.left.clone(),
            right: self.right.clone(),
            parent: self.parent.clone(),
            len: self.len,
            content: self.content.clone(),
        };
        new_node
    }
    pub fn make_branch(&mut self) {
        self.content = None;
    }
    pub fn set_left(&mut self, left: Rc<RefCell<Node>>) {
        self.left = Some(left);
    }
    pub fn set_right(&mut self, right: Rc<RefCell<Node>>) {
        self.right = Some(right);
    }
    pub fn get_left(&self) -> Option<Rc<RefCell<Node>>> {
        self.left.clone()
    }
    pub fn get_right(&self) -> Option<Rc<RefCell<Node>>> {
        self.right.clone()
    }
    pub fn get_content(&self) -> Vec<char> {
        self.content.clone().unwrap_or(vec![])
    }
}
pub fn add(node: Rc<RefCell<Node>>, c: char) -> Rc<RefCell<Node>> {
    if node.clone().borrow().len() < LEAF_LEN {
        node.clone().borrow_mut().add_content(c);
        node
    } else {
        let mut new_left = node.clone().borrow().mirror();
        new_left.parent = Some(node.clone());
        node.borrow_mut().make_branch();
        let mut new_right = Node::init();
        new_right.parent = Some(node.clone());
        new_right.add_content(c);
        let new_rc = Rc::new(RefCell::new(new_right));
        node.clone()
            .borrow_mut()
            .set_left(Rc::new(RefCell::new(new_left)));
        node.clone().borrow_mut().set_right(new_rc.clone());
        new_rc
    }
}
fn collect_print_string(node: Rc<RefCell<Node>>, depth: usize) -> Vec<(usize, String)> {
    let mut s = vec![];
    if !node.borrow().left.is_some() && !node.borrow().right.is_some() {
        return vec![(
            depth,
            format!("{}", node.borrow().get_content().iter().collect::<String>()),
        )];
    } else {
        s.push((depth, format!("{}", node.borrow().len())));
        if let Some(left) = node.borrow().get_left() {
            let mut left = collect_print_string(left, depth + 1);
            s.append(left.as_mut());
        }
        if let Some(right) = node.borrow().get_right() {
            let mut right = collect_print_string(right, depth + 1);
            s.append(right.as_mut());
        }
    }
    return s;
}
pub fn print_rope(node: Rc<RefCell<Node>>, depth: usize) {
    let mut s = collect_print_string(node, depth);
    s.sort_by(|a, b| a.0.cmp(&b.0));
    let mut d = 0;
    for (depth, string) in s {
        if d != depth {
            print!("\n\r{}", string);
            d = depth;
        } else if depth == 0 {
            print!("{}", string);
        } else {
            print!("-{}", string);
        }
    }
    print!("\n\r");
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_rope() {
        let rope = Rc::new(RefCell::new(Node::init()));
        add(rope.clone(), '1');
        add(rope.clone(), '2');
        add(rope.clone(), '3');
        add(rope.clone(), '4');
        add(rope.clone(), '5');
        add(rope.clone(), '6');
        add(rope.clone(), '7');
        add(rope.clone(), '8');
        add(rope.clone(), '9');
        add(rope.clone(), 'a');
        add(rope.clone(), 'b');
        add(rope.clone(), 'c');
        add(rope.clone(), 'd');
        add(rope.clone(), 'e');
        add(rope.clone(), 'f');
        add(rope.clone(), 'g');
        print_rope(rope.clone(), 0);
        assert!(1 == 2);
    }
}
