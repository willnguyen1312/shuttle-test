use actix_web::{get, web::ServiceConfig};
use shuttle_actix_web::ShuttleActixWeb;

#[get("/")]
async fn hello_world() -> &'static str {
    "Hi World ✨"
}

#[shuttle_runtime::main]
async fn main() -> ShuttleActixWeb<impl FnOnce(&mut ServiceConfig) + Send + Clone + 'static> {
    let config = move |cfg: &mut ServiceConfig| {
        cfg.service(hello_world);
    };

    Ok(config.into())
}


// https://playhop.com/app/272602?utm_campaign=en-tier_rich_playhop_title-top_google_search%257C21174017225&utm_medium=search&utm_source=game_header_logo&utm_term=fish%2520eat%2520getting%2520big&utm_content=k50id%7Ckwd-2173349715979%7Ccid%7C21174017225%7Caid%7C696263462870%7Cgid%7C162304046873%7Cpos%7C%7Csrc%7Cg_%7Cdvc%7Cc%7Creg%7C9215891%7Crin%7C%7C#goog_rewarded
