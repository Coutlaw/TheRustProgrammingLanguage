use std::cell::RefCell;
use std::rc::{Rc, Weak};

// Implementation of a basic tree with out a cyclical reference, using strong and weak references
/*
* Every node is going to own its children, but share them so we can access each node directly
*   to accomplish this, we make every child a Vec<T> and T is an Rc<Node> to maintain a reference count for the smart pointers
* We also need to be able to modify nodes that are children of other nodes
*   to accomplish this we wrap each child Vec in RefCell<T>
* We also need to track who is the parent of the node
*   to accomplish this we add a parent struct field, we can't use an RC because that would be a strong reference
*   instead, we can use RefCell<Weak<Node>> so that the our strong_count will not prevent rust ownership and borrow checker errors
*/
#[derive(Debug)]
struct Node {
    value: i32,
    parent: RefCell<Weak<Node>>,      // the node does not own its parent
    children: RefCell<Vec<Rc<Node>>>, // but does own its children
}


// With the above struct, we can create strong relations from parents to children,
// and weak relations from children to parents
fn main() {
    let leaf = Rc::new(Node {
        value: 3,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![]),
    });

    // upgrade brings the weak pointer to an Rc
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade()); // nothing, because there is no parent
    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    ); // leaf will have one strong self reference, and 0 weak references

    // parent of leaf, has a reference to it
    let branch = Rc::new(Node {
        value: 5,
        parent: RefCell::new(Weak::new()),
        children: RefCell::new(vec![Rc::clone(&leaf)]), // this establishes that leaf belongs to the branch through a reference
    });

    *leaf.parent.borrow_mut() = Rc::downgrade(&branch); // mutate the leaf parent field to be a weak reference to the branch

    // At this point the branch owns the leaf as a child, and the leaf has the branch (weak ref) as a parent
    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // branch could be deleted here and the leaf would persist
    println!("leaf parent = {:?}", leaf.parent.borrow().upgrade());

    // Remove the leaf from the branch and unset the leaf's parent reference
    *branch.children.borrow_mut() = vec![];
    *leaf.parent.borrow_mut() = Weak::new();

    println!(
        "branch strong = {}, weak = {}",
        Rc::strong_count(&branch),
        Rc::weak_count(&branch)
    );

    println!(
        "leaf strong = {}, weak = {}",
        Rc::strong_count(&leaf),
        Rc::weak_count(&leaf)
    );

    // At this point neither branch nor leaf reference each other and are both safe to delete
}
