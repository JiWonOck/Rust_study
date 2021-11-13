//src/lib.rs 내에 client 모듈을 network 모듈 안으로 이동
#![allow(unused)]
fn main() {
	mod network {
	    fn connect() {
	    }

	    mod client {
	        fn connect() {
	        }
	    }
	}
}
/*
communicator
 └── network
     └── client
*/
