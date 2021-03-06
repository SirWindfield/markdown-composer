use crate::types::list::{List, ListItem};

#[derive(Clone, Debug, Default)]
pub struct ListBuilder {
    items: Vec<ListItem>,
}

impl ListBuilder {
    pub fn new() -> Self {
        Self::default()
    }

    #[deprecated(note = "Please use the `append` function instead.", since = "0.3.0")]
    pub fn add(mut self, item: impl Into<ListItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn append(mut self, item: impl Into<ListItem>) -> Self {
        self.items.push(item.into());
        self
    }

    pub fn ordered(self) -> List {
        List::ordered_with(self.items)
    }

    pub fn unordered(self) -> List {
        List::unordered_with(self.items)
    }
}

impl List {
    pub fn builder() -> ListBuilder {
        ListBuilder::new()
    }
}
