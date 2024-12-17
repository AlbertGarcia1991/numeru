// THIS IS A BADLY IMPLEMENTED LINKED LIST (STACK)

/*
There are 3 primary forms that self can take: self, &mut self, and &self.
These 3 forms represent the three primary forms of ownership in Rust:
    self - Value
    &mut self - mutable reference
    &self - shared reference
A value represents true ownership. You can do whatever you want with a 
value: move it, destroy it, mutate it, or loan it out via a reference. 
When you pass something by value, it's moved to the new location. The new 
location now owns the value, and the old location can no longer access it.
For this reason most methods don't want self -- it would be pretty lame if
trying to work with a list made it go away!

A mutable reference represents temporary exclusive access to a value that 
you don't own. You're allowed to do absolutely anything you want to a 
value you have a mutable reference to as long you leave it in a valid 
state when you're done (it would be rude to the owner otherwise!). This 
means you can actually completely overwrite the value. A really useful 
special case of this is swapping a value out for another, which we'll be 
using a lot. The only thing you can't do with an &mut is move the value 
out with no replacement. &mut self is great for methods that want to 
mutate self.

A shared reference represents temporary shared access to a value that you 
don't own. Because you have shared access, you're generally not allowed to 
mutate anything. Think of & as putting the value out on display in a 
museum. & is great for methods that only want to observe self.
*/
use std::mem;


struct Node {
    elem: i32,
    next: Link,
}

enum Link {
    Nil,
    More(Box<Node>),
}

pub struct List {
    head: Link,
}

impl List {
    pub fn new() -> Self {
        List { head: Link::Nil }
    }

    pub fn push(&mut self, elem: i32) {
        // We want to mutate the List, hence, we need &self
        let new_node = Box::new(Node {
            elem: elem,
            next: mem::replace(&mut self.head, Link::Nil),
        });
        // We need to replace the head first to avoid being in a instant 
        // where self is undefined

        self.head = Link::More(new_node);
    }

    pub fn pop(&mut self) -> Option<i32> {
        // We need to return Option since the value may not exists
        match mem::replace(&mut self.head, Link::Nil) {
            Link::Nil => None,
            Link::More(node) => {
                self.head = node.next;
                Some(node.elem)
            }
        }
    }
}

impl Drop for List {
    fn drop(&mut self) {
        let mut cur_link = mem::replace(&mut self.head, Link::Nil);
        // `while let` == "do this thing until this pattern doesn't match"
        while let Link::More(mut boxed_node) = cur_link {
            cur_link = mem::replace(&mut boxed_node.next, Link::Nil);
            // boxed_node goes out of scope and gets dropped here;
            // but its Node's `next` field has been set to Link::Nil
            // so no unbounded recursion occurs.
        }
    }
}


// fn main() {
// }

mod test {
    use super::List;

    #[cfg(test)]
    // cfg indicates that test module only compiled if running 'cargo test'
    fn basics() {
        let mut list = List::new();

        assert_eq!(list.pop(), None);

        list.push(1);
        list.push(2);
        list.push(3);

        assert_eq!(list.pop(), Some(3));
        assert_eq!(list.pop(), Some(2));

        list.push(4);

        assert_eq!(list.pop(), Some(4));
        assert_eq!(list.pop(), Some(1));
        assert_eq!(list.pop(), None);

    }
}