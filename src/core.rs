use crate::simulate::Simulate;
use crate::InnerResult;
use clipboard::{ClipboardContext, ClipboardProvider};
use std::collections::HashMap;
use std::thread::sleep;
use std::time::Duration;

pub struct FastPaste {
    content: HashMap<char, String>,
}

impl FastPaste {
    pub fn new() -> Self {
        Self {
            content: Default::default(),
        }
    }

    pub fn copy(&mut self, c: char) -> InnerResult<()> {
        let old = Self::get_contents()?;
        Simulate::ctrl_c();
        let new = Self::get_contents()?;
        self.content.insert(c, new.clone());
        Self::set_contents(old);

        Ok(())
    }

    pub fn paste(&self, c: char) -> InnerResult<()> {
        let context = match self.content.get(&c) {
            None => {
                return Ok(());
            }
            Some(context) => context.clone(),
        };

        let old = Self::get_contents()?;
        Self::set_contents(context);
        Simulate::ctrl_v();
        Self::set_contents(old);

        Ok(())
    }

    //因为clipboard的关闭依赖对象销毁Drop，所以另外写一个方法
    fn get_contents() -> InnerResult<String> {
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        Ok(ctx.get_contents()?)
    }

    //因为clipboard的关闭依赖对象销毁Drop，所以另外写一个方法
    fn set_contents(context: String) -> InnerResult<()> {
        let mut ctx: ClipboardContext = ClipboardProvider::new()?;
        ctx.set_contents(context)?;
        Ok(())
    }
}
