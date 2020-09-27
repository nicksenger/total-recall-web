use seed::prelude::Orders;

use super::{messages::Msg, operations::operate};

pub mod authentication;
pub mod entities;
pub mod session;
pub mod ui;

pub struct Model {
    pub authentication: authentication::AuthenticationModel,
    pub entities: entities::EntitiesModel,
    pub session: session::SessionModel,
    pub ui: ui::UIModel,
}

impl Model {
    pub fn new() -> Self {
        Self {
            authentication: authentication::AuthenticationModel::new(),
            entities: entities::EntitiesModel::new(),
            session: session::SessionModel::new(),
            ui: ui::UIModel::new(),
        }
    }
}

pub fn update(action: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    authentication::update(&action, &mut model.authentication);
    entities::update(&action, &mut model.entities);
    session::update(&action, &mut model.session);
    ui::update(&action, &mut model.ui);
    operate(&action, &model, orders);
}
