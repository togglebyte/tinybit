use euclid::default::Point2D;

pub struct World<T> {
    zones: Vec<Zone<T>>
}

impl<T> World<T> {
    pub fn new() -> Self {
        Self {
            zones: Vec::new(), 
        }
    }

    pub fn add_zone(&mut self, zone: Zone<T>) {
        self.zones.push(zone);
    }
}

pub struct Zone<T> {
    pub entities: Vec<(usize, Point2D<T>)>
}

impl<T> Zone<T> {
    pub fn new() -> Self {
        Self {
            entities: Vec::new(), 
        }
    }

    pub fn add(&mut self, ent_id: usize, pos: Point2D<T>) {
        self.entities.push((ent_id, pos));
    }
}
