#![feature(type_alias_impl_trait)]
// check-pass

fn main() {
    type T = impl Copy;
    let foo: T = Some((1u32, 2u32));
    let x = move || {
        match foo {
            None => (),
            Some((a, b)) => (),
        }
    };
}
