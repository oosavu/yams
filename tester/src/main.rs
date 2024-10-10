#![allow(dead_code)]
#![allow(unused)]

use steel::steel_vm::engine::Engine;
use steel::steel_vm::register_fn::RegisterFn;

use steel_derive::Steel;

// In order to register a type with Steel,
// it must implement Clone, Debug, and Steel
#[derive(Clone, Debug, Steel, PartialEq)]
pub struct ExternalStruct {
    foo: usize,
    bar: String,
    baz: f64,
}

#[derive(Clone, Debug, Steel, PartialEq)]
struct MyGlobalCoreObject {}

impl MyGlobalCoreObject {
    pub fn my_global_function(&self, parameter: &str) -> i32 {
        23
    }
}

#[derive(Clone, Debug, Steel, PartialEq)]
struct MyLittleObject {
    val: i32,
}

impl MyLittleObject {
    pub fn new(global: &MyGlobalCoreObject) -> Self {
        MyLittleObject { val: global.my_global_function("foobarbaz")}
    }

    pub fn get_val(&self) -> i32 {
        self.val
    }

}


pub fn main() {
    let mut vm = Engine::new();
    vm.register_type::<MyLittleObject>("MyLittleObject?");
    vm.register_fn("MyLittleObject", MyLittleObject::new);
    vm.register_fn("get_val", MyLittleObject::get_val);
    let global_object: MyGlobalCoreObject = MyGlobalCoreObject {};
    vm.register_external_value("global_object", global_object)
        .unwrap();

    // how to implicitly pass the global_object here?
    let out = vm
        .compile_and_run_raw_program(
            r#"
            (define my_little_object (MyLittleObject global_object))
            (get_val my_little_object)
        "#,
        ).unwrap();

    println!("get_output: {out:?}");
}


