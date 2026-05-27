// At compile time, Rust needs to know how much space a type takes up. This
// becomes problematic for recursive types, where a value can have as part of
// itself another value of the same type. To get around the issue, we can use a
// `Box` - a smart pointer used to store data on the heap, which also allows us
// to wrap a recursive type.
//
// The recursive type we're implementing in this exercise is the "cons list", a
// data structure frequently found in functional programming languages. Each
// item in a cons list contains two elements: The value of the current item and
// the next item. The last item is a value called `Nil`.

use crate::List::Cons;
use crate::List::Nil;
// TODO: Use a `Box` in the enum definition to make the code compile.
#[derive(PartialEq, Debug)]
enum List {
    Cons(i32, Box<List>),
    Nil,
}

impl List {
    // Why can't i do with new associated fns create an empty list ?
    fn new() -> Self{ 
        List::Nil
    }
    // fn create_empty() -> List {
    //     List::Nil
    // }

    // If there are multiple val i need to pass than what i'll do is pass an array and loop through that arrary and push value in the Cons list in Box until the loop ends but my doubt is that since box is a immutable type of refference can we even change the value or do we have to use refCell
    fn list_new(self, x: i32) -> List{
        // self.push(x)
        List::Cons(x, Box::new(self))
    }
}

// TODO: Create an empty cons list.
fn create_empty_list() -> List {
    // let s  = List::Nil;
    let s  = List::new();
    s
}

// TODO: Create a non-empty cons list.
fn create_non_empty_list() -> List {
    // todo!()
    // let s = Cons(12, Box::new(Cons(43, Box::new(Nil))));
    let mut s = List::new();
    s = s.list_new(12);
    s
}

fn main() {
    println!("This is an empty cons list: {:?}", create_empty_list());
    println!(
        "This is a non-empty cons list: {:?}",
        create_non_empty_list(),
    );
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_empty_list() {
        assert_eq!(create_empty_list(), List::Nil);
    }

    #[test]
    fn test_create_non_empty_list() {
        assert_ne!(create_empty_list(), create_non_empty_list());
    }
}
