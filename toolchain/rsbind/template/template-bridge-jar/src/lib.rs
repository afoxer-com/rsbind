#![allow(warnings)]

extern crate $(*521%-host_crate_underscore);
extern crate jni;
#[macro_use]
extern crate serde_derive;
extern crate serde;


use $(*521%-host_crate_underscore)::contract;
use $(*521%-host_crate_underscore)::imp;

pub mod java;
#[macro_use]
extern crate lazy_static;

#[macro_use] extern crate log;
use log::Level;
