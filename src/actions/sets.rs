use super::ErrorPayload;
use crate::state::entities::{Card, Set};

pub struct AddSetPayload {
    pub deck_id: usize,
    pub name: String,
    pub card_ids: Vec<usize>,
}

pub struct AddSetSuccessPayload {
    pub deck_id: usize,
}

pub struct DeleteSetPayload {
    pub set_id: usize,
}

pub struct DeleteSetSuccessPayload {
    pub set_id: usize,
}

pub struct GetSetsPayload {
    pub deck_id: usize,
}

pub struct GetSetsSuccessPayload {
    pub sets: Vec<Set>,
    pub deck_id: usize,
}

pub struct GotoAddSetPayload {
    pub cards: Vec<Card>,
}

pub struct ViewSetDetailsPayload {
    pub set: Set,
}

pub enum SetsAction {
    AddSet(AddSetPayload),
    AddSetFailed(ErrorPayload),
    AddSetSuccess(AddSetSuccessPayload),
    DeleteSet(DeleteSetPayload),
    DeleteSetFailed(ErrorPayload),
    DeleteSetSuccess(DeleteSetSuccessPayload),
    GetSets(GetSetsPayload),
    GetSetsFailed(ErrorPayload),
    GetSetsSuccess(GetSetsSuccessPayload),
    GotoAddSet(GotoAddSetPayload),
    ViewSetDetails(ViewSetDetailsPayload),
}
