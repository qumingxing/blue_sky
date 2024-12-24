use crate::web_handler::WebHandler;

pub fn register_request_mapping(){
    let router = &mut crate::server::INSTANCE.lock().unwrap();
    WebHandler::register_route_handle_demo(router);
    WebHandler::register_route_handle_demo_x(router);
}