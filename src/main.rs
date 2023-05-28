use {{ crate_name }}::configuration::get_configuration;

use {{ crate_name }}::startup::Application;
use {{ crate_name }}::telemetry::{get_subscriber, init_subscriber};


#[tokio::main]
async fn main() -> Result<(), hyper::Error> {
    let subscriber = get_subscriber("{{ crate_name }}".into(), "info".into(), std::io::stdout);
    init_subscriber(subscriber);
    let configuration = get_configuration().expect("Failed to read configuration.");
    let app = Application::build(configuration)
        .await
        .expect("App build failed");
    app.run_until_stopped().await?;
    Ok(())
}
