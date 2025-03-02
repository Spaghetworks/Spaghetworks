use std::{
    cell::{Cell, RefCell},
    fmt::Debug,
    rc::Rc,
};
#[derive(Debug)]
pub struct UfdsElement<T> {
    value: Rc<UfdsBox<T>>,
}
enum UfdsBox<T> {
    HasParent(RefCell<Rc<UfdsBox<T>>>),
    Root(UfdsRootData<T>),
}
impl<T> UfdsBox<T> {
    fn new_root(payload: T) -> Self {
        Self::Root(UfdsRootData {
            payload,
            descendant_count: Cell::new(0),
        })
    }
    fn get_root(self: &Rc<Self>) -> Rc<Self> {
        match self.as_ref() {
            UfdsBox::HasParent(parent_cell) => {
                let root = parent_cell.borrow().get_root();
                // Flatten for time complexity
                parent_cell.borrow_mut().clone_from(&root);
                root
            }
            UfdsBox::Root(_) => self.clone(),
        }
    }
}

#[derive(Debug)]
struct UfdsRootData<T> {
    descendant_count: Cell<usize>,
    payload: T,
}
impl<T> Debug for UfdsBox<T> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("UfdsBox").finish_non_exhaustive()
    }
}

impl<T> UfdsElement<T> {
    pub fn with_payload(payload: T) -> Self {
        Self {
            value: Rc::new(UfdsBox::new_root(payload)),
        }
    }
    pub fn with_same_subset(other: &UfdsElement<T>) -> Self {
        Self {
            value: Rc::new(UfdsBox::HasParent(RefCell::new(other.value.get_root()))),
        }
    }
    pub fn split_off(&mut self, payload: T) {
        self.value = Rc::new(UfdsBox::new_root(payload))
    }
    pub fn is_connected(&self, other: &UfdsElement<T>) -> bool {
        Rc::ptr_eq(&self.value.get_root(), &other.value.get_root())
    }
    pub fn connect_to(&mut self, other: &mut UfdsElement<T>) -> Result<T, ()> {
        let own_root = self.value.get_root();
        let other_root = other.value.get_root();
        if !Rc::ptr_eq(&own_root, &other_root) {
            assert!(matches!(own_root.as_ref(), UfdsBox::Root(_)));
            assert!(matches!(other_root.as_ref(), UfdsBox::Root(_)));

            if let (UfdsBox::Root(own_data), UfdsBox::Root(other_data)) =
                (own_root.as_ref(), other_root.as_ref())
            {
                let (parent, child) =
                    if own_data.descendant_count.get() > other_data.descendant_count.get() {
                        (self, other)
                    } else {
                        (other, self)
                    };
            } else {
                panic!("The value of get_root should always be a root");
            }
            todo!()
        } else {
            Err(())
        }
    }
}
impl<T> Clone for UfdsElement<T> {
    fn clone(&self) -> Self {
        todo!()
    }
}
