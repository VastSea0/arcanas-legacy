
mod level;
mod players;

use players::{Player, Class};
use std::io::{self, Write}; // std::io::stdin, std::io::stdout için import yapıyoruz
use std::process::Command;
use std::thread::sleep;
use std::time::Duration;


fn clear_terminal() {
    #[cfg(target_os = "windows")]
    {
        Command::new("cmd")
            .args(["/C", "cls"])
            .output()
            .expect("Failed to clear terminal");
    }

    #[cfg(not(target_os = "windows"))]
    {
        Command::new("clear")
            .output()
            .expect("Failed to clear terminal");
    }
}



fn main() {
    println!("Welcome to...");
    sleep(Duration::from_secs(1));
    println!(r#"
     _                              _       _
    / \   _ __ ___ __ _ _ __   __ _( )___  | |    ___  __ _  __ _  ___ _   _
   / _ \ | '__/ __/ _` | '_ \ / _` |// __| | |   / _ \/ _` |/ _` |/ __| | | |
  / ___ \| | | (_| (_| | | | | (_| | \__ \ | |__|  __/ (_| | (_| | (__| |_| |
 /_/   \_\_|  \___\__,_|_| |_|\__,_| |___/ |_____\___|\__, |\__,_|\___|\__, |
                                                      |___/            |___/
 _____ _          __        ___     _                       __   _____ _
|_   _| |__   ___ \ \      / / |__ (_)___ _ __   ___ _ __ __\ \ / / __| | __ _ _ __
  | | | '_ \ / _ \ \ \ /\ / /| '_ \| / __| '_ \ / _ \ '__/ __\ V /| __| |/ _` | '__|
  | | | | | |  __/  \ V  V / | | | | \__ \ |_) |  __/ |  \__ \| | | |__| | (_| | |
  |_| |_| |_|\___|   \_/\_/  |_| |_|_|___/ .__/ \___|_|  |___/|_| |_____|_\__,_|_|
                                         |_|
"#);
    sleep(Duration::from_secs(2));
    println!("Prepare yourself for a journey of mystery and magic...");
    sleep(Duration::from_secs(2));
    game();
}

fn game() {
    println!("Welcome to Arcana's Legacy: The Whispers of Elara");
    println!("In this game, you will uncover ancient secrets and face unexpected challenges.");
    println!("Your choices will shape the story and determine your fate.");
    println!("\nPlease select your player (1, 2, or 3): ");

    // Kullanıcıdan giriş al
    io::stdout().flush().expect("Failed to flush stdout");
    let mut player_choice = String::new();
    io::stdin().read_line(&mut player_choice).expect("Failed to read line");

    // Giriş verisini sayıya dönüştür
    let player_choice: i32 = player_choice.trim().parse().unwrap_or(1);

    // Seçimi işleme
    match player_choice {
        1 => select_char(1),
        2 => select_char(2),
        3 => select_char(3),
        _ => select_char(1), // Varsayılan olarak 1 seçilir
    }
}

fn select_char(num: i32) {
    let gladius_player = Player::new(Class::Gladius);
    let arcana_player = Player::new(Class::Arcana);
    let aegis_player = Player::new(Class::Aegis);
    match num {
        // 1 => {
        //     println!("Selected character: 1")
        // }, Burada 1. karakter seçimi yapmayız çünkü zaten 2. veya 3. karakter
        // Seçilmediyse bu durumda 1. karakter seçilmiş demektir.
        2 => {
            println!("Selected character: 2");
            println!("Arcana Player: {}", arcana_player);
            play_with(arcana_player);

        },
        3 => {
            println!("Selected character: 3");
            println!("Aegis Player: {}", aegis_player);
            play_with(aegis_player);

        },
        _ => {
            println!("Selected character: 1");
            println!("Gladius Player: {}", gladius_player);
            play_with(gladius_player);

        },
    }
}

fn play_with(char: Player) {
    println!("Playing Character: {}", char.name);
    clear_terminal();
    if char.name == "Arcana" {
        println!("Now you're going to home");
        println!("...");
        sleep(Duration::new(2, 0));
        println!("You're finally home, you should get some rest. It's been a tiring day.");
        sleep(Duration::new(4, 0));
        println!("Hello from a new day. It looks like the sun has just come out. I think it's going to be a beautiful day.");
        sleep(Duration::new(3, 0));
        println!("Wait a minute????");
        sleep(Duration::new(1, 0));
        println!("I wonder if something happened at night when the house was more cluttered than it should be.");
        sleep(Duration::new(3, 0));
        println!("You were so tired yesterday, I guess you didn't hear that.");
        sleep(Duration::new(4, 0));
        println!("It can't have been a burglar, nothing seems to have been stolen... STOP");
        sleep(Duration::new(3, 0));
        println!(
            "What's on that paper on the floor?"
        );
        sleep(Duration::new(3, 0));
        println!("                  Dear Finder,");
        sleep(Duration::new(2, 0));
        println!("                                  If you are reading this note, it means that something unexpected has happened. My name is Elara, and I used to live in this house many years ago. I’m sorry for the sudden intrusion into your peaceful home.");
        sleep(Duration::new(4, 0));
        println!("                                  I left behind a special amulet that has been in my family for generations. It holds great power, and I had to hide it here to keep it safe from those who sought to misuse it. If you have found this note, it means that the amulet has been discovered, or someone is looking for it.");
        sleep(Duration::new(6, 0));
        println!("                                  Please be cautious. There are others who might come looking for it. The amulet is hidden beneath the floorboards in the north corner of the attic. It is vital to keep it safe, as it holds the key to a great secret that must not fall into the wrong hands.");
        sleep(Duration::new(7, 0));
        println!("                                  If you decide to take on this responsibility, know that you are not alone. I’ve left behind clues that will guide you. Trust in your instincts and stay vigilant.");
        sleep(Duration::new(5, 0));
        println!("                                  Thank you for your bravery and discretion. May fortune be with you.");
        sleep(Duration::new(3, 0));
        println!("                 Elara");
        sleep(Duration::new(2, 0));

        println!("You feel a chill run down your spine as you finish reading the note.");
        sleep(Duration::new(3, 0));
        println!("Your mind races with questions. Who was Elara? What is this amulet? And why does it feel like your ordinary life has suddenly become extraordinarily complicated?");
        sleep(Duration::new(5, 0));
        println!("You glance towards the stairs leading to the attic. Should you go and look for the amulet now?");
        sleep(Duration::new(3, 0));
        println!("As you contemplate your next move, you hear a sudden knock at the door. Your heart skips a beat.");
        sleep(Duration::new(3, 0));
        println!("Who could it be? Is it just a coincidence, or could it be someone looking for the amulet?");
        sleep(Duration::new(3, 0));
        println!("You have to make a decision:");
        sleep(Duration::new(2, 0));
        println!("1. Go to the attic to look for the amulet");
        sleep(Duration::new(1, 0));
        println!("2. Answer the door");
        sleep(Duration::new(1, 0));
        println!("3. Pretend you're not home and try to gather more information");

        let mut choice = String::new();
        io::stdin().read_line(&mut choice).expect("Failed to read line");
        let choice: u32 = choice.trim().parse().expect("Please type a number!");

        match choice {
            1 => {
                println!("You decide to quickly check the attic before dealing with whoever is at the door.");
            },
            2 => {
                println!("Taking a deep breath, you approach the door and open it.");
            },
            3 => {
                println!("You remain silent, your heart pounding as you try to figure out what to do next.");
            },
            _ => println!("Invalid choice. The story ends here."),
        }
    }

    else if char.name == "Aegis" {
        println!("Now you're going to shop");
        println!("...")

    } else { // Gladius
        println!("Now you're going to shop");
        println!("...");

    }



}