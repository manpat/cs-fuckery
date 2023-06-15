use std::ffi::CStr;

#[repr(C)]
#[derive(Debug)]
struct BasicStruct {
    wahoo: i32,
    small: i8,
    foo: f32,
}


extern "C" {
    fn hello_world();
    fn process_value(_: i32) -> i32;
    fn returns_struct() -> BasicStruct;

    fn returns_string() -> *const std::ffi::c_char;
    fn cs_free(_: *const std::ffi::c_char);
}

fn main() -> anyhow::Result<()> {
    unsafe {
        hello_world();
        println!("process_value() -> {}", process_value(5));
        println!("returns_struct() -> {:?}", returns_struct());

        let cs_string = returns_string();
        println!("returns_string() -> {}", CStr::from_ptr(cs_string).to_str()?);
        cs_free(cs_string);
    }

    println!(":)");

    Ok(())
}




