use dotenv::dotenv;
use basedcast_api::app;

itconfig::config! {
    DATABASE_URL: String,

    ROCKET {
        static BASE_URL: String => "/",
    }
}

fn main() {
    dotenv().ok();

    app::init().launch();
}
