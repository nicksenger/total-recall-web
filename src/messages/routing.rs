use crate::state::routing::Route;

#[derive(Clone)]
pub enum RoutingMsg {
    Navigate(Route),
    Push(Route),
}
