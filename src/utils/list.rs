use std::cell::{RefCell, RefMut};
use std::rc::Rc;

#[derive(Debug)]
pub struct Node<T> {
    pub data: T,
    pub next: Link<T>,
}

impl<T> Node<T> {
    pub fn new(data: T) -> Self {
        Self { data, next: None }
    }

}
pub type Link<T> = Option<Rc<RefCell<Node<T>>>>;
pub struct List<T> {
    pub head: Link<T>,
}
impl<T> List<T> {
    pub fn new(data: T) -> Self {
        Self {
            head: Some(Rc::new(RefCell::new(Node::new(data)))),
        }
    }
    pub fn append(&mut self, data: T) {
        match self.head {
            Some(_) => {
                self
                .iter()
                .last()
                .unwrap()
                .borrow_mut()
                .next =
                    Some(Rc::new(RefCell::new(Node::new(data))));
            }
            None => {
                self.head = Some(Rc::new(RefCell::new(Node::new(data))));
            }
        }
    }
    
    pub fn iter(&self) -> ListIter<T> {
        ListIter {
            next: self.head.clone(),
        }
    }
}
pub struct ListIter<T> {
    next: Option<Rc<RefCell<Node<T>>>>,
}
impl<T> Iterator for ListIter<T> {
    type Item = Rc<RefCell<Node<T>>>;
    fn next(&mut self) -> Option<Self::Item> {
        match self.next.clone() {
            Some(next) => {
                self.next = next.borrow().next.clone();
                Some(next)
            }
            None => None,
        }
    }
}
#[cfg(test)]
mod list_tests{
    use super::*;
    #[test]
    fn creating_and_appending(){
        let mut list = List::new(0);
        list.append(1);
        list.append(2);
        list.iter().enumerate().for_each(|(i,node)|{
            assert_eq!(node.borrow().data,i)
        });
    }
}