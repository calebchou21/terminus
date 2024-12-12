use crate::utils::{SelectionPrompt, type_effect, delay};

pub fn start_up() {
    let title = "
             _______                  _                 
            |__   __|                (_)                
                | | ___ _ __ _ __ ___  _ _ __  _   _ ___ 
                | |/ _ \\ '__| '_ ` _ \\| | '_ \\| | | / __|
                | |  __/ |  | | | | | | | | | | |_| \\__ \\
                |_|\\___|_|  |_| |_| |_|_|_| |_|\\__,_|___/   
                
        ";
        
    type_effect(&title, 3);

    loop {
        let menu_options = vec!["New Game", "Options", "Quit"];
        let selection = SelectionPrompt::new(None, menu_options);

        match selection.select_option().as_str() {
            "New Game" => play_intro(),
            "Options" => options_menu(),
            "Quit" => std::process::exit(0),
            _ => eprintln!("Invalid Option! This shouldn't happen!"),
        }
    }
}

fn play_intro () {
    type_effect("A candy-colored clown they call the sandman", 50);
    delay(1000);
    type_effect("Tiptoes to my room every night", 30);
    delay(1000);
    type_effect("Just to sprinkle stardust and to whisper", 30);
    delay(1000);
    type_effect("Go to sleep, everything is alright", 30);
    delay(1000);
    println!();
}

fn options_menu() {
    loop {
        let menu_options = vec!["Graphics", "Sound", "Go Back"];
        let selection = SelectionPrompt::new(None, menu_options);

        match selection.select_option().as_str() {
            "Graphics" => break,
            "Sound" => break,
            "Go Back" => break, // Exits the loop and goes back to the previous menu
            _ => eprintln!("Invalid Option! This shouldn't happen!"),
        }
    }
}