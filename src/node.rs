use futures::prelude::*;

pub struct Trace {}

pub struct Error {}
pub struct TransitionError {}

pub enum NodeState {
    NotReady,
    Ready,
}

pub trait Node<T>: Future {
    fn state(&self) -> &T;
    fn trace(&self) -> Trace;
    fn eval(previous: &impl Node<T>) -> bool;
    fn action(&mut self) -> Result<NodeState, Error>; // Update internal state appropriately, such that this just needs to return ready or not
}

pub trait Transition<T: Node<Self::State> + Clone> {
    type State;
    type Next;

    fn transition(previous: &T) -> Result<Self::Next, TransitionError>
    where
        Self::Next: From<T>,
    {
        if T::eval(previous) {
            Ok(Self::Next::from(previous.clone()))
        } else {
            Err(TransitionError {})
        }
    }
}
