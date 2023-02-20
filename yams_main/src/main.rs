extern crate yams_core;
use std::{thread, time::Duration};
//
// slint::slint!{
//     HelloWorld := Window {
//         Text {
//             text: "hello world";
//             color: green;
//         }
//     }
// }

fn main() {
    //biser::soundtest();
    let mut e = yams_core::Engine::default();
    e.
    e.start();

    thread::sleep(Duration::from_millis(3000));
    e.stop();
}
