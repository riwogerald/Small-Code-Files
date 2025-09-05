use std::collections::HashMap;
use std::io;

#[derive(Debug, Clone)]
struct Item {
    name: String,
    description: String,
    usable: bool,
}

impl Item {
    fn new(name: &str, description: &str, usable: bool) -> Self {
        Item {
            name: name.to_string(),
            description: description.to_string(),
            usable,
        }
    }
}

#[derive(Debug, Clone)]
struct Room {
    name: String,
    description: String,
    exits: HashMap<String, String>,
    items: Vec<Item>,
    visited: bool,
}

impl Room {
    fn new(name: &str, description: &str) -> Self {
        Room {
            name: name.to_string(),
            description: description.to_string(),
            exits: HashMap::new(),
            items: Vec::new(),
            visited: false,
        }
    }

    fn add_exit(&mut self, direction: &str, room_id: &str) {
        self.exits.insert(direction.to_string(), room_id.to_string());
    }

    fn add_item(&mut self, item: Item) {
        self.items.push(item);
    }

    fn remove_item(&mut self, item_name: &str) -> Option<Item> {
        if let Some(pos) = self.items.iter().position(|item| item.name.eq_ignore_ascii_case(item_name)) {
            Some(self.items.remove(pos))
        } else {
            None
        }
    }

    fn describe(&self) -> String {
        let mut description = format!("📍 {}\n{}", self.name, self.description);
        
        if !self.items.is_empty() {
            description.push_str("\n\n🎒 Items here:");
            for item in &self.items {
                description.push_str(&format!("\n  • {} - {}", item.name, item.description));
            }
        }

        if !self.exits.is_empty() {
            description.push_str("\n\n🚪 Exits:");
            for (direction, _) in &self.exits {
                description.push_str(&format!("\n  • {}", direction));
            }
        }

        description
    }
}

#[derive(Debug)]
struct Player {
    current_room: String,
    inventory: Vec<Item>,
    health: i32,
    score: i32,
}

impl Player {
    fn new(starting_room: &str) -> Self {
        Player {
            current_room: starting_room.to_string(),
            inventory: Vec::new(),
            health: 100,
            score: 0,
        }
    }

    fn add_item(&mut self, item: Item) {
        println!("✓ You picked up: {}", item.name);
        self.inventory.push(item);
    }

    fn has_item(&self, item_name: &str) -> bool {
        self.inventory.iter().any(|item| item.name.eq_ignore_ascii_case(item_name))
    }

    fn use_item(&mut self, item_name: &str) -> Option<Item> {
        if let Some(pos) = self.inventory.iter().position(|item| item.name.eq_ignore_ascii_case(item_name)) {
            let item = self.inventory.remove(pos);
            if item.usable {
                Some(item)
            } else {
                println!("You can't use that item.");
                self.inventory.insert(pos, item);
                None
            }
        } else {
            None
        }
    }

    fn show_inventory(&self) {
        if self.inventory.is_empty() {
            println!("🎒 Your inventory is empty.");
        } else {
            println!("🎒 Inventory:");
            for (i, item) in self.inventory.iter().enumerate() {
                let usable_text = if item.usable { " (usable)" } else { "" };
                println!("  {}. {} - {}{}", i + 1, item.name, item.description, usable_text);
            }
        }
    }

    fn show_status(&self) {
        println!("👤 Player Status:");
        println!("  Health: {}/100", self.health);
        println!("  Score: {}", self.score);
        println!("  Items: {}", self.inventory.len());
    }
}

struct Game {
    rooms: HashMap<String, Room>,
    player: Player,
    game_over: bool,
    commands: Vec<String>,
}

impl Game {
    fn new() -> Self {
        let mut game = Game {
            rooms: HashMap::new(),
            player: Player::new("entrance"),
            game_over: false,
            commands: vec![
                "look".to_string(),
                "go <direction>".to_string(),
                "take <item>".to_string(),
                "use <item>".to_string(),
                "inventory".to_string(),
                "status".to_string(),
                "help".to_string(),
                "quit".to_string(),
            ],
        };

        game.setup_world();
        game
    }

