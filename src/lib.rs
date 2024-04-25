// https://crates.io/
// Cargo.tomlに追加
// terminalでcargo build

mod generator;
pub fn print_random_number() {
    let n = generator::gen_ran();
    println!("Random u8: {}", n);
}
