use graphql_client::Response;
use seed::prelude::fetch;

pub fn get_gql_error_message<T>(response: &fetch::Result<Response<T>>, generic_message: &str) -> String {
  match response {
      Ok(Response {
          data: _,
          errors: Some(errors),
      }) => errors
          .last()
          .map(|e| e.message.to_owned())
          .unwrap_or(generic_message.to_string()),
      _ => generic_message.to_string(),
  }
}