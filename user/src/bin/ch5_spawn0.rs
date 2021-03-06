#![no_std]
#![no_main]

#[macro_use]
extern crate user_lib;

use user_lib::{spawn, wait};
const MAX_CHILD: usize = 40;

#[no_mangle]
pub fn main() -> i32 {
    for _ in 0..MAX_CHILD {
        let cpid = spawn("ch5_getpid");
        assert!(cpid >= 0, "child pid invalid");
        println!("new child {}", cpid);
    }
    let mut exit_code: i32 = 0;
    for _ in 0..MAX_CHILD {
        assert!(wait(&mut exit_code) <= 0, "wait stopped early");
        assert_eq!(exit_code, 0, "error exit ocde {}", exit_code);
    }
    assert!(wait(&mut exit_code) > 0, "wait got too many");
    println!("TEST spawn0 OK!");
    0
}