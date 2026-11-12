mod params;
mod server;

use diesel_ext::{
    db_backend::{DbConnector, PoolBackend},
    db_tester::DbTester,
};
use digivine_api::response::Post;
use digivine_api_postgres::field::FieldFilter;
use digivine_server::{
    GetAllFieldsResponse, GetAllTokensResponse, GetUserTokensResponse, PostFieldResponse,
    PostPlantResponse,
};
use params::{FieldParams, ServerClientTestableResp, TokenParams};
use rand::{prelude::ThreadRng, thread_rng};
use reqwest::blocking::Client;
use serde_json::json;
use server::{api_addr, post_empty_coordinates_test, Server};

#[ignore]
#[test]
pub fn server_client_test() {
    let mut s = Server::default();
    let client = reqwest::blocking::Client::new();
    let mut rng = thread_rng();
    s.start::<PoolBackend>();
    all_server_client_tests(&client, &mut rng);
    s.shutdown();

    s.start::<DbTester>();
    all_server_client_tests(&client, &mut rng);
    s.shutdown();
}

fn all_server_client_tests(client: &Client, rng: &mut ThreadRng) {
    get_fields_test();
    get_tokens_test();
    empty_harvest_test(&client, rng);
    empty_plant_test(&client, rng);
}

impl ServerClientTestableResp for GetAllFieldsResponse {}
pub fn get_fields_test() {
    GetAllFieldsResponse::server_client_test("All Fields", FieldParams::default());

    let bbox = Some(vec![
        8.047680117127902,
        49.110206354547145,
        8.210368988395313,
        49.458770080524246,
    ]);
    let fuzzy = vec!["R", "Mütterle", "Dornfelder"];
    let field_names = vec!["Mütterle"];
    let fruit_names = vec!["Dornfelder", "Mütterle"];
    let filter = FieldFilter::new()
        .with_fuzzy(&fuzzy)
        .with_field_names(&field_names)
        .with_fruit_names(&fruit_names);

    GetAllFieldsResponse::server_client_test(
        "Fields via BBox",
        FieldParams {
            bbox: bbox.as_ref(),
            ..Default::default()
        },
    );

    GetAllFieldsResponse::server_client_test(
        "Fields via keywords",
        FieldParams {
            bbox: None,
            filter: filter.clone(),
        },
    );

    GetAllFieldsResponse::server_client_test(
        "Fields via BBox and keywords",
        FieldParams {
            bbox: bbox.as_ref(),
            filter: filter.clone(),
        },
    );
}

impl ServerClientTestableResp for GetUserTokensResponse {}
impl ServerClientTestableResp for GetAllTokensResponse {}
pub fn get_tokens_test() {
    let keyword_all = Some(vec!["all".to_string()]);

    GetAllTokensResponse::server_client_test("all tokens", TokenParams::default());
    GetAllTokensResponse::server_client_test(
        "tokens contain all",
        TokenParams {
            user_id: None,
            tokenstr_contains: keyword_all.as_ref(),
        },
    );

    GetUserTokensResponse::server_client_test(
        "tokens with user_id=5",
        TokenParams {
            user_id: Some(5),
            ..Default::default()
        },
    );

    GetUserTokensResponse::server_client_test(
        "tokens with user_id=5 and contain all",
        TokenParams {
            user_id: Some(5),
            tokenstr_contains: keyword_all.as_ref(),
        },
    );
}

pub fn empty_harvest_test(client: &Client, rng: &mut ThreadRng) -> () {
    post_empty_coordinates_test(
        client,
        rng,
        format!("{}/harvest/complete", api_addr()),
        100000..200000,
        |id: u32| {
            json!(
            {
              "features" : [],
              "geometry" : {
                "bbox" : [],
                "coordinates" : [],
                "type" : "Polygon"
              },
              "type" : "FeatureCollection",
              "properties" : {
                "task_name" : "Ernte",
                "fruit_name" : "Riesling",
                "plain_name" : "Auf dem Weinberg",
                "total_area" : 5874,
                "effective_total_distance" : 2580,
                "task_id" : id,
                "yield_total_mass" : 7568,
                "average_fructose" : 7,
                "fruit_group_name" : "Traube",
                "average_malic_acid" : 23,
                "plain_size" : 7668,
                "average_tartaric_acid" : 23,
                "average_glucose" : 7,
                "customer_name" : "Julius Kühn-Institut",
                "effective_total_time" : 95,
                "farm_name" : "Geilweilerhof"
              }
            }
            )
            .to_string()
        },
        true,
    )
}

pub fn empty_plant_test(client: &Client, rng: &mut ThreadRng) -> () {
    post_empty_coordinates_test(
        client,
        rng,
        format!("{}/field/167/planter/1/plant", api_addr()),
        100000..200000,
        |id: u32| {
            json!({
              "geometry" : {
                "bbox" : [],
                "coordinates" : [],
                "type" : "Point"
              },
              "type" : "Feature",
              "properties" : {
                "import_key" : "1-example_feld1",
                "row_distance" : 2000,
                "plant_distance" : 1000,
                "deviation" : 12.3456789012345,
                "id" : id,
                "row_number" : 2,
                "speed" : 1.23456789012345,
                "timestamp" : "1970-01-01T00:00:00Z",
                "height" : 123000
              }
            })
            .to_string()
        },
        false,
    )
}
