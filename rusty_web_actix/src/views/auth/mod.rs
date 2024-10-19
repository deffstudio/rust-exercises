mod login;
mod logout;

use actix_web::web::{get, scope, ServiceConfig};

pub fn auth_views_factory(app: &mut ServiceConfig) {
    app.service(
        scope("v1/auth")
            .route("login", get().to(login::login))
            .route("logout", get().to(logout::logout)),
    );
}
