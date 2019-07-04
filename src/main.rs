#![feature(try_from)]
mod node;

use futures::prelude::*;
use node::*;

// Manual -- note, fields should be private
#[derive(Clone)]
pub struct Entry {
    state: i32,
}

// Manual -- the internal types should be structs or new types
impl Node<i32> for Entry {
    fn state(&self) -> &i32 {
        &self.state
    }

    fn trace(&self) -> Trace {
        Trace {}
    }

    // Entry node evaluates to true??
    fn eval(_previous: &impl Node<i32>) -> bool {
        true
    }

    fn action(&mut self) -> Result<NodeState, Error> {
        // poll internal future
        if self.state == 5 {
            Ok(NodeState::Ready)
        } else {
            self.state += 1;
            Ok(NodeState::NotReady)
        }
    }
}

// Optional constructor for entry point, for fields should be private for best practice
impl Entry {
    fn new(initial: i32) -> Entry {
        Entry { state: initial }
    }
}

// generated
pub enum States {
    Second(Second),
}

// Generated
impl Future for Entry {
    type Item = States;
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.action() {
            Ok(NodeState::Ready) => {
                if let Ok(next) = Second::transition(self) {
                    Ok(futures::Async::Ready(States::Second(next)))
                } else {
                    // Check next possible transition instead of error, only until no further transitions 
                    Err(Error {})
                }
            }
            Ok(NodeState::NotReady) => Ok(futures::Async::NotReady),
            Err(e) => Err(Error {}),
        }
    }
}

// Manual -- fields should be private
pub struct Second {
    state: i32,
}

// Manual
impl Node<i32> for Second {
    fn state(&self) -> &i32 {
        &self.state
    }

    fn trace(&self) -> Trace {
        Trace {}
    }

    fn eval(previous: &impl Node<i32>) -> bool {
        *previous.state() > 5
    }

    fn action(&mut self) -> Result<NodeState, Error> {
        // poll internal future
            Ok(NodeState::Ready)
    }
}

// Generated -- Second is an exit node, so future returns nothing?
impl Future for Second {
    type Item = ();
    type Error = Error;

    fn poll(&mut self) -> Result<Async<Self::Item>, Self::Error> {
        match self.action() {
            Ok(NodeState::Ready) => Ok(futures::Async::Ready(())),
            Ok(NodeState::NotReady) => Ok(futures::Async::NotReady),
            Err(e) => Err(Error {}),
        }
    }
}

// Manual type conversion
impl From<Entry> for Second {
    fn from(previous: Entry) -> Self {
        Second {
            state: *previous.state(),
        }
    }
}

// Generated -- can play with the ergonomics here
impl Transition<Entry> for Second {
    type State = i32;
    type Next = Second;
}

fn main() {
    let mut entry = Entry::new(0);
    entry.poll();
    entry.poll();
    entry.poll();
    entry.poll();
    entry.poll(); // state = 5
                  //let next = Second::try_from(entry);
                  //assert!(next.is_ok());
}
