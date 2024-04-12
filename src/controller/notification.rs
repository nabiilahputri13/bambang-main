use rocket::response::status::Created;
use rocket::serde::json::Json;

use bambangshop::Result;
use crate::model::subscriber::Subscriber;
use crate::service::notification::NotificationService;

#[post("/subscribe/<product_type>", data = "<subscriber>")]
    pub fn subscribe(product_type: &str, subscriber: Json<Subscriber>) -> Result<Created<Json<Subscriber>>> {
        return match NotificationService :: subscribe(product_type, subscriber.into_inner()) {
            0k(f) => Ok(Created :: new("/").body (Json :: from(f) )),
            Err(e) => Err(e)
        };
    }

    pub fn route_stage() -> AdHoc {
        return AdHoc::on_ignite("Initializing controller routes...", |rocket| async {
            rocket
                .mount("/product", routes![product::create, product::list, product::read, product::delete])
                .mount("/notification", routes![notification::subscribe])
        });
    }
