use std::collections::HashMap;

use seed::prelude::Orders;

use crate::messages::{Msg, cache::CacheMsg};

pub struct CacheModel {
  pub cache: HashMap<String, String>,
}

impl CacheModel {
  pub fn new() -> Self {
      Self {
          cache: HashMap::new(),
      }
  }
}

pub fn update(
  action: &Msg,
  model: &mut CacheModel,
  orders: &mut impl Orders<Msg>,
) {
  match action {
      Msg::Cache(CacheMsg::AddToCache(payload)) => {
          model.cache.insert(payload.uri.clone(), payload.path.clone());
      }

      Msg::Cache(CacheMsg::HydrateCache(payload)) => {
          model.cache = payload.cache.clone();
      }

      _ => {}
  }
}
