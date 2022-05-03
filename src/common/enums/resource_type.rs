pub enum ResourceType {
    MENU = 1,
    BUTTON = 2,
}

impl ResourceType {
    pub fn value(&self) -> i32 {
        match *self {
            ResourceType::MENU => 1,
            ResourceType::BUTTON => 2,
        }
    }
}

