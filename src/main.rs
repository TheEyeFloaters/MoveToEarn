use std::io::{self, Write};
use rand::Rng;

// Define the structure for the game's player character
struct Person {
    x: i32,
    y: i32,
    energy: u32,
    coins: u32,
}

// Define the possible directions the player can move
#[derive(Debug, Clone, Copy)]
enum Direction {
    N, NE, E, SE, S, SW, W, NW, NNW,
}

// Implementation of methods for the player character
impl Person {
    // Function to create a new player character with default values
    fn new() -> Self {
        Person {
            x: 0,
            y: 0,
            energy: 200,
            coins: 0,
        }
    }

    // Function to move the player character in the specified direction
    fn move_direction(&mut self, direction: Direction) {
        match direction {
            Direction::N => self.y += 1,
            Direction::NE => {
                self.x += 1;
                self.y += 1;
            }
            Direction::E => self.x += 1,
            Direction::SE => {
                self.x += 1;
                self.y -= 1;
            }
            Direction::S => self.y -= 1,
            Direction::SW => {
                self.x -= 1;
                self.y -= 1;
            }
            Direction::W => self.x -= 1,
            Direction::NW => {
                self.x -= 1;
                self.y += 1;
            }
            Direction::NNW => self.y += 1,
        }
    }

    // Function to move the player character to a specified position
    fn move_to(&mut self, x: i32, y: i32) {
        // Calculate distance to the target position
        let dx = x - self.x;
        let dy = y - self.y;
        let distance = ((dx * dx + dy * dy) as f64).sqrt() as u32;

        // Check if there's enough energy to make the move
        if distance > self.energy {
            println!("Not enough energy to move that far!");
            return;
        }

        // Deduct energy and update position
        self.energy -= distance;
        self.x = x;
        self.y = y;

        println!("Moved to ({}, {}).", self.x, self.y);
    }

    // Function to fill the energy of the player character
    fn fill_energy(&mut self) {
        if self.coins < 10 {
            println!("Not enough coins to fill energy!");
            return;
        }

        // Deduct coins and fill energy
        self.energy += 200;
        self.coins -= 10;

        println!("Energy filled to {}.", self.energy);
    }

    // Function to get the current position of the player character
    fn get_position(&self) -> (i32, i32) {
        (self.x, self.y)
    }
}

// Function to print the grid representing the game environment
fn print_grid(person: &Person, target_x: i32, target_y: i32) {
    for y in -10..=10 {
        for x in -10..=10 {
            if x == person.x && y == person.y {
                print!("P "); // Player character
            } else if x == target_x && y == target_y {
                print!("T "); // Target position
            } else {
                print!(". "); // Empty space
            }
        }
        println!();
    }
}

// Main function to run the game
fn main() {
    let mut person = Person::new();
    let mut rng = rand::thread_rng();
    let target_x = rng.gen_range(-10..10);
    let target_y = rng.gen_range(-10..10);

    println!("Your starting position is ({}, {}).", person.x, person.y);
    println!("The target position is ({}, {}).", target_x, target_y);

    print_grid(&person, target_x, target_y);

    // Game loop
    loop {
        let mut input = String::new();
        println!("Enter 'move' to move or 'fill' to fill energy:");
        io::stdout().flush().unwrap();
        io::stdin().read_line(&mut input).expect("Failed to read line");

        match input.trim() {
            "move" => {
                // Ask the user to choose a direction
                println!("Enter the direction (N, NE, E, SE, S, SW, W, NW, NNW):");
                let mut direction_input = String::new();
                io::stdin().read_line(&mut direction_input).expect("Failed to read line");
                let direction = match direction_input.trim() {
                    "N" => Direction::N,
                    "NE" => Direction::NE,
                    "E" => Direction::E,
                    "SE" => Direction::SE,
                    "S" => Direction::S,
                    "SW" => Direction::SW,
                    "W" => Direction::W,
                    "NW" => Direction::NW,
                    "NNW" => Direction::NNW,
                    _ => {
                        println!("Invalid direction!");
                        continue;
                    }
                };

                person.move_direction(direction);
                println!("Current position: ({}, {})", person.x, person.y);

                // Check if player reached the target
                if person.get_position() == (target_x, target_y) {
                    println!("You reached the target! You gained 10 coins!");
                    person.coins += 10;
                    break;
                }
            }
            "fill" => person.fill_energy(), // Fill energy if requested
            _ => println!("Invalid input!"), // Invalid input handling
        }

        // Check if player ran out of energy
        if person.energy <= 0 {
            println!("You ran out of energy!");
            break;
        }

        // Print updated grid
        print_grid(&person, target_x, target_y);
    }

    println!("Game over. You collected {} coins.", person.coins);
}