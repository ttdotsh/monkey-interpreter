use super::object::Object;
use std::collections::HashMap;

pub struct Environment<'e> {
    store: HashMap<String, Object>,
    parent: Option<&'e Environment<'e>>,
}

impl<'e> Environment<'e> {
    pub fn new() -> Environment<'e> {
        Environment {
            store: HashMap::new(),
            parent: None,
        }
    }

    #[allow(unused)]
    pub fn make_child(&'e self) -> Environment<'e> {
        Environment {
            store: HashMap::new(),
            parent: Some(&self),
        }
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
            Some(parent_env) => match parent_env.store.get(key) {
                Some(o) => Some(o.to_owned()),
                None => parent_env.check_parent(key),
            },
            None => None,
        }
    }
}

#[cfg(test)]
mod test {
    use super::{Environment, Object};

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
        let mut env = Environment::new();
        env.set("five".to_string(), Object::Integer(5));

        let mut child_env = env.make_child();
        child_env.set("six".to_string(), Object::Integer(6));

        let mut grandchild_env = child_env.make_child();
        grandchild_env.set("seven".to_string(), Object::Integer(7));

        let five_from_parent = grandchild_env.get("five");
        let six_from_child = grandchild_env.get("six");
        let seven_from_grandchild = grandchild_env.get("seven");

        assert_eq!(five_from_parent, Some(Object::Integer(5)));
        assert_eq!(six_from_child, Some(Object::Integer(6)));
        assert_eq!(seven_from_grandchild, Some(Object::Integer(7)));
    }
}
