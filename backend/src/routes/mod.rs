pub mod contact;
pub mod root;
pub mod messages;

use actix_web::web;

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(root::index);
    cfg.service(contact::contact);
    cfg.service(messages::list_messages);
}