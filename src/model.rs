use log::trace;
use std::collections::HashMap;

pub trait EntryClone {
    fn clone_box(&self) -> Box<dyn Entry>;
}

impl<T> EntryClone for T
where
    T: 'static + Entry + Clone,
{
    fn clone_box(&self) -> Box<dyn Entry> {
        Box::new(self.clone())
    }
}

pub trait Entry: EntryClone {
    fn name(&self) -> String;
    fn set_name(&mut self, name: &String);
}

impl Clone for Box<dyn Entry> {
    fn clone(&self) -> Box<dyn Entry> {
        self.clone_box()
    }
}

#[derive(Clone)]
struct Folder {
    name: String,
    children: HashMap<String, Box<dyn Entry>>,
}

impl Folder {
    pub fn empty(name: &String) -> Folder {
        Self {
            name: name.clone(),
            children: HashMap::new(),
        }
    }

    pub fn is_empty(&self) -> bool {
        self.children.is_empty()
    }

    pub fn has_children(&self) -> bool {
        !self.is_empty()
    }

    pub fn children_count(&self) -> usize {
        self.children.len()
    }

    pub fn insert_child(&mut self, child: Box<dyn Entry>) {
        trace!(
            "Adding new child {} to folder {}",
            child.name(),
            self.name()
        );

        self.children.insert(child.name(), child);
    }

    pub fn delete_child(&mut self, child_name: &String) {
        self.children.remove(child_name);
    }
}

impl Entry for Folder {
    fn name(&self) -> String {
        self.name.clone()
    }

    fn set_name(&mut self, name: &String) {
        self.name = name.clone();
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_folder_getter_setter() {
        let name = String::from("some_name");
        let folder = Folder::empty(&name);
        assert_eq!(name, folder.name());
        assert!(folder.is_empty());
        assert_eq!(false, folder.has_children());

        let child = Folder::empty(&"child_1".to_string());
        let mut folder = folder;
        folder.insert_child(Box::new(child.clone()));
        assert_eq!(1, folder.children_count());

        folder.delete_child(&child.name());
    }
}
