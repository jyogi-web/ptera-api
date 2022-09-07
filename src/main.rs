use aws_sdk_dynamodb::{model::AttributeValue, Client};
use lambda_http::{
    http::{request, Method},
    request::RequestContext,
    run, service_fn, Body, Error, Request, RequestExt, Response,
};
use log::LevelFilter;
use serde::{Deserialize, Serialize};
use serde_json::json;
use simplelog::{CombinedLogger, ConfigBuilder, TermLogger};

#[derive(Debug, Default, Serialize, Deserialize)]
struct RateInfo {
    uuid: String,
    rate: u64,
    name: String,
}

/// This is the main body for the function.
/// Write your code inside it.
/// There are some code example in the following URLs:
/// - https://github.com/awslabs/aws-lambda-rust-runtime/tree/main/examples
async fn function_handler(event: Request) -> Result<Response<Body>, Error> {
    let region = std::env::var("region").unwrap_or_default();

    let shared_config = aws_config::load_from_env().await;
    let client = Client::new(&shared_config);
    let req = client.list_tables().limit(10);
    let list_tables = req.send().await;
    let list_tables = match list_tables {
        Ok(l) => l.table_names.unwrap()[0].clone(),
        Err(e) => e.to_string(),
    };
    // let request = client
    //     .put_item()
    //     .table_name(&list_tables)
    //     .item(
    //         "user_id",
    //         AttributeValue::S(uuid::Uuid::new_v4().to_string()),
    //     )
    //     .item("name", AttributeValue::S("kona".to_string()))
    //     .item("rate", AttributeValue::N(4440.to_string()));
    // request.send().await.unwrap();

    // let item = client.get_item().table_name(&list_tables).key(
    //     "user_id",
    //     AttributeValue::S("99315bb2-c1eb-4875-9080-67f41281ea7c".to_string()),
    // );
    // let item = item.send().await?;
    // dbg!(item.item);
    let name = "";
    log::debug!("[body] {:#?}", event.body());
    let mut route_key = String::new();

    // Extract some useful information from the request
    let resource_path = if let RequestContext::ApiGatewayV2(http_context) = event.request_context()
    {
        log::debug!("[domain_name] {:#?}", http_context.domain_name.unwrap());
        log::debug!("[domain_prefix] {:#?}", http_context.domain_prefix.unwrap());
        log::debug!(
            "[route_key] {:#?}",
            http_context.route_key.as_ref().unwrap()
        );
        route_key = http_context.route_key.unwrap();
        log::debug!("[stage] {:#?}", http_context.stage.unwrap());
        log::debug!("[time] {:#?}", http_context.time.unwrap());
        log::debug!("[time_epoch] {:#?}", http_context.time_epoch);
        http_context.http.path.unwrap()
    } else {
        unreachable!()
    };

    match (event.method(), resource_path.as_ref()) {
        (&Method::GET, "/dev/rate") => {}
        _ => (),
    }

    // Return something that implements IntoResponse.
    // It will be serialized to the right response event automatically by the runtime
    let resp = Response::builder()
        .status(200)
        .header("content-type", "application/json")
        .body(
            json!({
                "message": "This is a Rust server!",
                "method": event.method().as_ref(),
                "path_param": event.path_parameters(),
                "resource_path": resource_path,
                "tables": list_tables,
                "region": region,
                "name": name,
                "route_key": route_key
            })
            .to_string()
            .into(),
        )
        .map_err(Box::new)?;
    Ok(resp)
}

#[tokio::main]
async fn main() -> Result<(), Error> {
    CombinedLogger::init(vec![TermLogger::new(
        if cfg!(debug_assertions) {
            LevelFilter::Debug
        } else {
            LevelFilter::Info
        },
        ConfigBuilder::default().build(),
        simplelog::TerminalMode::Mixed,
        simplelog::ColorChoice::Always,
    )])
    .unwrap();

    if cfg!(debug_assertions) {
        log::info!("Debug mode");
    } else {
        log::info!("Release mode");
    }

    tracing_subscriber::fmt()
        .with_max_level(tracing::Level::INFO)
        // disable printing the name of the module in every log line.
        .with_target(false)
        // disabling time is handy because CloudWatch will add the ingestion time.
        .without_time()
        .init();

    run(service_fn(function_handler)).await
}
