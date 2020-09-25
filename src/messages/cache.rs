use std::collections::HashMap;

use super::ErrorPayload;

pub struct AddToCachePayload {
    pub uri: String,
    pub path: String,
}

pub struct FetchImagePayload {
    pub uri: String,
}

pub struct HydrateCachePayload {
    pub cache: HashMap<String, String>,
    pub auth: Option<(String, String)>,
}

pub struct PlayAudioPayload {
    pub uri: String,
}

pub enum CacheMsg {
    AddToCache(AddToCachePayload),
    AddToCacheFailed(ErrorPayload),
    FetchImage(FetchImagePayload),
    FetchImageFailed(ErrorPayload),
    HydrateCache(HydrateCachePayload),
    PlayAudio(PlayAudioPayload),
    PlayAudioFailed(ErrorPayload),
}
