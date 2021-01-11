pub struct Leaf<T> {
    pub contents: Vec<(T, Option<Leaf<T>>)>,
}

impl<T> Leaf<T> {
    pub fn new(contents: Vec<(T, Option<Leaf<T>>)>) -> Self {
        Leaf { contents }
    }
}

pub trait LeafPrint {
    fn id(&self) -> String;
    fn desc(&self) -> String;
}

impl<T: Clone> Leaf<T> {
    pub fn prune(&mut self) -> bool {
        if self.contents.is_empty() {
            return true;
        }
        if self.contents[0].1.is_none() || self.contents[0].1.as_mut().unwrap().prune() {
            self.contents.remove(0);
        }
        self.contents.is_empty()
    }

    pub fn list(&self) -> Vec<T> {
        if self.contents.is_empty() {
            vec![]
        } else if let Some(leaf) = &self.contents[0].1 {
            let mut child_contents = leaf.list();
            child_contents.push(self.contents[0].0.clone());
            child_contents
        } else {
            vec![self.contents[0].0.clone()]
        }
    }
}

impl<T: LeafPrint> Leaf<T> {
    pub fn print(&self, indentation: usize, prefix: &str) {
        if self.contents.is_empty() {
            debug_log!("{1: <0$}{2} Leaf: empty", indentation, "", prefix);
        } else {
            debug_log!(
                "{1: <0$}{3} Leaf: {2}",
                indentation,
                "",
                self.contents[0].0.id(),
                prefix
            );
            for (content, leaf) in &self.contents {
                if let Some(leaf) = leaf {
                    leaf.print(indentation + 2, &content.desc());
                }
            }
        }
    }
}
