/*fn main ini merupakan entri point untuk rust pertama kali dijalankan */
fn main() {
    // Print `Helo kepiting di terminal`
    println!("Hello Kepiting");

    // Belajar Type Data
    // Convention untuk penulisan variable menggunakan snake case
    let is_oke = "OK";
    println!("\nPrint OK >> {}", is_oke);

    // pendefinisian variable dengan tipe data
    let counter:i32 = 0;
    let title:String = "Current Counter".to_string();
    println!("\n{}", title);
    println!("`{}`", counter);

    
}