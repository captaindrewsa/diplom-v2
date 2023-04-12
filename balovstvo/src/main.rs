use std::sync::Weak;
use std::rc::Rc;

fn main() {
    let mut a = Test1{ value: 135, ..Default::default() };

    let mut b = Test1{ value: 425, child_node:  };

    a.child_node = Box::new(b);




}


#[derive(Default, Debug)]
pub struct Test1{
    pub value: i32,
    pub child_node: Weak<Test1>
}
