use std::mem::replace;

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
        let ptr: *const LinkedListNode = std::ptr::addr_of!(*self);
        match self {
            Self::Nil => println!("Nil({:p})", ptr),
            Self::Node(v, tail) => {
                print!("{}({:p})->", v, ptr);
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
        // Clone version
        // *self = Self::Node(value, Box::new(self.clone()));

        // Swap version
        // let mut temp = Self::Nil;
        // std::mem::swap(self, &mut temp);
        // *self = Self::Node(value, Box::new(temp));

        // replace version
        let temp2 = std::mem::replace(self, Self::Nil);
        *self = Self::Node(value, Box::new(temp2));
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
                    // Cloen version
                    // *self = *tail.clone();

                    // Swap version
                    let mut temp = Self::Nil;
                    std::mem::swap(tail as &mut Self, &mut temp);
                    *self = temp;
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
    lista.push_back(20);
    lista.push_back(30);
    lista.display_list();
    lista.push_front(10);
    lista.display_list();
//    lista.delete(0);
//    lista.display_list();
}