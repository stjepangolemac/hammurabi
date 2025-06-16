use crate::game::state::GameState;

#[derive(Debug, Clone)]
pub enum GameAction {
    BuyLand(u32),
    SellLand(u32),
    PlantAcres(u32),
    FeedPopulation(u32),
}

#[derive(Debug, Clone)]
pub enum ActionResult {
    Success,
    InsufficientGrain,
    InsufficientLand,
    InsufficientPopulation,
}

impl GameState {
    pub fn execute_action(&mut self, action: GameAction) -> ActionResult {
        match action {
            GameAction::BuyLand(acres) => self.buy_land(acres),
            GameAction::SellLand(acres) => self.sell_land(acres),
            GameAction::PlantAcres(acres) => self.plant_acres(acres),
            GameAction::FeedPopulation(bushels) => self.feed_population(bushels),
        }
    }

    fn buy_land(&mut self, acres: u32) -> ActionResult {
        if acres == 0 {
            return ActionResult::Success;
        }

        let cost = acres * self.land_price;
        if cost > self.grain {
            return ActionResult::InsufficientGrain;
        }

        self.grain -= cost;
        self.land += acres;
        ActionResult::Success
    }

    fn sell_land(&mut self, acres: u32) -> ActionResult {
        if acres == 0 {
            return ActionResult::Success;
        }

        if acres > self.land {
            return ActionResult::InsufficientLand;
        }

        self.land -= acres;
        self.grain += acres * self.land_price;
        ActionResult::Success
    }

    fn plant_acres(&mut self, acres: u32) -> ActionResult {
        if acres == 0 {
            self.acres_planted = 0;
            return ActionResult::Success;
        }

        if acres > self.land {
            return ActionResult::InsufficientLand;
        }

        if acres > self.grain {
            return ActionResult::InsufficientGrain;
        }

        if acres > self.population * 10 {
            return ActionResult::InsufficientPopulation;
        }

        self.grain -= acres;
        self.acres_planted = acres;
        ActionResult::Success
    }

    fn feed_population(&mut self, bushels: u32) -> ActionResult {
        if bushels > self.grain {
            return ActionResult::InsufficientGrain;
        }

        self.grain -= bushels;

        let people_fed = bushels / 20;
        if people_fed < self.population {
            self.deaths_starvation = self.population - people_fed;
            self.population = people_fed;
            self.total_deaths += self.deaths_starvation;
        } else {
            self.deaths_starvation = 0;
        }

        ActionResult::Success
    }
}