    fn setup_world(&mut self) {
        // Create rooms
        let mut entrance = Room::new("Entrance Hall", 
            "You stand in a grand entrance hall with marble floors and high ceilings. \
             Dust particles dance in the sunlight streaming through tall windows.");
        entrance.add_exit("north", "library");
        entrance.add_exit("east", "kitchen");
        entrance.add_item(Item::new("key", "A rusty old key", true));

        let mut library = Room::new("Library", 
            "Towering bookshelves line the walls of this quiet library. \
             Ancient tomes and scrolls are scattered across wooden tables.");
        library.add_exit("south", "entrance");
        library.add_exit("west", "study");
        library.add_item(Item::new("book", "A leather-bound spellbook", true));
        library.add_item(Item::new("candle", "A flickering candle", false));

        let mut kitchen = Room::new("Kitchen", 
            "A rustic kitchen with copper pots hanging from the ceiling. \
             The smell of herbs and spices fills the air.");
        kitchen.add_exit("west", "entrance");
        kitchen.add_exit("north", "pantry");
        kitchen.add_item(Item::new("bread", "Fresh baked bread", true));

        let mut study = Room::new("Study", 
            "A cozy study with a large oak desk covered in papers and quills. \
             A fireplace crackles warmly in the corner.");
        study.add_exit("east", "library");
        study.add_item(Item::new("scroll", "An ancient scroll with mysterious writing", true));

        let mut pantry = Room::new("Pantry", 
            "A small storage room filled with jars, sacks, and preserved foods. \
             It's quite dark and cramped in here.");
        pantry.add_exit("south", "kitchen");
        pantry.add_item(Item::new("potion", "A glowing health potion", true));

        // Add rooms to the game
        self.rooms.insert("entrance".to_string(), entrance);
        self.rooms.insert("library".to_string(), library);
        self.rooms.insert("kitchen".to_string(), kitchen);
        self.rooms.insert("study".to_string(), study);
        self.rooms.insert("pantry".to_string(), pantry);
    }

    fn process_command(&mut self, input: &str) {
        let parts: Vec<&str> = input.trim().split_whitespace().collect();
        
        if parts.is_empty() {
            return;
        }

        let command = parts[0].to_lowercase();
        
        match command.as_str() {
            "look" | "l" => self.look_around(),
            "go" | "move" => {
                if parts.len() > 1 {
                    self.go(parts[1]);
                } else {
                    println!("Go where? Try 'go north' or 'go east'.");
                }
            }
            "take" | "get" => {
                if parts.len() > 1 {
                    self.take_item(parts[1]);
                } else {
                    println!("Take what? Try 'take key' or 'take book'.");
                }
            }
            "use" => {
                if parts.len() > 1 {
                    self.use_item(parts[1]);
                } else {
                    println!("Use what? Try 'use key' or 'use potion'.");
                }
            }
            "inventory" | "inv" | "i" => self.player.show_inventory(),
            "status" | "stats" => self.player.show_status(),
            "help" | "h" => self.show_help(),
            "quit" | "exit" | "q" => {
                println!("Thanks for playing! Goodbye! 👋");
                self.game_over = true;
            }
            "north" | "south" | "east" | "west" | "up" | "down" => {
                self.go(&command);
            }
            _ => {
                println!("I don't understand that command. Type 'help' for available commands.");
            }
        }
    }

    fn look_around(&mut self) {
        if let Some(room) = self.rooms.get_mut(&self.player.current_room) {
            room.visited = true;
            println!("{}", room.describe());
        }
    }

    fn go(&mut self, direction: &str) {
        let current_room_id = self.player.current_room.clone();
        
        if let Some(room) = self.rooms.get(&current_room_id) {
            if let Some(new_room_id) = room.exits.get(direction) {
                self.player.current_room = new_room_id.clone();
                println!("🚶 You go {}.", direction);
                self.look_around();
                
                // Award points for exploring new rooms
                if let Some(new_room) = self.rooms.get(&self.player.current_room) {
                    if !new_room.visited {
                        self.player.score += 10;
                        println!("🎉 +10 points for discovering a new area!");
                    }
                }
            } else {
                println!("You can't go {} from here.", direction);
            }
        }
    }

