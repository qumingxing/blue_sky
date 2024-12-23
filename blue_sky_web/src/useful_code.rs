use std::any::Any;

fn check_type(val: &dyn Any) {
    if let Some(s) = val.downcast_ref::<String>() {
        println!("It's a String: {}", s);
    } else {
        println!("Not a String");
    }
}