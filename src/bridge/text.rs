use crate::{Serde, Text, Tree};
use std::{collections::HashMap, convert::Into};

impl Into<Tree> for Text {
    fn into(self) -> Tree {
        let mut m = HashMap::<String, String>::new();
        let mut cm = HashMap::<String, String>::new();

        m.insert("style".into(), self.style.ser());
        cm.insert("text".into(), self.text.into());

        Tree::new(
            m,
            HashMap::new(),
            vec![Tree::new(cm, HashMap::new(), vec![], None, "plain".into())],
            None,
            "p".into(),
        )
        .borrow()
        .to_owned()
    }
}
