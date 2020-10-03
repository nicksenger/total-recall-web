use crate::state::routing::Route;

#[derive(Clone)]
pub enum RoutingMsg {
    ModalOpen(bool),
    Navigate(Route),
    Push(Route),
}
