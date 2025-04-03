pub struct ESP {
    items: Vec<String>,
}

impl ESP {
    pub fn new() -> Self {
        ESP { items: Vec::new() }
    }

    pub fn detect_items(&mut self) {
        self.items.push("Health Pack".to_string());
        self.items.push("Shield Potion".to_string());
        println!("Detected items: {:?}", self.items);
    }

    pub fn display(&self) {
        for item in &self.items {
            println!("ESP: {}", item);
        }
    }
}