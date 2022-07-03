use std::collections::{VecDeque, vec_deque::IterMut};

pub trait RuledEvent {
    fn is_viable(&self) -> bool;
}

pub struct RuledEventQueue<T: RuledEvent> {
    events: VecDeque<T>
}

impl <T: RuledEvent> RuledEventQueue<T> {
    
    fn add_event(&mut self, event: T) {
        self.events.push_back(event);
    }

    fn read_events(&mut self) -> IterMut<T> {
        self.events.retain(|e| e.is_viable());
        self.events.iter_mut()
    }
}