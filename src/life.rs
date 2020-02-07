//! LifeCycle in Elvis
use crate::Tree;
use std::convert::Into;

/// Lifecycle
///
/// 1. `create()` calling when constructs
/// 2. `update()` calling after `set_state()`
/// 3. `render()` calling after `create()` and `update()`
/// 4. `dispose()` calling after deleting tree
/// 5. `set_state()` calling by users
pub trait LifeCycle<T> {
    fn create(&mut self);
    fn update(&self);
    fn render(self) -> T;
    fn dispose(&mut self);
}

impl<T> LifeCycle<Tree> for T
where
    T: Into<Tree>,
{
    fn create(&mut self) {}
    fn update(&self) {}
    fn render(self) -> Tree {
        self.into()
    }
    fn dispose(&mut self) {}
}
