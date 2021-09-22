fn greet_world() {
    println!("Hello, world!");
    let thai = "สวัสดี ชาวโลก";
    let japan = "ハロー・ワールド";
    let regions = [thai, japan];
    for region in regions.iter() {
        println!("{}", &region);
    }
}
fn main() {
    greet_world();
}
