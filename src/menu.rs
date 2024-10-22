use dialoguer::Select;

pub struct Menu {
    items: Vec<String>,
}

impl Menu {
    pub fn new(items: Vec<&str>) -> Self {
        Self {
            items: items.into_iter().map(String::from).collect(),
        }
    }

    pub fn get_selection(&self) -> String {
        let selection = Select::new()
            .with_prompt("What do you choose?")
            .items(&self.items)
            .interact()
            .unwrap();
        self.items[selection].clone()
    }
}
