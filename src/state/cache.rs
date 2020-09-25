use std::collections::HashMap;

use seed::prelude::Orders;

use crate::actions::{GlobalAction, cache::CacheAction};

pub struct CacheModel {
  cache: HashMap<String, String>,
}

impl CacheModel {
  pub fn new() -> Self {
      Self {
          cache: HashMap::new(),
      }
  }
}

pub fn update(
  action: &GlobalAction,
  model: &mut CacheModel,
  orders: &mut impl Orders<GlobalAction>,
) {
  match action {
      GlobalAction::Cache(CacheAction::AddToCache(payload)) => {
          model.cache.insert(payload.uri.clone(), payload.path.clone());
      }

      GlobalAction::Cache(CacheAction::HydrateCache(payload)) => {
          model.cache = payload.cache.clone();
      }

      _ => {}
  }
}
