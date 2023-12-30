//main.rs
mod game;

use game::tictactoe::tictactoe::TicTacToe as TicTacToe;
use crate::game::tictactoe::tictactoe::GameError;

use actix_cors::Cors;
use actix_web::{self, HttpServer, HttpResponse, App, web, http::header, post, /*get*/};
use serde_json::json;
use serde::{Serialize, Deserialize};
use std::sync::{Arc, Mutex};
use log::{error, info};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct UserData  {
    row: usize,
    col: usize,
    player: char
}

#[derive(Default)]
struct AppState {
    user_data: Mutex<Option<UserData>>,
    game_state: Mutex<Option<TicTacToe>>,
}

#[post("/send")]
async fn get_position(data: web::Json<UserData>, app_state: web::Data<Arc<AppState>>) -> HttpResponse {
    let mut user_data = data.into_inner();
    let mut app_data = app_state.user_data.lock().unwrap();

    *app_data = Some(user_data.clone());

    let mut game_state = app_state.game_state.lock().unwrap();
    if game_state.is_none()
        { *game_state = Some(TicTacToe::new()); }

    if let Some(game_state) = game_state.as_mut() {
        info!("Position: row: {}, col: {}, player: {}", user_data.row, user_data.col, user_data.player);

        // Use if let for better readability
        if let Err(error) = game_state.make_move(user_data.row, user_data.col, user_data.player) {
            let error_message = match &error {
                GameError::InvalidMove(msg) => msg.clone(),
                GameError::OutOfBounds => "Out of bounds".to_string(),
                GameError::GameOver => "Game over".to_string(),
            };

            error!("Error: {}", error_message);

            return HttpResponse::InternalServerError().json(json!({"error": error_message}));
        }

        info!("Board: {:?}", game_state.board);
        match game_state.is_over() {
            Some(winner) => {
                user_data.player = if user_data.player == 'X' { 'O' } else { 'X' };
                HttpResponse::Ok().json(json!({"status": "over", "winner": winner}))
            },
            None => HttpResponse::Ok().json(json!({
                "row": user_data.row,
                "col": user_data.col,
                "status": "in_progress",
                "current_player": user_data.player
            })),
        }
    }
    else { HttpResponse::InternalServerError().json(json!({"Error": GameError::InvalidMove(String::from("Cell already occupied or out of bounds")).to_string()}))}
}

#[post("/clear")]
async fn clear_board(app_state: web::Data<Arc<AppState>>) -> HttpResponse {
    info!("Clearing..");
    let mut user_data = app_state.user_data.lock().unwrap();
    *user_data = None;

    let mut game_state = app_state.game_state.lock().unwrap();
    *game_state = None;

    info!("Board is cleared");
    HttpResponse::Ok().json(json!({"status": "board_cleared"}))
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    std::env::set_var("RUST_LOG", "debug");
    std::env::set_var("RUST_BACKTRACE", "1");
    env_logger::init();

    let address = "127.0.0.1:8080";
    println!("Server is running at http://{}", address);
    let app_state = Arc::new(AppState::default());

    HttpServer::new(move || {
        let cors = Cors::default()
            .allow_any_origin()
            .allowed_methods(vec!["GET", "POST"])
            .allowed_headers(vec![header::AUTHORIZATION, header::ACCEPT, header::CONTENT_TYPE])
            .max_age(3600);

        App::new()
            .wrap(cors)
            .app_data(web::Data::new(app_state.clone()))
            .service(get_position)
            .service(clear_board)

    }).bind(address)?
        .run()
        .await?;

    Ok(())
}
