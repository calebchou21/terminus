mod utils;
mod location;

use utils::SelectionPrompt;
use location::Location;

fn main() {
    utils::type_effect("Welcome to the game of the century! Welcome to the game of the century! Welcome to the game of the century! Welcome to the game of the century!Welcome to the game of the century! Welcome to the game of the century! Welcome to the game of the century! Welcome to the game of the century! Welcome to the game of the century! Welcome to the game of the century! Welcome to the game of the century! Welcome to the game of the century! Welcome to the game of the century!", 20);

    let swamp = Location::new("Swamp", "A murky wetland filled with fog.", None, None);
    let forest = Location::new("Forest", "A dense forest teeming with life.", None, None);
    let mountain = Location::new("Mountain", "A towering peak that scrapes the sky.", None, None);

    let moisty_mire = Location::new(
        "Moisty Mire",
        "A swampy marsh in the Fortnite universe.",
        None,
        Some(vec![swamp, forest, mountain]),
    );

    let options = vec!["Explore", "Rest", "Quit"];
    let prompt = SelectionPrompt::new("What would you like to do?", options).select_option();

    if prompt == "Explore" {
        moisty_mire.describe();
        moisty_mire.travel_to();
    } else if prompt == "Rest" {
        println!("You take a moment to rest.");
    } else if prompt == "Quit" {
        println!("Goodbye! Thanks for playing.");
    }
}
