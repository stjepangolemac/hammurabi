use crate::game::state::GameState;
use crate::messages::MessageTemplates;
use rand::Rng;


impl GameState {
    pub fn process_year_events(&mut self, messages: &mut MessageTemplates) -> Vec<String> {
        let mut outcomes = Vec::new();

        // Harvest
        let harvest_outcome = self.process_harvest(messages);
        outcomes.push(harvest_outcome);

        // Rats (40% chance)
        if self.rng.gen_range(0..100) < 40 {
            let rats_outcome = self.process_rats(messages);
            outcomes.push(rats_outcome);
        }

        // Immigration
        let immigration_outcome = self.process_immigration(messages);
        outcomes.push(immigration_outcome);

        // Plague (15% chance)
        if self.rng.gen_range(0..100) < 15 {
            let plague_outcome = self.process_plague(messages);
            outcomes.push(plague_outcome);
        }

        outcomes
    }

    fn process_harvest(&mut self, messages: &mut MessageTemplates) -> String {
        self.harvest_yield = self.rng.gen_range(1..=5);
        self.grain_harvested = self.acres_planted * self.harvest_yield;
        self.grain += self.grain_harvested;

        messages.harvest_message(self.harvest_yield, self.grain_harvested)
    }

    fn process_rats(&mut self, messages: &mut MessageTemplates) -> String {
        let damage_percent = self.rng.gen_range(10..=30);
        self.grain_eaten_by_rats = self.grain * damage_percent / 100;
        self.grain -= self.grain_eaten_by_rats;

        messages.rats_message(self.grain_eaten_by_rats)
    }

    fn process_plague(&mut self, messages: &mut MessageTemplates) -> String {
        let deaths = self.population / 2;
        self.deaths_plague = deaths;
        self.population -= deaths;
        self.total_deaths += deaths;

        messages.plague_message()
    }

    fn process_immigration(&mut self, messages: &mut MessageTemplates) -> String {
        if self.deaths_starvation > 0 {
            self.new_citizens = 0;
            return messages.no_immigration_message();
        }

        let base_immigration = (20 * self.land + self.grain) / (100 * self.population) + 1;
        self.new_citizens = base_immigration.min(50);
        self.population += self.new_citizens;

        messages.immigration_message(self.new_citizens)
    }
}
