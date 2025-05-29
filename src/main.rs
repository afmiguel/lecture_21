#[derive(Clone, Debug)]
enum LinkedListNode {
    Nil,
    Node(i32, Box<LinkedListNode>),
}

// impl Drop for LinkedListNode {
//     fn drop(&mut self) {
//         let ptr: *const LinkedListNode = std::ptr::addr_of!(*self);
//         match self {
//             Self::Nil => println!("Encerrando LinkedListNode(Nil) {:p}!", ptr),
//             Self::Node(v, tail) => {
//                 println!("Encerrando LinkedListNode({:?}) {:p}!", v, ptr);
//             }
//         }
//     }
// }

impl LinkedListNode {
    fn display_list(&self) {
        match self {
            Self::Nil => println!("Nil"),
            Self::Node(v, tail) => {
                print!("{}->", v);
                tail.display_list();
            }
        }
    }

    pub fn push_back(&mut self, value: i32){
        match self{
            Self::Nil => *self = Self::Node(value,
                                            Box::new(Self::Nil)),
            Self::Node(_, tail) =>
                tail.push_back(value),
        }
    }

    pub fn push_front(&mut self, value: i32){
        *self = Self::Node(value, Box::new(self.clone()));
    }

    pub fn insert(&mut self, index: usize, value:i32){
        if index == 0{
            self.push_front(value);
        } else{
            match self{
                Self::Nil => panic!("Nil at {}!", index),
                Self::Node(_, tail) =>
                    tail.insert(index-1, value),
            }
        }
    }

    pub fn delete(&mut self, index: usize) -> Option<i32>{
        if index == 0{
            match self{
                Self::Nil => None,
                Self::Node(value, tail) => {
                    let temp_value = *value;
                    *self = *tail.clone();
                    Some(temp_value)
                }
            }
        } else{
            match self{
                Self::Nil => None,
                Self::Node(_, tail) =>
                    tail.delete(index-1)
            }
        }
    }

}

fn main() {
    let mut lista = LinkedListNode::Nil;
    lista.display_list();
    lista.push_back(10);
    lista.push_back(20);
    lista.push_back(30);
    lista.display_list();
    lista.push_front(5);
    lista.display_list();
    lista.insert(2,15);
    lista.display_list();
    lista.delete(2);
    lista.display_list();
}