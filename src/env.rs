use std::{cell::RefCell, collections::HashMap, rc::Rc};

use crate::object::Object;

#[derive(Default,Debug,PartialEq, Eq)]
pub struct Env{
    parent: Option<Rc<RefCell<Env>>>,
    vars: HashMap<String,Object>
}

impl Env {

    // 默认初始化
    pub fn new() -> Self {
        Default::default()
    }

    // 添加模块
    pub fn extend(parent: Rc<RefCell<Self>>) -> Env {
        Env{
            vars : HashMap::new(),
            parent : Some(parent),
        }
    }

    // 获取东西
    pub fn get(&self,name: &str) -> Option<Object>{
        match self.vars.get(name) {
            Some(value) => Some(value.clone()),
            None => self.parent.as_ref().and_then(|o| o.borrow().get(name).clone()),
        }
    }
    
    // 添加解析元素
    pub fn set (&mut self, name: &str , val: Object){
        self.vars.insert(name.to_string(), val);
    }
}
