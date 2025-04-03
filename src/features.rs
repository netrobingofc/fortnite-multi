use rand::Rng;

pub fn initialize() {
    println!("Fortnite Multi initialized.");
    setup_cheats();
}

fn setup_cheats() {
    println!("Setting up cheats...");
    let aimbot_enabled = true;
    let esp_enabled = true;
    if aimbot_enabled {
        println!("Aimbot is enabled.");
    }
    if esp_enabled {
        println!("ESP is enabled.");
    }
}

pub async fn check_for_updates() {
    let response = reqwest::get("https://api.github.com/repos/netrobingofc/fortnite-multi/releases/latest")
        .await
        .unwrap()
        .json::<serde_json::Value>()
        .await
        .unwrap();
    let latest_version = response["tag_name"].as_str().unwrap();
    println!("Latest version: {}", latest_version);
}