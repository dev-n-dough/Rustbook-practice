pub trait Messenger{
    fn send(&self, msg: &str);
}

pub struct LimitTracker<'a,T:Messenger>{
    messenger: &'a T, // we are storing a reference to a type which implements the T trait , and since we are using reference in a struct , we must specify lifetime
    value: usize,
    max: usize,
}

impl<'a,T> LimitTracker<'a,T>
where
    T: Messenger
{
    pub fn new(messenger: &'a T, max : usize) -> LimitTracker<'a,T>{
        LimitTracker{
            messenger,
            value:0,
            max
        }
    }

    pub fn set_value(&mut self , value: usize){
        self.value = value; // a mutable ref to self allows you to bring a mut ref to its attributes and do stuff on them

        let percentage = value as f64/self.max as f64;

        if percentage >= 1.0 {
            self.messenger.send("You are over your qouta for today");
        } else if percentage > 0.9 {
            self.messenger.send("You have used up over 90% of your quota");
        } else if percentage > 0.75 {
            self.messenger.send("You have used over 75% of your quota");
        }
    }
}

// we have specified how a messenger for my program should look like. In reality I would be using a messenger which sends a mail,text,etc , but here let us make a mock object which implements the Messenger trait , just to test whether everything is going smoothly.

#[cfg(test)]
mod tests {
    use super::*;
    use std::cell::RefCell;

    struct MockMessenger{
        // messages: Vec<String>,
        messages: RefCell<Vec<String>>,
    }

    impl MockMessenger{
        pub fn new() -> MockMessenger{
            MockMessenger{
                // messages: Vec::new(),
                messages: RefCell::new(Vec::new()),
            }
        }
    }

    impl Messenger for MockMessenger{
        fn send(&self, msg:&str){
            // in an actual messenger object , the message sending action is going to happen here


            // self.messages.push(msg.to_string()); 
            self.messages.borrow_mut().push(msg.to_string());
        }
    }

    #[test]
    fn mock_messenger_works(){
        let mockMessenger = MockMessenger::new();
        let mut limitTracker = LimitTracker::new(&mockMessenger,1000);
        limitTracker.set_value(800);

        assert_eq!(
            mockMessenger.messages.borrow().len(),
            1
        );
    }
}
