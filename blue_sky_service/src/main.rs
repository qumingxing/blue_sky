use blue_sky_dao::dao::jdbc_template::BaseDao;
use blue_sky_dao::get_user_dao;

mod services;

fn main() {
    let user_dao = get_user_dao();
    let rs = user_dao.select_list(None);
    if let Ok(users) = rs {
        for item in users {
            println!("{}",item.user_name)
        }
    }
}
