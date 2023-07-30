use super::object::Object;
use std::{cell::RefCell, collections::HashMap, rc::Rc};

#[derive(Debug, PartialEq)]
pub struct Environment {
    store: HashMap<String, Object>,
    parent: Option<Rc<RefCell<Environment>>>,
}

impl Environment {
    pub fn new() -> Environment {
        Environment {
            store: HashMap::new(),
            parent: None,
        }
    }

    pub fn child_of(parent: &Rc<RefCell<Environment>>) -> Environment {
        Environment {
            store: HashMap::new(),
            parent: Some(Rc::clone(parent)),
        }
    }

    // pub fn with(mut self, names: Vec<String>, objs: Vec<Object>) -> Environment {
    //     names
    //         .into_iter()
    //         .zip(objs.into_iter())
    //         .for_each(|(name, obj)| self.set(name, obj));
    //     self
    // }

    pub fn with<K, V>(mut self, keys: K, values: V) -> Environment
    where
        K: Iterator<Item = String>,
        V: Iterator<Item = Object>,
    {
        keys.zip(values).for_each(|(k, v)| self.set(k, v));
        self
    }

    pub fn get(&self, key: &str) -> Option<Object> {
        match self.store.get(key) {
            Some(o) => Some(o.to_owned()),
            None => self.check_parent(key),
        }
    }

    pub fn set(&mut self, key: String, value: Object) {
        self.store.insert(key, value);
    }

    fn check_parent(&self, key: &str) -> Option<Object> {
        match self.parent {
            Some(ref parent_env) => match parent_env.borrow().store.get(key) {
                Some(o) => Some(o.to_owned()),
                None => parent_env.borrow().check_parent(key),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Environment, Object};
    use std::{cell::RefCell, rc::Rc};

    #[test]
    fn test_get() {
        let mut env = Environment::new();
        env.set("five".to_string(), Object::Integer(5));

        let five = env.get("five");
        let six = env.get("six");

        assert_eq!(five, Some(Object::Integer(5)));
        assert_eq!(six, None);
    }

    #[test]
    fn test_check_parent() {
        // TODO: This test doesn't reflect actual use, may be worth revisiting the API here
        let env = Rc::new(RefCell::new(Environment::new()));
        env.borrow_mut().set("five".to_string(), Object::Integer(5));

        let child_env = Rc::new(RefCell::new(Environment::child_of(&env)));
        child_env
            .borrow_mut()
            .set("six".to_string(), Object::Integer(6));

        let grandchild_env = Rc::new(RefCell::new(Environment::child_of(&child_env)));
        grandchild_env
            .borrow_mut()
            .set("seven".to_string(), Object::Integer(7));

        let five_from_parent = grandchild_env.borrow().get("five");
        let six_from_child = grandchild_env.borrow().get("six");
        let seven_from_grandchild = grandchild_env.borrow().get("seven");

        assert_eq!(five_from_parent, Some(Object::Integer(5)));
        assert_eq!(six_from_child, Some(Object::Integer(6)));
        assert_eq!(seven_from_grandchild, Some(Object::Integer(7)));
    }
}
