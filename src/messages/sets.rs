use graphql_client::Response;
use seed::prelude::fetch;

use super::ErrorPayload;
use crate::operations::sets;
use crate::state::entities::Set;

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

pub enum SetsMsg {
    AddSet(AddSetPayload),
    AddSetFetched(
        usize,
        fetch::Result<Response<sets::create_set::ResponseData>>,
    ),
    AddSetFailed(ErrorPayload),
    AddSetSuccess(AddSetSuccessPayload),
    DeleteSet(DeleteSetPayload),
    DeleteSetFetched(
        usize,
        fetch::Result<Response<sets::delete_set::ResponseData>>,
    ),
    DeleteSetFailed(ErrorPayload),
    DeleteSetSuccess(DeleteSetSuccessPayload),
    GetSets(GetSetsPayload),
    GetSetsFetched(
        usize,
        fetch::Result<Response<sets::user_sets::ResponseData>>,
    ),
    GetSetsFailed(ErrorPayload),
    GetSetsSuccess(GetSetsSuccessPayload),
}
