// TODO: racer an sich machen, distance beide
// hund extra pace etc.
#[derive(Clone, PartialEq)]
pub struct Horse {
    disabled: bool,
    distance: f64,
}

impl Horse {
    pub fn new() -> Self {
        Horse {
            disabled: false,
            distance: 0 as f64,
        }
    }

    pub fn set_distance(&mut self, distance: f64) {
        self.distance = distance;
    }

    pub fn add_to_distance(&mut self, step: f64) {
        self.set_distance(self.distance + step);
    }

    pub fn get_distance(&self) -> f64 {
        self.distance
    }
}
