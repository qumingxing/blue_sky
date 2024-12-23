use flexi_logger::{AdaptiveFormat, Duplicate, Logger};

mod server;
mod router;
mod users;

fn main() {
    Logger::with_env_or_str("info")
        .adaptive_format_for_stderr(AdaptiveFormat::Detailed)
        .adaptive_format_for_stdout(AdaptiveFormat::Default)
        .log_to_file()
        .duplicate_to_stderr(Duplicate::Info)
        .start()
        .unwrap();

    server::start_server();



}
