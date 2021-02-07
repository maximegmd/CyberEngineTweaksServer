use std::collections::HashMap;
use std::time::SystemTime;
use std::cmp;

#[derive(Debug, Copy, Clone)]
pub struct Statistics{
    pub player_count: u32,
    pub max_player_count: u32
}

pub struct Context {
    pub stats: Statistics,
    players: HashMap::<String, SystemTime>
}

impl Context {
    pub fn new() -> Context {
        Context {
            stats: Statistics{
                player_count: 0,
                max_player_count: 0
            },
            players: HashMap::<String, SystemTime>::new()
        }
    }

    pub fn clean(&mut self){
        self.players.retain(|_, value| {
            match value.elapsed() {
                Ok(n) => n.as_secs() < 120,
                Err(_) => false,
            }
        });

        self.generate_list();
    }

    pub fn update(&mut self, endpoint: String, name: String) {
        let key = format!("{}:{}", endpoint, name);

        self.players.insert(key, SystemTime::now());

        self.generate_list();
    }

    fn generate_list(&mut self) {
        self.stats.player_count = self.players.len() as u32;

        self.stats.max_player_count = cmp::max(self.stats.max_player_count, self.stats.player_count);
    }

    pub fn generate_stats(&self) -> String {
        format!("{{\"current\": {{\"players\": {} }}, \"max\": {{\"players\": {} }} }}",
            self.stats.player_count, self.stats.max_player_count)
    }
}