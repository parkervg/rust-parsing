use std::cell::RefCell;
use std::rc::Rc;

// 'Smart pointers' are like traditional pointers (which point to an address in memory)
// but with additional metadata and capabilities.
// Most common smart pointers in Rust:
// Box<T> for allocating values on the heap
// Because Box<T> is a pointer, Rust always knows how much space it needs
// a pointer's size doesn't change based on the amount of data it's pointing to
// Rc<T>, a reference counting type that enables multiple ownership
// Ref<T>, accessed through RefCell<T> that enforces borrowing rules at runtime instead of compile time
// Interior mutability is a design pattern in Rust that allows you to mutate data even when there are immutable references to that data
// RefCell is an example of this
// Recall:
//  At any given time, you can have either 1) one mutable reference, or 2) any number of immutable references
//  Box enforces these rules at compile time, RefCell enforces at runtime
// Because some analysis is impossible, if the Rust compiler can’t be sure the code complies with the ownership rules, it might reject a correct program; in this way, it’s conservative.
// The RefCell<T> type is useful when you’re sure your code follows the borrowing rules but the compiler is unable to understand and guarantee that.

type TreeNodeRef = Rc<RefCell<TreeNode>>;

#[derive(Debug)]
pub struct TreeNode {
    left: Option<TreeNodeRef>,
    right: Option<TreeNodeRef>,
    name: String,
}

impl TreeNode {
    pub fn new(name: &str) -> TreeNodeRef {
        Rc::new(RefCell::new(TreeNode {
            left: None,
            right: None,
            name: name.to_string(),
        }))
    }

    pub fn to_string(&self) -> String {
        let mut result = String::new();
        self.to_string_helper(&mut result);
        result
    }

    fn to_string_helper(&self, result: &mut String) {
        result.push('(');
        if let Some(left) = &self.left {
            left.borrow().to_string_helper(result);
            result.push(' ');
        }
        result.push_str(&self.name);
        if let Some(right) = &self.right {
            result.push(' ');
            right.borrow().to_string_helper(result);
        }
        result.push(')');
    }
}

// The + '_ at the end of the return type tells the compiler that the returned iterator has a lifetime that's
// connected to some input parameter (in this case, s).
fn lexer(s: &str) -> impl Iterator<Item = char> + '_ {
    s.chars().filter(|&c| c != ' ')
}

pub struct Parser<'a> {
    lex: Box<dyn Iterator<Item = char> + 'a>,
    current: char,
}

impl Parser<'_> {
    pub fn new(s: &str) -> Parser<'_> {
        let mut lex = lexer(s);
        let current = lex.next().unwrap_or('\0');
        Parser {
            lex: Box::new(lex),
            current,
        }
    }

    fn accept(&mut self, c: char) -> bool {
        if self.current == c {
            self.current = self.lex.next().unwrap_or('\0');
            true
        } else {
            false
        }
    }

    fn literal(&mut self) -> Option<TreeNodeRef> {
        if self.accept('(') {
            let l = self.start();
            if self.accept(')') {
                return l;
            } else {
                return None;
            }
        }
        let l = self.current;
        self.current = self.lex.next().unwrap_or('\0');
        if self.current.is_uppercase() {
            println!("Expected a capital letter");
            return None;
        }
        Some(TreeNode::new(&String::from(l)))
    }

    fn negation(&mut self) -> Option<TreeNodeRef> {
        if self.accept('!') {
            self.literal().map(|l| {
                let n = TreeNode::new("Not");
                n.borrow_mut().right = Some(l);
                n
            })
        } else {
            self.literal()
        }
    }

    fn disjunction(&mut self) -> Option<TreeNodeRef> {
        let l = self.conjunction();
        if self.accept('|') {
            self.disjunction().map(|r| {
                let n = TreeNode::new(&String::from("or"));
                n.borrow_mut().right = Some(r);
                n.borrow_mut().left = l;
                n
            })
        } else {
            return l;
        }
    }

    fn conjunction(&mut self) -> Option<TreeNodeRef> {
        let l = self.negation();
        if self.accept('&') {
            self.conjunction().map(|r| {
                let n = TreeNode::new("and");
                n.borrow_mut().right = Some(r);
                n.borrow_mut().left = l;
                n
            })
        } else {
            l
        }
    }

    pub fn start(&mut self) -> Option<TreeNodeRef> {
        let l = self.disjunction();
        if self.accept('-') {
            if self.accept('>') {
                self.start().map(|r| {
                    let n = TreeNode::new(&String::from("implies"));
                    n.borrow_mut().right = Some(r);
                    n.borrow_mut().left = l;
                    n
                })
            } else {
                None
            }
        } else {
            l
        }
    }
}
