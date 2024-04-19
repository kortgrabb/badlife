
pub struct Character {
    name: String,
    age: u32,
    pub stats: Stats,
    pub sickness: SicknessStats,
}

pub struct Stats {
    health: f32,
    happiness: f32,
    looks: f32,
    intellect: f32,
    strength: f32,
    willpower: f32,
    luck: f32,
}

impl Stats {
    pub fn new() -> Self {
        Stats {
            health: 0.0,
            happiness: 0.0,
            looks: 0.0,
            intellect: 0.0,
            strength: 0.0,
            willpower: 0.0,
            luck: 0.0,
        }
    }

    pub fn new_random() -> Self {
        let mut stats = Stats::new();

        stats.health = (rand::random::<f32>() * 0.2) + 0.8;
        stats.happiness = 1.0;
        stats.intellect = (rand::random::<f32>() * 0.8) + 0.2;
        stats.strength = rand::random::<f32>();
        stats.intellect = (rand::random::<f32>() * 0.8) + 0.2;
        stats.luck = rand::random::<f32>();
        stats.willpower = (rand::random::<f32>() * 0.6) + 0.4;
        stats.looks = (rand::random::<f32>() * 0.8) + 0.2;

        stats
    }

    pub fn generate_stat_bar(stat: f32) -> String {
        let mut bar = String::new();
        let bar_length = (10.0 * stat) as usize;
        let total_length = 10;

        for _ in 0..bar_length {
            bar.push('#');
        }

        for _ in bar_length..total_length {
            bar.push('-');
        }

        bar
    }

    pub fn display(&self) {
        println!("===== Stats ======");
        
        println!("Health:     {}", Stats::generate_stat_bar(self.health));
        println!("Happiness:  {}", Stats::generate_stat_bar(self.happiness));
        println!("Looks:      {}", Stats::generate_stat_bar(self.looks));
        println!("Intellect:  {}", Stats::generate_stat_bar(self.intellect));

        println!("==================");
    }
    
}

pub struct SicknessStats {
    pub sick: bool,
    pub days_sick: u32,
    pub death_chance: f32,
    pub severity: f32,
}

impl SicknessStats {
    pub fn new() -> Self {
        SicknessStats {
            sick: false,
            days_sick: 0,
            death_chance: 0.0,
            severity: 0.0,
        }
    }
}

impl Character {
    pub fn new(name: String) -> Self {
        Character {
            name,
            age: 0,

            sickness: SicknessStats::new(),
            stats: Stats::new_random(),
        }
    }

    pub fn age(&mut self) {
        self.age += 1;
    }

    pub fn display(&self) {
        println!("{} is now {} years old.", self.name, self.age);
        self.stats.display();
    }
}
