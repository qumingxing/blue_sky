use flexi_logger::{AdaptiveFormat, Duplicate, Logger};

mod router;
mod server;
mod users;
mod request_mapping;
mod web_handler;
mod response;

fn main() {
    Logger::with_env_or_str("info")
        .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
        .adaptive_format_for_stdout(AdaptiveFormat::Default)
        .log_to_file()
        .duplicate_to_stderr(Duplicate::Info)
        .start()
        .unwrap();

    log::info!("Server start up!");
    server::start_server();
}
