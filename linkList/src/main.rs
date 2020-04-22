use std::option::Option;


pub struct List {
    head:Option<Box<Node>>,
}   

#[derive(Debug)]
struct Node {
    value:i32,
    next:Option<Box<Node>>,
}

impl List {
    //New
    fn new() -> Self {
        List {
            head:None
        }
    }

    //Push
    fn push(&mut self, value:i32) {
        let node = Box::new(Node {
            value:value,
            next:self.head.take(),
        });

        self.head = Some(node);
    }

    //Pop
    fn pop(&mut self) -> Option<i32> {
        let node = self.head.take();
        match node {
            Some(x) => {
                self.head = x.next;
                Some(x.value)
            }

            None => {
                None
            }
        }
    }

    fn iter(&self) -> ListIterator {
        ListIterator {
            next:self.head.as_ref(),
        }
    }

    //Display
    fn display(&self){
        
        let mut next = &self.head;
        loop {
            match next {
                Some(x) => {
                    println!("{}", x.value);
                    next = &x.next;
                }

                None => {
                    println!("End of List");
                    break;
                }
            }
        }
    }
}


//Interator Datatype for List
struct ListIterator<'a> {
    next:Option<&'a Box<Node>>,
}

//Implement interator trait for 'for loop'
impl<'a> Iterator for ListIterator<'a> {
    type Item = i32;
    fn next(&mut self) -> Option<i32> {
        match self.next {
            Some(node) => {
                self.next = node.next.as_ref();
                Some(node.value)
            }
            _ => {
                self.next = None;
                None
            } 
        }
        
    }
}


fn main() {
    let mut lst:List = List::new();
    let range = 0..11;
    
    //Fill in the Link List with some values
    for val in range {
        lst.push(val*5);
    }
    
    //Iterate Over the Link list
    for (i,node) in lst.iter().enumerate() {
        println!("Index {} - {}",i, node);
    }

    //Pop an Element
    lst.pop();

    //Display
    lst.display();
}
