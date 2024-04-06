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

    // pemanggilan fungsi
    cetak_diterminal(x, y);

    // print hasil penjumlahn
    let jumlah = calculate(x, y);
    print!("\n{} + {} = {}", x, y, jumlah);

    // 3 Variable Binding
    // mutable and imutable variable, by default variable di rust itu imutable
    // imutable variable
    let (d1, d2)= (500, 5000);
    print!("\nD1 : {}\nD2 : {}", d1, d2);

    // mutable varibale
    let mut data1 = 100;
    let mut data2 = 30;
    data1 = data1 + 100;
    data2 = data2 + 10;
    println!("\nTotal Data 1 : {}", data1);
    println!("Total Data 2 : {}\n", data2);

    // String assigment
    let say1 = "hello".to_string();
    let say2 = String::from("hello 2");
    let say3 = "hello 3";

    print!("\nSay1:{}\nSay2:{}\nSay3:{}\n`", say1, say2, say3);

    // Aritmatic Operation
    println!("2 + 4 = {}\n", 2 + 3 );
    println!("2 - 4 = {}\n", 2 - 3 );
    println!("2 * 4 = {}\n", 2 * 3 );
    println!("2 / 4 = {}\n", 2 / 3 );
    println!("2 % 4 = {}\n", 2 % 3 );

    // Logical Operation
    println!("true AND true = {}", true && true);
    println!("true AND false = {}", true && false);
    println!("false AND false = {}", false && false);
    println!("true OR true = {}", true || true);
    println!("true OR false = {}", true || false);
    println!("false OR false = {}", false || false);
    println!("NOT false = {}", !false);
    println!("NOT true = {}", !true);

    // Comparsion
    let xx:i32 = 100;
    let yy:i32 = 200;
    println!("xx is greater than yy : {}", xx > yy);
    println!("xx is less than yy : {}", xx < yy);
    println!("xx is unequal to yy : {}", xx != yy);
    println!("xx is greater/equal to yy : {}", xx >= yy);
    println!("xx is less/equal to yy : {}", xx <= yy);
    println!("xx is completelyy equal to yy : {}", xx == yy);

    // Array
    let mut arr: [i32; 4] = [8; 4];
    arr[1] = 10;
    arr[2] = 20;
    print!("0 =>{}, 1 =>{}, 2 =>{}, 3 =>{} ", arr[0], arr[1], arr[2], arr[3]);

    let mut arr2: [f32; 4] = [0.1, 0.2, 0.3, 0.4];
    arr2[2] = 10.5;
    print!("0 =>{}, 1 =>{}, 2 =>{}, 3 =>{} ", arr2[0], arr2[1], arr2[2], arr2[3]);

    
}


// untuk nama fungsi convention yang digunakan anakan snake case misal tambah_data, simpan_data
fn cetak_diterminal(x:i32, y:i32) {
    print!("Nilai X >> {}\nNilai Y >> {}", x, y);
}

fn calculate(x:i32, y:i32) -> i32 {
    x+y
}