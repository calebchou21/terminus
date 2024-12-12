use std::{fmt::Display, io::{self, Write}, thread, time};
use crossterm::event::{self, Event};
use dialoguer::Select;
use textwrap;

pub struct SelectionPrompt<'a, T> {
    prompt: Option<&'a str>,
    options: Vec<T>
}

impl<'a, T: Display + Clone> SelectionPrompt<'a, T> {
    pub fn new(prompt: Option<&'a str>, options: Vec<T> ) -> Self {
        SelectionPrompt { prompt, options}
    }
    
    pub fn select_option(&self) -> String {
        let mut select = Select::new()
            .default(0)
            .max_length(5)
            .items(&self.options)
            .report(false);

        if let Some(prompt) = self.prompt {
            select = select.with_prompt(prompt);
        }

        let selection = select.interact();

        match selection {
            Ok(index) => self.options[index].clone().to_string(),
            Err(_) => panic!("An error occured during option selection. Quitting game.")
        }
    }
}

pub fn type_effect(text_to_type: &str, delay: u64) {
    let wrapped_text = textwrap::wrap(text_to_type, textwrap::termwidth());
    
    for line in wrapped_text {
        for c in line.chars() {
            print!("{c}");
            
            if let Err(_) = io::stdout().flush() {
                println!("{}", text_to_type);
                return;
            }
    
            thread::sleep(std::time::Duration::from_millis(delay));
        }
        println!();
    }

    clear_input_buffer();
}

pub fn delay(delay: u64) {
    thread::sleep(time::Duration::from_millis(delay));
}

fn clear_input_buffer() {
    while event::poll(std::time::Duration::from_millis(0)).unwrap() {
        if let Event::Key(_) = event::read().unwrap() {}
    }
}
