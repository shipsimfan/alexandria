fn main() {
    println!("     --- System Paths ---");
    println!(
        "Home Directory: {}",
        alexandria::system::home_dir().unwrap().display()
    );
    println!(
        "Desktop Directory: {}",
        alexandria::system::desktop_dir().unwrap().display()
    );
    println!(
        "Downloads Directory: {}",
        alexandria::system::downloads_dir().unwrap().display()
    );
    println!(
        "Documents Directory: {}",
        alexandria::system::documents_dir().unwrap().display()
    );
    println!(
        "Pictures Directory: {}",
        alexandria::system::pictures_dir().unwrap().display()
    );
    println!(
        "Screenshots Directory: {}",
        alexandria::system::screenshots_dir().unwrap().display()
    );
    println!(
        "Music Directory: {}",
        alexandria::system::music_dir().unwrap().display()
    );
    println!(
        "Videos Directory: {}",
        alexandria::system::videos_dir().unwrap().display()
    );
    println!(
        "Temp Directory: {}",
        alexandria::system::temp_dir().unwrap().display()
    );

    println!();
    println!(
        "Game Data Directory: {}",
        alexandria::system::game_data_dir("Company", "Game")
            .unwrap()
            .display()
    );
    println!(
        "Saved Games Directory: {}",
        alexandria::system::saved_games_dir("Company", "Game")
            .unwrap()
            .display()
    );
    println!(
        "Logs Directory: {}",
        alexandria::system::logs_dir("Company", "Game")
            .unwrap()
            .display()
    );
    println!(
        "Config Directory: {}",
        alexandria::system::config_dir("Company", "Game")
            .unwrap()
            .display()
    );
    println!(
        "Cache Directory: {}",
        alexandria::system::cache_dir("Company", "Game")
            .unwrap()
            .display()
    );
}
