use std::sync::Arc;
use rlist_vfs::Wheel;
use actix_web::web;

pub type WheelData = web::Data<Arc<Wheel>>;