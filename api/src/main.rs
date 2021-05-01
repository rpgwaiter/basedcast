use basedcast_api::app;
use basedcast_core::settings;


itconfig::config! {
    DATABASE_URL: String,

    ROCKET {
        static BASE_URL: String => "/",
    }
}

fn main() {
    app::init().launch();
}
