use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::walk_item::{self, Command, WalkItem};

pub struct Dir<'a> {
    pub name: &'a str,
    pub parent: Option<Rc<Dir<'a>>>,
    pub children: RefCell<Vec<Rc<Dir<'a>>>>,
    pub files: RefCell<Vec<File<'a>>>,
}

#[derive(Debug)]
pub struct File<'a> {
    pub name: &'a str,
    pub size: usize,
}

impl<'a> Dir<'a> {
    pub fn root(name: &'a str) -> Self {
        Dir {
            name,
            parent: None,
            children: Default::default(),
            files: Default::default(),
        }
    }

    pub fn dir(name: &'a str, parent: Rc<Dir<'a>>) -> Self {
        Dir {
            name,
            parent: Some(parent),
            children: Default::default(),
            files: Default::default(),
        }
    }

    pub fn add_file(&self, name: &'a str, size: usize) {
        let file = File { name, size };
        self.files.borrow_mut().push(file);
    }

    pub fn add_dir(self: &Rc<Self>, dir_name: &'a str) -> Rc<Self> {
        let dir = Rc::new(Dir::dir(dir_name, self.clone()));
        self.children.borrow_mut().push(dir.clone());
        dir
    }

    pub fn cd(&self, dir_name: &'a str) -> Rc<Self> {
        if dir_name == ".." {
            self.parent.as_ref().expect("CD to root's parent").clone()
        } else {
            self.children
                .borrow()
                .iter()
                .find(|dir| dir.name == dir_name)
                .expect("CD to dir not under current directory")
                .clone()
        }
    }

    #[allow(dead_code)]
    pub fn size(&self) -> usize {
        let file_sizes: usize = self.files.borrow().iter().map(|file| file.size).sum();
        file_sizes
            + self
                .children
                .borrow()
                .iter()
                .map(|dir| dir.size())
                .sum::<usize>()
    }

    pub fn predicate_size(&self, predicate: fn(usize) -> bool) -> (usize, usize) {
        let file_sizes: usize = self.files.borrow().iter().map(|file| file.size).sum();
        let (total_children_size, total_children_filtered_size) = self
            .children
            .borrow()
            .iter()
            .fold((0, 0), |(total_size, filtered_size), dir| {
                let (child_size, child_filtered_size) = dir.predicate_size(predicate);
                (total_size + child_size, filtered_size + child_filtered_size)
            });
        let my_size = total_children_size + file_sizes;
        let total_filtered_size = if predicate(my_size) {
            total_children_filtered_size + my_size
        } else {
            total_children_filtered_size
        };
        (my_size, total_filtered_size)
    }

    pub fn infimo(&self, predicate: impl Fn(usize) -> bool + Copy) -> (usize, usize) {
        let file_sizes: usize = self.files.borrow().iter().map(|file| file.size).sum();
        let (total_children_size, children_infimo) = self.children.borrow().iter().fold(
            (0, usize::max_value()),
            |(total_size, current_child_infimo), dir| {
                let (child_size, child_infimo) = dir.infimo(predicate);
                (
                    total_size + child_size,
                    child_infimo.min(current_child_infimo),
                )
            },
        );
        let my_size = total_children_size + file_sizes;
        let current_infimo = if predicate(my_size) {
            children_infimo.min(my_size)
        } else {
            children_infimo
        };
        (my_size, current_infimo)
    }
}

impl<'a> std::fmt::Debug for Dir<'a> {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let mut builder = f.debug_struct("Node");
        builder.field("name", &self.name);
        if let Some(parent) = self.parent.as_ref() {
            builder.field("parent", &parent.name);
        }
        if !self.files.borrow().is_empty() {
            builder.field("files", &self.files);
        }
        if !self.children.borrow().is_empty() {
            builder.field("children", &self.children);
        }
        builder.finish()
    }
}
