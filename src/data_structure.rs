use log::{debug, trace};
use std::collections::HashSet;
use std::ops::{Deref, DerefMut};

use crate::vertex::VertexId;

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

impl<T> IntoIterator for Stack<T> {
    type Item = T;
    type IntoIter = std::vec::IntoIter<T>;

    fn into_iter(self) -> Self::IntoIter {
        self.vec.into_iter()
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

pub struct HullSet {
    hull: HashSet<VertexId>,
}

impl Deref for HullSet {
    type Target = HashSet<VertexId>;

    fn deref(&self) -> &Self::Target {
        &self.hull
    }
}

impl DerefMut for HullSet {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.hull
    }
}

impl IntoIterator for HullSet {
    type Item = VertexId;
    type IntoIter = std::collections::hash_set::IntoIter<VertexId>;

    fn into_iter(self) -> Self::IntoIter {
        self.hull.into_iter()
    }
}

impl HullSet {
    pub fn new() -> Self {
        Self {
            hull: HashSet::new(),
        }
    }

    pub fn insert(&mut self, id: VertexId) {
        debug!("Adding vertex to hull: {id}");
        self.hull.insert(id);
    }
}
