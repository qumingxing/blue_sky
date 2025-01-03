## Learning Rust as a Beginner with Java Experience

As a new Rust learner with a background in Java, I am still trying to grasp Rust based on our previous experiences.
However, I have found the results to be unsatisfactory. Therefore, I decided to create a new project that simulates a
real business scenario as my target and as few third-party libraries as possible.

After several days of learning, I have summarized some conclusions as follows:

1. **Never give up on practice**, even if it's just one line of code a day.
2. **Embrace the learning curve**,Transitioning from Java to Rust can be challenging, but understanding Rust’s ownership
   model and borrowing concepts is crucial.
3. **Build practical projects:**,Applying what I learn in a project helps solidify my understanding and makes the
   learning process more enjoyable.
4. **Utilize community resources:**,Engaging with the Rust community, such as forums, documentation, and tutorials, has
   been invaluable in my learning journey.
5. **Be patient:**,Mastery takes time, and it’s important to celebrate small victories along the way.

By focusing on these principles, I hope to improve my Rust skills and gain more confidence in using this powerful
language.

## Demo Practice

#### Step 1: Modify configuration(Config.toml)

```

title = "blue_sky configuration"

[redis]
conn_addr = "127.0.0.1:6379"
database = 10

[mysql]
db_url = "mysql://user_name:password@ip:port/db"
connection_max = 5000
```

#### Step 2: Add request handlers

```
use crate::response::demo_response::DemoResponseBuilder;
use crate::router::Response;
use crate::server::HttpRequest;
use blue_sky_macro::route;

#[route("GET", "/demo")]
fn handle_demo(request: &HttpRequest) -> Response {
    let response = DemoResponseBuilder::default()
        .message(String::from("Hello"))
        .build()
        .unwrap();
    Response {
        status_code: 200,
        data: Some(serde_json::to_value(response).unwrap()),
    }
}

#[route("POST", "/demo_other")]
fn handle_demo_other(request: &HttpRequest) -> Response {
    Response {
        status_code: 200,
        ..Default::default()
    }
}
```

#### Step 3: Modify request_mapping.rs

```

pub fn register_request_mapping(){
    let router = &mut crate::server::INSTANCE.lock().unwrap();
    WebHandler::register_route_handle_demo(router);
    WebHandler::register_route_handle_demo_other(router);
    ...add yours
}
```