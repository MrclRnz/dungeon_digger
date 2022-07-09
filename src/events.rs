use std::collections::{vec_deque::IterMut, VecDeque};

pub trait RuledEvent {
    fn is_viable(&self) -> bool;
}

pub struct RuledEventQueue<T: RuledEvent> {
    events: VecDeque<T>,
}

impl<T: RuledEvent> RuledEventQueue<T> {
    pub fn new() -> Self {
        Self {
            events: VecDeque::new(),
        }
    }

    pub fn add_event(&mut self, event: T) {
        self.events.push_back(event);
    }

    pub fn read_events(&mut self) -> IterMut<T> {
        self.events.retain(|e| e.is_viable());
        self.events.iter_mut()
    }
}