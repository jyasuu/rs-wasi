fn main() {
    let s : &str = "Hello WasmEdge!";
    println!("{}", s);
}

#[no_mangle]
pub fn add(a: i32, b: i32) -> i32 {
  return a + b;
}