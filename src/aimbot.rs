pub struct Aimbot {
    target: Option<String>,
}

impl Aimbot {
    pub fn new() -> Self {
        Aimbot { target: None }
    }

    pub fn find_target(&mut self) {
        self.target = Some("EnemyPlayer".to_string());
        println!("Target acquired: {:?}", self.target);
    }

    pub fn aim(&self) {
        if let Some(ref target) = self.target {
            println!("Aiming at {}", target);
        }
    }
}