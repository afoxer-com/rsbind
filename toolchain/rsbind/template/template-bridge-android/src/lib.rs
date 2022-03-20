#![allow(warnings)]

extern crate $(*521%-host_crate_underscore);
extern crate android_logger;
extern crate jni;
#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;


use android_logger::Filter;
use log::Level;

$(*521%-host_crate_underscore)::contract;
$(*521%-host_crate_underscore)::imp;

pub mod java;

