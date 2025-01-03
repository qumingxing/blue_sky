use blue_sky_dao::dao::jdbc_template::BaseDao;
use blue_sky_dao::get_user_dao;
use blue_sky_service::utils::get_hex;

mod services;

fn main() {
    println!("Hello, world!{}",get_hex("123456"));
}
