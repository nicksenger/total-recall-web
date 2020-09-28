use crate::state::routing::Route;

#[derive(Clone)]
pub enum RoutingMessage {
    Navigate(Route),
    Push(Route),
}
