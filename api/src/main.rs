use basedcast_api::app;

itconfig::config! {
    DATABASE_URL: String,

    ROCKET {
        static BASE_URL: String => "/",
    }
}

fn main() {
    app::init().launch();
}
