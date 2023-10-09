pub struct EloRating {
    k: f64,
    rating: f64,
}

impl EloRating {
    pub fn new(rating: f64) -> EloRating {
        let k = match rating {
            rating if rating <= 2100.0 => 32.0,
            rating if rating <= 2400.0 => 24.0,
            _ => 16.0,
        };
        EloRating { k, rating }
    }

    pub fn win(&self, opponent: &EloRating) -> EloRating {
        let expected = 1.0 / (1.0 + 10.0_f64.powf((opponent.rating - self.rating) / 400.0));
        let new_rating = self.rating + self.k * (1.0 - expected);
        EloRating::new(new_rating)
    }

    pub fn loss(&self, opponent: &EloRating) -> EloRating {
        let expected = 1.0 / (1.0 + 10.0_f64.powf((opponent.rating - self.rating) / 400.0));
        let new_rating = self.rating + self.k * (0.0 - expected);
        EloRating::new(new_rating)
    }

    pub fn draw(&self, opponent: &EloRating) -> EloRating {
        let expected = 1.0 / (1.0 + 10.0_f64.powf((opponent.rating - self.rating) / 400.0));
        let new_rating = self.rating + self.k * (0.5 - expected);
        EloRating::new(new_rating)
    }
}