use crate::utils;

pub struct Location {
    name: String,
    description: String,
    children: Option<Vec<Location>>,
    parent: Option<Box<Location>>,
}

impl Location {
    pub fn new(name: &str, description: &str, parent: Option<Box<Location>>, children: Option<Vec<Location>>) -> Self {
        Location{
            name: name.to_string(),
            description: description.to_string(),
            parent,
            children
        }
    }

    pub fn describe(&self) {
        println!("{}", self.name);
        println!("{}", self.description);
    }

    pub fn travel_to(&self) -> Option<String> {
        if let Some(children) = &self.children {
            let options: Vec<String> = children.iter().map(|child| child.name.clone()).collect();
            let travel_prompt = utils::SelectionPrompt::new(Some("Where would you like to travel to?"), options);
            Some(travel_prompt.select_option())
        } else {
            println!("There are no places to travel to from here.");
            None
        }
    }
    
}