#![feature(lang_items)]
#![feature(rustc_private)]
#![feature(decl_macro)]
#![feature(panic_info_message)]
#![feature(negative_impls)]

extern crate core;
extern crate pi;
extern crate stack_vec;

pub mod console;
pub mod lang_items;
pub mod mutex;
pub mod shell;

#[no_mangle]
pub extern "C" fn kmain() {
    //let mut pin16 = Gpio::new(16).into_output();
    //let mut uart = MiniUart::new();

    //pin16.set();
    //console::kprintln!("Testing!");
    shell::shell("> ");
}
