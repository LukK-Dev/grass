mod app;
mod input_manager;
mod renderer;
mod timer;
mod timing;

use app::App;

#[tokio::main]
async fn main() {
    let app = App::new()
        .await
        .unwrap_or_else(|err| panic!("Failed to create App: {}", err));
    app.run();
}
