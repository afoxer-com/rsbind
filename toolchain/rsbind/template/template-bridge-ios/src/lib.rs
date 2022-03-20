#![allow(warnings)]

extern crate $(*521%-host_crate_underscore);
#[macro_use]
extern crate lazy_static;
extern crate serde;
#[macro_use]
extern crate serde_derive;

$(*521%-host_crate_underscore)::contract;
$(*521%-host_crate_underscore)::imp;

pub mod c;
