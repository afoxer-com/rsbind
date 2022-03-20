#![allow(warnings)]

extern crate $(*521%-host_crate_underscore);
extern crate jni;
#[macro_use]
extern crate lazy_static;
#[macro_use] extern crate log;
extern crate serde;
#[macro_use]
extern crate serde_derive;


use log::Level;

$(*521%-host_crate_underscore)::contract;
$(*521%-host_crate_underscore)::imp;

pub mod java;

