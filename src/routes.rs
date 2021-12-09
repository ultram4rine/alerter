use serde::Deserialize;
use serde_yaml::Mapping;

#[derive(Debug, Deserialize)]
pub struct Routes {
    routes: Vec<Route>,
}

#[derive(Debug, Deserialize)]
struct Route {
    filter: Mapping,
    chat_id: i64,
}
