use log::trace;
use std::ops::{Deref, DerefMut};

pub struct Stack<T> {
    vec: Vec<T>,
    name: Option<String>,
}

impl<T> Deref for Stack<T> {
    type Target = Vec<T>;

    fn deref(&self) -> &Self::Target {
        &self.vec
    }
}

impl<T> DerefMut for Stack<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.vec
    }
}

impl<T: std::fmt::Debug> Stack<T> {
    pub fn new() -> Self {
        Self {
            vec: Vec::new(),
            name: None,
        }
    }

    pub fn with_name(name: String) -> Self {
        Self {
            vec: Vec::new(),
            name: Some(name),
        }
    }

    pub fn push(&mut self, element: T) {
        if let Some(name) = &self.name {
            trace!("Push to {name} stack: {element:?}");
        } else {
            trace!("Push to stack: {element:?}");
        }
        self.vec.push(element);
    }

    pub fn pop(&mut self) -> Option<T> {
        let element = self.vec.pop();
        if let Some(e) = &element {
            if let Some(name) = &self.name {
                trace!("Pop from {name} stack: {e:?}");
            } else {
                trace!("Pop from stack: {e:?}");
            }
        } else {
            if let Some(name) = &self.name {
                trace!("Pop from empty {name} stack");
            } else {
                trace!("Pop from empty stack");
            }
        }
        element
    }
}
