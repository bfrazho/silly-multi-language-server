mod app_error;
mod response_converter;
mod users;
use axum::Router;

#[cfg(all(target_env = "musl", target_pointer_width = "64"))]
#[global_allocator]
static ALLOC: mimalloc::MiMalloc = mimalloc::MiMalloc;

#[derive(Clone, Default)]
struct AppState {
    pub user_base_url: String
}
impl AppState{
    pub fn set_user_base_url(&mut self, user_base_url: &str)-> &Self{
        self.user_base_url = user_base_url.to_string();
        self
    }
    pub fn to_owned(&self)-> Self {
        self.clone()
    }
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt::init();
    
    let app = Router::new().merge(routes(&AppState::default()
        .set_user_base_url("https://reqres.in/api")
    ));

    let listener = tokio::net::TcpListener::bind("127.0.0.1:3000")
        .await
        .unwrap();
    println!("listening on {}", listener.local_addr().unwrap());


    axum::serve(listener, app).await.unwrap();
}

fn routes(app_state: &AppState) -> Router {
    Router::new().merge(users::routes(app_state))
}

