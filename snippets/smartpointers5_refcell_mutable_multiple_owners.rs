pub trait Messenger {
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a, T: Messenger> {
    messenger: &'a T,
    value: usize,
    max: usize
}

impl <'a, T: Messenger> LimitTracker<'a, T> {
    pub fn new(messenger: &T, max: usize) -> LimitTracker<T> {
        LimitTracker {
            messenger,
            value: 0,
            max
        }
    }

    pub fn set_value(&mut self, value: usize) {
        self.value = value;
        let percentage_of_max = self.value as f64 / self.max as f64;

        if percentage_of_max >= 1.0 {
            self.messenger.send("Error: Over quota");
        } else if percentage_of_max > 0.9 {
            self.messenger.send("Warning: 90% of quota");
        } else if percentage_of_max >= 0.75 {
            self.messenger.send("Warning: 75% of quota");
        }
    }
} 

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger {
        sent_messages: RefCell<Vec<String>>
    }

    impl MockMessenger {
        fn new() -> MockMessenger {
            MockMessenger {
                sent_messages: RefCell::new(vec![])
            }
        }
    }

    impl Messenger for MockMessenger {
        fn send(&self, message: &str) {
            // Note how &self is immutable reference
            self.sent_messages.borrow_mut().push(String::from(message));
        }
    }

    #[test]
    fn it_sends_over_75() {
        let mock = MockMessenger::new();
        let mut limit_tracker = LimitTracker::new(&mock, 100);

        limit_tracker.set_value(80);
        // Borrow as immutably
        assert_eq!(mock.sent_messages.borrow().len(), 1);
    }
}