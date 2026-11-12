use diesel_ext::{db_backend::DbConnector, db_tester::DbTester};
use digivine_api as lib;
use digivine_server::BASE_PATH;
use hyper::StatusCode;
use lib::config::Settings;
use rand::{
    distributions::uniform::{SampleRange, SampleUniform},
    prelude::ThreadRng,
    thread_rng, Rng,
};
use reqwest::blocking::{Client, RequestBuilder};
use std::{fmt::Display, thread};
use tokio::sync::oneshot;

pub fn addr() -> String {
    "127.0.0.1:9999".to_string()
}

pub fn api_addr() -> String {
    format!("http://{}{}", addr(), BASE_PATH)
}

pub fn test_config() -> Settings {
    Settings {
        db: digivine_api::config::Database::Global(DbTester::test_url()),
        mockdata: true,
        server_addr: addr(),
        log_level: "Debug".to_string(),
    }
}

pub struct Server {
    s: Settings,
    shutdown_signal: Option<oneshot::Sender<()>>,
}

impl Server {
    pub fn new(s: Settings) -> Self {
        Self {
            s,
            shutdown_signal: None,
        }
    }

    pub fn start<C: DbConnector>(&mut self) {
        let _ = simplelog::SimpleLogger::init(
            simplelog::LevelFilter::Debug,
            simplelog::Config::default(),
        );
        let mut rt = tokio::runtime::Builder::new()
            .basic_scheduler()
            .enable_all()
            .build()
            .unwrap();
        let conf = self.s.clone();
        let (tx, rx) = oneshot::channel();
        self.shutdown_signal = Some(tx);
        thread::spawn(move || {
            rt.block_on(async move {
                let conf = conf; // move conf from outside to async block
                lib::start_server::<C>(&conf, Some(rx)).await.unwrap();
            });
        });
        let req = reqwest::blocking::Client::new()
            .get(format!("{}/field", api_addr()))
            .query(&[
                ("api-token", "all_readonly"),
                ("offset", "0"),
                ("limit", "1"),
                ("Accept", "application/json"),
            ]);
        thread::sleep(std::time::Duration::from_millis(500));
        let mut attempts = 0;
        while req.try_clone().unwrap().send().is_err() {
            attempts += 1;
            log::warn!("Server is not yet started, attempt: {}", attempts);
            if attempts >= 10 {
                panic!("Failed to start server after {} attempts", attempts);
            }
            thread::sleep(std::time::Duration::from_millis(500 + attempts * 100));
        }
        log::info!("Server started successfully");
    }

    pub fn shutdown(&mut self) {
        let tx = self.shutdown_signal.take().expect("No alive server");
        tx.send(()).unwrap();
    }
}

impl Default for Server {
    fn default() -> Self {
        Self::new(test_config())
    }
}

pub fn array_params<T: ToString>(key: &'static str, values: &[T]) -> Vec<(&'static str, String)> {
    values.iter().map(|v| (key, v.to_string())).collect()
}

/// check whether the status code of the response of the given request is expected
pub fn request_test(
    req: RequestBuilder,
    check: impl Fn(StatusCode) -> bool,
    err_msg: impl Fn(StatusCode) -> String,
) -> () {
    let got = req
        .query(&[("api-token", "all"), ("Accept", "application/json")])
        .send()
        .expect("cound not send request")
        .status();
    assert!(check(got), "{}", err_msg(got))
}

/// Performs a test for posting empty coordinates to a specified endpoint. According to the [Geometry Object definition](https://datatracker.ietf.org/doc/html/rfc7946#section-3.1), GeoJSON processors MAY interpret Geometry objects with empty "coordinates" arrays as null objects. In our implementation, if a null Geo object is allowed by an endpoint, e.g. `Option<Polygon>`, it will be converted into `None`. Otherwise, the request will be rejected with a 400 (Bad Request) error code.
///
/// # Arguments
///
/// * `client` - A reference to the client used to make HTTP requests.
/// * `rng` - A mutable reference to the random number generator.
/// * `end_point` - The endpoint to which the coordinates will be posted.
/// * `id_range` - A range from which the ID for the test will be generated. The ID should have a high likelihood of not already existing in the database (note that this test will not delete the generated test data on the server). If the ID already exists, this test will be ignored.
/// * `post_body` - A function that generates the post body based on the ID.
/// * `should_succeed` - A boolean indicating whether the post operation should succeed.
pub fn post_empty_coordinates_test<T: SampleUniform + Display>(
    client: &Client,
    rng: &mut ThreadRng,
    end_point: String,
    id_range: impl SampleRange<T>,
    post_body: impl Fn(T) -> String,
    should_succeed: bool,
) -> () {
    let id = rng.gen_range(id_range);
    let get_url = format!("{}/{}", end_point, id);
    request_test(
        client.get(&get_url),
        |got| {
            if got != StatusCode::NOT_FOUND {
                log::error!(
                    "Id {} for posting test is found via endpoint {}, skip this test",
                    id,
                    end_point
                )
            }
            true
        },
        |_| String::new(),
    );
    let body = post_body(id);
    request_test(
        client.post(&end_point).body(body.clone()),
        |got| should_succeed == got.is_success(),
        |got| {
            if should_succeed {
                format!(
                    "empty coordinates Post {} to \"{}\" should succeed, but got {}",
                    &body, &end_point, got
                )
            } else {
                format!(
                    "empty coordinates Post {} to \"{}\" should not succeed, but got {}",
                    &body, &end_point, got
                )
            }
        },
    );

    if should_succeed {
        request_test(
            client.get(&get_url),
            |got| got.is_success(),
            |got| {
                format!("data with empty coordinates got from \"{}\" should be returned successfully, but got {}", &get_url,got)
            },
        );
    }
}
