use euclid::default::Point2D;

pub type EntityId = usize;

pub struct Entities<T> {
    inner: Vec<Box<dyn Entity<T>>>,
}

impl<T> Entities<T> {
    pub fn with_capacity(cap: usize) -> Self {
        Self {
            inner: Vec::with_capacity(cap),
        }
    }

    pub fn get(&self, index: EntityId) -> &Box<dyn Entity<T>> {
        &self.inner[index]
    }

    pub fn get_mut<U>(&mut self, index: EntityId) { //-> Box<U>{
        //-> &mut Box<dyn Entity<T>> {
        let val = &mut self.inner[index];

        Box::downcast::<U>(val).unwrap();
    }

    pub fn push(&mut self, ent: Box<dyn Entity<T>>) -> EntityId {
        let id = self.inner.len();
        self.inner.push(ent);
        id
    }
}

pub trait Entity<T> {
    fn pixel(&self) -> char;
    fn position(&self) -> Point2D<T>;
}
