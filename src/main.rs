mod app;
mod input_manager;

use app::App;

fn main() {
    let app = App::new().unwrap();
    app.run();
}
