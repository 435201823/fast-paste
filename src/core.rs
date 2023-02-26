use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;
use clipboard::{ClipboardContext, ClipboardProvider};
use crate::simulate::Simulate;

pub struct FastPaste{
    content:HashMap<char,String>
}

impl FastPaste{
    pub fn new() -> Self{
        Self{ content: Default::default() }
    }

    pub fn copy(&mut self, c: char){
        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let old = ctx.get_contents().unwrap();
        Simulate::ctrl_c();
        let new = ctx.get_contents().unwrap();
        self.content.insert(c,new.clone());
        ctx.set_contents(old).unwrap();
    }

    pub fn paste(&self,c:char){

        let context = match self.content.get(&c) {
            None => {return; }
            Some(context) => {
                context.clone()
            }
        };

        let mut ctx: ClipboardContext = ClipboardProvider::new().unwrap();
        let old = ctx.get_contents().unwrap();
        ctx.set_contents(context).unwrap();
        Simulate::ctrl_v();
        ctx.set_contents(old).unwrap();
    }

}