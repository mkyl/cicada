#![allow(non_snake_case)]
#![allow(non_camel_case_types)]
#![allow(non_upper_case_globals)]

extern crate rand;

mod zobrist;
pub mod board;

fn main(){
    zobrist::init();
   // zobrist::hash();
}
