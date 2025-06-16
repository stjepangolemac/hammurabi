use rand::Rng;
use crate::game::state::GameState;

#[derive(Debug, Clone)]
pub enum RandomEvent {
    Harvest,
    Rats,
    Plague,
    Immigration,
}

#[derive(Debug, Clone)]
pub struct EventOutcome {
    pub event: RandomEvent,
    pub description: String,
    pub impact: i32,
}

impl GameState {
    pub fn process_year_events(&mut self) -> Vec<EventOutcome> {
        let mut outcomes = Vec::new();

        // Harvest
        let harvest_outcome = self.process_harvest();
        outcomes.push(harvest_outcome);

        // Rats (40% chance)
        if self.rng.gen_range(0..100) < 40 {
            let rats_outcome = self.process_rats();
            outcomes.push(rats_outcome);
        }

        // Immigration
        let immigration_outcome = self.process_immigration();
        outcomes.push(immigration_outcome);

        // Plague (15% chance)
        if self.rng.gen_range(0..100) < 15 {
            let plague_outcome = self.process_plague();
            outcomes.push(plague_outcome);
        }

        outcomes
    }

    fn process_harvest(&mut self) -> EventOutcome {
        self.harvest_yield = self.rng.gen_range(1..=5);
        self.grain_harvested = self.acres_planted * self.harvest_yield;
        self.grain += self.grain_harvested;

        EventOutcome {
            event: RandomEvent::Harvest,
            description: format!(
                "THY HARVEST YIELDED {} BUSHELS PER ACRE, TOTAL: {} BUSHELS",
                self.harvest_yield, self.grain_harvested
            ),
            impact: self.grain_harvested as i32,
        }
    }

    fn process_rats(&mut self) -> EventOutcome {
        let damage_percent = self.rng.gen_range(10..=30);
        self.grain_eaten_by_rats = self.grain * damage_percent / 100;
        self.grain -= self.grain_eaten_by_rats;

        EventOutcome {
            event: RandomEvent::Rats,
            description: format!(
                "RATS INFERNAL DEVOURED {} BUSHELS OF THY GRAIN!",
                self.grain_eaten_by_rats
            ),
            impact: -(self.grain_eaten_by_rats as i32),
        }
    }

    fn process_plague(&mut self) -> EventOutcome {
        let deaths = self.population / 2;
        self.deaths_plague = deaths;
        self.population -= deaths;
        self.total_deaths += deaths;

        EventOutcome {
            event: RandomEvent::Plague,
            description: format!("A GREAT PESTILENCE HATH SWEPT THY KINGDOM! HALF THY SUBJECTS PERISHED!"),
            impact: -(deaths as i32),
        }
    }

    fn process_immigration(&mut self) -> EventOutcome {
        if self.deaths_starvation > 0 {
            self.new_citizens = 0;
            return EventOutcome {
                event: RandomEvent::Immigration,
                description: "NO SOULS DARE ENTER THY STARVING KINGDOM".to_string(),
                impact: 0,
            };
        }

        let base_immigration = (20 * self.land + self.grain) / (100 * self.population) + 1;
        self.new_citizens = base_immigration.min(50);
        self.population += self.new_citizens;

        EventOutcome {
            event: RandomEvent::Immigration,
            description: format!("{} SOULS CAME TO DWELL IN THY KINGDOM", self.new_citizens),
            impact: self.new_citizens as i32,
        }
    }
}