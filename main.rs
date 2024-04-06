// consts didefiniskan di luar main
const NUM: i32 = 1000;

/*fn main ini merupakan entri point untuk rust pertama kali dijalankan */
fn main() {
    // 1. Print `Helo kepiting di terminal`
    println!("Hello Kepiting");

    // 2. Belajar Type Data
    // Convention untuk penulisan variable menggunakan snake case
    let is_oke = "OK";
    println!("\nPrint OK >> {}", is_oke);

    // pendefinisian variable dengan tipe data
    let counter: i32 = 0;
    let title: String = "Current Counter".to_string();
    println!("\n{}", title);
    println!("`{}`", counter);

    // Output format, digunakan untuk melakukan print data dengan passing variable
    let x = 100;
    let y = 200;
    let z = 500;
    println!("\n{}", x);
    print!("Data 1 >> {}\nData 2 >> {}\n", y, z);

    // Constanta, digunakan untuk mendefinisikan constants
    println!("\nPRINT CONST >> {}", NUM);

    // Convert type data
    let variable1: f32 = 100.99;
    let variable2: i32 = variable1 as i32;
    println!("\n{}", variable1);
    println!("{}", variable2);

    // Reserved Word
    /*
        Reserved Words
    abstract alignof as become box
    break const continue crate do
    else enum extern false final
    fn for if impl in
    let loop macro match mod
    move mut
    offsetof override priv
    proc pub pure ref return
    Self self sizeof static struct
    super trait true type typeof
    unsafe unsized use virtual where
    while yield
    Rust reserved words cannot be used when choosing identifier names for
    variables, functions, properties. */
}
