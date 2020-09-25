use seed::prelude::Orders;

use super::messages::Msg;

pub mod authentication;
pub mod cache;
pub mod entities;
pub mod session;
pub mod ui;

pub struct Model {
    pub authentication: authentication::AuthenticationModel,
    pub cache: cache::CacheModel,
    pub entities: entities::EntitiesModel,
    pub session: session::SessionModel,
    pub ui: ui::UIModel,
}

impl Model {
    pub fn new() -> Self {
        Self {
            authentication: authentication::AuthenticationModel::new(),
            cache: cache::CacheModel::new(),
            entities: entities::EntitiesModel::new(),
            session: session::SessionModel::new(),
            ui: ui::UIModel::new(),
        }
    }
}

pub fn update(action: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    authentication::update(&action, &mut model.authentication, orders);
    cache::update(&action, &mut model.cache, orders);
    entities::update(&action, &mut model.entities, orders);
    session::update(&action, &mut model.session, orders);
    ui::update(&action, &mut model.ui, orders);
}
