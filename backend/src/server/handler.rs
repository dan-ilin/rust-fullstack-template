use reqwest::StatusCode;
use warp::{Filter, path, Rejection, Reply, reply};

use common::Example;

pub struct Handler {}

impl Handler {
    pub fn build_routes(self) -> impl Filter<Extract=(impl Reply, ), Error=Rejection> + Clone {
        let prefix = path!("api" / "v1" / ..);
        prefix.and(Self::example_route())
    }

    fn example_route() -> impl Filter<Extract=(impl Reply, ), Error=Rejection> + Clone {
        warp::path!("hello" / String).and_then(Self::example_handler)
    }

    async fn example_handler(arg: String) -> Result<impl Reply, Rejection> {
        let example_response = Example {
            string: arg,
            int: 12345,
            float: 67.890,
        };
        Ok(reply::with_status(reply::json(&example_response), StatusCode::OK))
    }
}
