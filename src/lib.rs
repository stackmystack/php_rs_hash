#![cfg_attr(windows, feature(abi_vectorcall))]
use std::boxed::Box;
use std::collections::HashMap;
use std::convert::{From, Into};
use std::hash::{Hash, Hasher};

use ext_php_rs::flags::DataType;
use ext_php_rs::prelude::*;
use ext_php_rs::types::Zval;

#[derive(Debug)]
struct Z(Zval);

impl Z {
    fn new(zval: Zval) -> Self {
        Self(zval)
    }
}

impl Clone for Z {
    fn clone(&self) -> Self {
        Self(self.0.shallow_clone())
    }
}

impl PartialEq for Z {
    fn eq(&self, other: &Self) -> bool {
        self.0.is_identical(&other.0)
    }
}

impl Eq for Z {
}

impl Hash for Z {
    fn hash<H: Hasher>(&self, state: &mut H) {
        match self.0.get_type() {
          DataType::String => { self.0.string().unwrap().hash(state); }
          _ => { },
        }
    }
}

impl From<&Zval> for Z {
    fn from(zv: &Zval) -> Self {
        Z::new(zv.shallow_clone())
    }
}

impl Into<Zval> for Z {
    fn into(self) -> Zval {
        self.0.shallow_clone()
    }
}

#[php_class]
#[derive(Debug)]
pub struct H {
    map: *mut HashMap<Z, Z>
}

fn build_map(data: Vec<&Zval>) -> HashMap<Z, Z> {
    data
        .chunks_exact(2)
        .map(|chunk| (chunk[0].into(), chunk[1].into()))
        .collect::<HashMap<_, _>>()
}

#[php_impl]
impl H {
    pub fn __construct(data: Vec<&Zval>) -> Self {
        let b = Box::new(build_map(data));
        let p = Box::into_raw(b);
        Self { map: p }
    }

    pub fn __destruct(#[this] this: &mut Self) {
        unsafe {
            let _ = Box::from_raw(this.map);
        }
    }

    pub fn get(&self, key: &Zval) -> Option<Zval> {
        unsafe {
            self.map.as_ref().unwrap().get(&key.into()).map(Clone::clone).map(Into::into)
        }
    }
}

#[php_module]
pub fn get_module(module: ModuleBuilder) -> ModuleBuilder {
    module
}
