use std::sync::Arc;
use colored::Colorize;

use warp::{
    http::Uri,
    hyper::{Response, StatusCode},
    path::{FullPath, Tail},
    Filter, Rejection, Reply,
};

mod api;
mod hackerrank;
mod rust_lang;

use hackerrank::*;
use rust_lang::*;
use utoipa::OpenApi;
use utoipa_swagger_ui::Config;

#[tokio::main]
async fn main() {
    println!("{}", "Hacker Rank Challenges\n".blue().bold());

    println!("{}", "Problem Solving\n".green().bold());

    println!("{}", "Birthday Candles".yellow().bold());
    println!("{}", problem_solving::birthday_cake_candles(&[3, 2, 1, 3]));
    println!();

    println!("{}", "Bon Appetit".yellow().bold());
    problem_solving::bon_appetit(&[3, 10, 2, 9], 1, 7);
    println!();

    println!("{}", "Grading Students".yellow().bold());
    problem_solving::grading_students(&[73, 67, 38, 33]);
    println!();

    println!("{}", "Migratory Birds".yellow().bold());
    println!("{}", problem_solving::migratory_birds(&[1, 2, 3, 4, 5, 4, 3, 2, 1, 3, 4]));
    println!();

    println!("{}", "Min & Max Sum".yellow().bold());
    problem_solving::mini_max_sum(&[1, 2, 3, 4, 5]);
    println!();
    println!();

    println!("{}", "Plus Minus".yellow().bold());
    problem_solving::plus_minus(&[-4, 3, -9, 0, 4, 1]);
    println!();

    println!("{}", "Sock Merchant".yellow().bold());
    println!("{}", problem_solving::sock_merchant(9, &[10, 20, 20, 10, 10, 30, 50, 10, 20]));
    println!();

    println!("{}", "Staircase".yellow().bold());
    problem_solving::staircase(5);

    println!("{}", "Time Conversion".yellow().bold());
    println!("{}", problem_solving::time_conversion("12:40:22AM"));
    println!();

    println!("{}", "Data Structures\n".green().bold());

    println!("{}", "Reverse Array".yellow().bold());
    println!("{:?}", data_structures::reverse_array(&[1, 4, 3, 2]));
    println!();

    println!("{}", "Rotate Left".yellow().bold());
    println!("{:?}", data_structures::rotate_left(4, &[1, 2, 3, 4, 5]));
    println!();

    println!("{}", "Rust Language Book Programming\n".blue().bold());

    println!("{}", "Enums".yellow().bold());
    println!("{}", enums(WebEvent::Click { x: 20, y: 80 }));
    println!();

    println!("{}", "Secret Number".yellow().bold());
    secret_number();
    println!();

    println!("{}", "Warp\n".blue().bold());
    println!();

    let config = Arc::new(Config::from("/api-doc.json"));

    #[derive(OpenApi)]
    #[openapi(
        paths(
            problem_solving::staircase,
            problem_solving::time_conversion
        ),
        tags((
            description = "Learning Rust API",
            name = "Learning Rust"
        ))
    )]
    struct ApiDoc;

    let api_doc =
        warp::path!("api-doc.json")
            .and(warp::get())
            .map(|| warp::reply::json(&ApiDoc::openapi()));

    println!("{}", "🚀 Serving from http://127.0.0.1:3030/ - try:");
    println!("{}", "  ➡️ http://127.0.0.1:3030/bon_appetit/");
    println!("{}", "  ➡️ http://127.0.0.1:3030/rotate_left/4/");
    println!("{}", "  ➡️ http://127.0.0.1:3030/staircase/6/");
    println!("{}", "  ➡️ http://127.0.0.1:3030/time_conversion/12:40:22AM/");

    let swagger_ui = warp::path("swagger-ui")
        .and(warp::get())
        .and(warp::path::full())
        .and(warp::path::tail())
        .and(warp::any().map(move || config.clone()))
        .and_then(serve_swagger);

    warp::serve(api_doc.or(swagger_ui).or(api::handlers()))
        .run(([127, 0, 0, 1], 3030))
        .await;
}

async fn serve_swagger(
    full_path: FullPath,
    tail: Tail,
    config: Arc<Config<'static>>,
) -> Result<Box<dyn Reply + 'static>, Rejection> {
    if full_path.as_str() == "/swagger-ui" {
        return Ok(Box::new(warp::redirect::found(Uri::from_static(
            "/swagger-ui/",
        ))));
    }

    let path = tail.as_str();

    match utoipa_swagger_ui::serve(path, config) {
        Ok(file) => {
            if let Some(file) = file {
                Ok(Box::new(
                    Response::builder()
                        .header("Content-Type", file.content_type)
                        .body(file.bytes),
                ))
            } else {
                Ok(Box::new(StatusCode::NOT_FOUND))
            }
        }
        Err(error) => Ok(Box::new(
            Response::builder()
                .status(StatusCode::INTERNAL_SERVER_ERROR)
                .body(error.to_string()),
        )),
    }
}
