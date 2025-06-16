#[derive(Debug, Clone)]
pub struct Score {
    pub total_deaths: u32,
    pub death_rate: f32,
    pub acres_per_person: f32,
    pub population_growth: i32,
    pub rating: PerformanceRating,
}

#[derive(Debug, Clone, PartialEq)]
pub enum PerformanceRating {
    Terrible,
    Poor,
    Fair,
    Good,
    Excellent,
}

pub fn evaluate_performance(
    initial_population: u32,
    final_population: u32,
    total_deaths: u32,
    final_land: u32,
) -> Score {
    let death_rate = (total_deaths as f32) / (initial_population as f32 * 10.0) * 100.0;
    let acres_per_person = if final_population > 0 {
        final_land as f32 / final_population as f32
    } else {
        0.0
    };
    let population_growth = final_population as i32 - initial_population as i32;

    let rating = if death_rate > 33.0 || acres_per_person < 7.0 {
        PerformanceRating::Terrible
    } else if death_rate > 20.0 || acres_per_person < 9.0 {
        PerformanceRating::Poor
    } else if death_rate > 10.0 || acres_per_person < 10.0 {
        PerformanceRating::Fair
    } else if death_rate > 3.0 || acres_per_person < 12.0 {
        PerformanceRating::Good
    } else {
        PerformanceRating::Excellent
    };

    Score {
        total_deaths,
        death_rate,
        acres_per_person,
        population_growth,
        rating,
    }
}

impl Score {
    pub fn get_rating_message(&self) -> &'static str {
        match self.rating {
            PerformanceRating::Terrible => {
                "Due to your extreme mismanagement, you have been impeached and thrown out of office! \
                 The people are rallying to lynch you!"
            }
            PerformanceRating::Poor => {
                "Your poor leadership has left the city in bad shape. \
                 The people have lost confidence in your rule."
            }
            PerformanceRating::Fair => {
                "Your performance was adequate. The city survived, \
                 but there is much room for improvement."
            }
            PerformanceRating::Good => {
                "You have done well! The city has prospered under your rule, \
                 and the people respect your leadership."
            }
            PerformanceRating::Excellent => {
                "Fantastic! You are truly a great leader! \
                 The city has thrived beyond all expectations. \
                 Songs will be sung of your reign for generations!"
            }
        }
    }
}