    fn take_item(&mut self, item_name: &str) {
        let current_room_id = self.player.current_room.clone();
        
        if let Some(room) = self.rooms.get_mut(&current_room_id) {
            if let Some(item) = room.remove_item(item_name) {
                self.player.score += 5;
                println!("🎉 +5 points for finding an item!");
                self.player.add_item(item);
            } else {
                println!("There's no '{}' here.", item_name);
            }
        }
    }

    fn use_item(&mut self, item_name: &str) {
        if let Some(item) = self.player.use_item(item_name) {
            match item.name.as_str() {
                "potion" => {
                    self.player.health = (self.player.health + 50).min(100);
                    self.player.score += 15;
                    println!("🧪 You drink the health potion and feel refreshed! (+50 health, +15 points)");
                }
                "key" => {
                    if self.player.current_room == "study" {
                        println!("🗝️  You use the key to unlock a secret compartment!");
                        if let Some(study) = self.rooms.get_mut("study") {
                            study.add_item(Item::new("treasure", "A chest full of gold coins!", false));
                            self.player.score += 50;
                            println!("🎉 +50 points for solving the puzzle!");
                        }
                    } else {
                        println!("The key doesn't seem useful here.");
                        self.player.inventory.push(item); // Put it back
                    }
                }
                "book" => {
                    println!("📖 You read the spellbook and learn a new spell! (+20 points)");
                    self.player.score += 20;
                }
                "bread" => {
                    self.player.health = (self.player.health + 20).min(100);
                    println!("🍞 You eat the bread and feel a bit better! (+20 health)");
                }
                "scroll" => {
                    println!("📜 The scroll reveals the location of hidden treasure!");
                    println!("💡 Hint: Try using the key in the study...");
                    self.player.score += 10;
                }
                _ => {
                    println!("You can't figure out how to use the {}.", item.name);
                    self.player.inventory.push(item); // Put it back
                }
            }
        } else {
            println!("You don't have a '{}'.", item_name);
        }
    }

    fn show_help(&self) {
        println!("🆘 Available Commands:");
        for command in &self.commands {
            println!("  • {}", command);
        }
        println!("\nShort forms: l (look), i (inventory), h (help), q (quit)");
        println!("You can also just type direction names: north, south, east, west");
    }

    fn check_win_condition(&self) -> bool {
        self.player.has_item("treasure")
    }

    fn run(&mut self) {
        println!("🏰 Welcome to the Mysterious Manor!");
        println!("===================================");
        println!("You find yourself in an old, mysterious manor house.");
        println!("Explore the rooms, collect items, and uncover the secrets within!");
        println!("Type 'help' for available commands.\n");

        self.look_around();

        while !self.game_over {
            print!("\n> ");
            io::Write::flush(&mut io::stdout()).unwrap();

            let mut input = String::new();
            io::stdin().read_line(&mut input).expect("Failed to read input");
            
            let input = input.trim();
            if input.is_empty() {
                continue;
            }

            self.process_command(input);

            // Check win condition
            if self.check_win_condition() {
                println!("\n🎊 Congratulations! You found the treasure!");
                println!("🏆 Final Score: {} points", self.player.score);
                println!("Thanks for playing!");
                self.game_over = true;
            }

            // Check lose condition
            if self.player.health <= 0 {
                println!("\n💀 Game Over! Your health reached zero.");
                println!("🏆 Final Score: {} points", self.player.score);
                self.game_over = true;
            }
        }
    }
}

fn main() {
    let mut game = Game::new();
    game.run();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_room_creation() {
        let room = Room::new("Test Room", "A test room");
        assert_eq!(room.name, "Test Room");
        assert_eq!(room.description, "A test room");
        assert!(!room.visited);
    }

    #[test]
    fn test_item_management() {
        let mut room = Room::new("Test Room", "A test room");
        let item = Item::new("test_item", "A test item", true);
        
        room.add_item(item.clone());
        assert_eq!(room.items.len(), 1);
        
        let removed = room.remove_item("test_item");
        assert!(removed.is_some());
        assert_eq!(room.items.len(), 0);
    }

    #[test]
    fn test_player_inventory() {
        let mut player = Player::new("test_room");
        let item = Item::new("test_item", "A test item", true);
        
        player.add_item(item);
        assert_eq!(player.inventory.len(), 1);
        assert!(player.has_item("test_item"));
    }
}