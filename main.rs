use util::str_to_uppercase;

mod util;

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
    print!("0 =>{}, 1 =>{}, 2 =>{}, 3 =>{} \n", arr2[0], arr2[1], arr2[2], arr2[3]);

    // Slyce
    let arrdata = [1,2,3,4,5,6,7];
    let slc = &arrdata[2..5];
    println!("{}", slc[0]);

    // 4 Control Flow
    // if stetment
    let nomor = 10;
    if nomor == 10 {
        println!("NILAINYA SEBULUH");
    }

    // Else if
    if nomor > 10 {
        println!("Lebih dari sebpuluh");
    } else {
        println!("kurang dari sepuluh");
    }

    // letif
    let nomor = if true {
        10000
    } else {
        500
    };

    println!("NOMOR : {}", nomor);

    // 5 Loop
    let mut looping = 0;

    // loop
    loop {
        println!("Looping ke : {}", looping);
         if looping == 4 {
            break;
         }
         looping = looping + 1;
    }
    

    // for statement
    for data in 0..10 {
        println!("DATA KE : {}", data);
    }

    // while statemnt
    while  looping < 10 {
        println!("Looping ke >> {}", looping);
        looping += 1;
    }

    // Tuples
    let tupl = (10, 10.5, "coba");
    print!("{} {} {}\n", tupl.0, tupl.1, tupl.2);

    // match
    let databaru:i32 = 4;

    match databaru {
        1 => println!("SATU"),
        2 => println!("DUA"),
        3 => println!("TIGA"),
        4 => println!("EMPAT"),
        5 => println!("LIMA"),
        _ => println!("DEFAULT"),
    }

    // 5 Struct
    // struct
    struct Person {
        nama: String,
        usia: i32,
    }   

    let pengunjung = Person{
        nama: String::from("Aulia"),
        usia: 30,
    };

    println!("NAMA : {}", pengunjung.nama);
    println!("USIA : {}", pengunjung.usia);

    enum STATUS {
        OK,
        NOK,
    }

    let status = STATUS::OK;
    let statusnok = STATUS::NOK;
    match status {
        STATUS::OK => println!("STATUS OK"),
        STATUS::NOK => println!("STATUS NOT OK"),
    }

    match statusnok {
        STATUS::OK => println!("STATUS OK"),
        STATUS::NOK => println!("STATUS NOT OK"),
    }


    // Ownership
    let nama = String::from("AULIA");
    let namanya = nama;
    // println!("NAMA : {}", nama); // error karena ownership di memorinya udah pindah ke variable namanya
    println!("NAMANYA : {}", namanya);

    let nama_hewan = String::from("kepiting 🦀");
    println!("INI >> {}", nama_hewan);

    let uppercase_nama_hewan = uppercase(nama_hewan);
    println!("INI >> {}", uppercase_nama_hewan);
    // println!("INI ERROR >> {}", nama_hewan); // ini bakalan error karena ownershipnya pindah ke var yang di fungsi uppercase

    // Referrence mirip pointer di go
    let makanan = String::from("ice cream 🍦");
    let makanan2 = &makanan;
    println!("Makanan 1 >> {}", makanan);
    println!("Makanan 2 >> {}", makanan2);

    let makanan_uppercase = uppercase_ref(&makanan);
    println!("Makanan >> {}", makanan);
    println!("Makanan Uppercase >> {}", makanan_uppercase);

    // Module
    utility::cetak_nama(&makanan);
    
    // EMBEDEN MODULE
    let number = 5;
    utility::counter::increament(number);
    utility::counter::decfrement(number);

    // external file
    str_to_uppercase(makanan);

    // utility::test_private(); // ini error karenamanggil private function

    // super
    utility::counter::call_test();

    // vector
    let vector_name = vec![100, 200, 300];
    println!("VECTOR 1 {}", vector_name[0]);
    println!("VECTOR 2 {}", vector_name[2]);

    let v2 = vec![10;4];
    println!("VECTOR 1 {}", v2[0]);

    let mut v3 = Vec::new();
    v3.push(54);
    v3.push(5);
    println!("VECTOR 1 {}", v3[0]);
    println!("VECTOR 2 {}", v3[1]);

    // multiple pattern, mirip kayak or
    let test_data = 0;
    match test_data {
        0 | 5 => println!("0 | 5"),
        1 => println!("1"),
        _ => println!("-")
    }

    // range bissa dipake untuk or dalam kontek misalkan array
    let z = 4;
    match z {
        // 2...5 => println!("2...5"), // range dengan tanda titi tiga kali (...) sudah deprecated
        2..=5 => println!("2..=5"), // gunakan pattern ini sebagai pengganti ref -> https://doc.rust-lang.org/reference/patterns.html, https://stackoverflow.com/a/49834813/20927756
        1 => println!("1"),
        _ => println!("0")
    }

    // binding a range
    let c = 5;
    match c {
        // var @5...10 => println!("{}", var), // pattern ini deprecated
        var @5..=10 => println!("{}", var),
        _ => println!("other")
    }

    // GENERIC
    let d: Option<i32> = Some(100);
    match d {
        Some(d) => println!("{}", d),
        None => print!("NONE")
    }

    // METHOD
    let obj = Circle{radius: 10.0};
    println!("AREA OF CIRCLE >> {}", obj.area());

    // Trait ini kayak interface, kita buat trait dari struct lalu definisikan fun dalam trait, dan harus diimplement
    let obj2 = Square{long: 2.0, width: 3.0, height: 10.0};
    println!("Volume of Square>> {}", obj2.volume());
    println!("Wide of Square>> {}", obj2.wide());

    // Drop, fungsi bawaan untuk decrement reference count, valuenya harus imutable agar bisa dikurangi
    let football = Game{point: 1};
    let basketball = Game{point: 2};
    let socer = Game{point: 3};
}


// untuk nama fungsi convention yang digunakan anakan snake case misal tambah_data, simpan_data
fn cetak_diterminal(x:i32, y:i32) {
    print!("Nilai X >> {}\nNilai Y >> {}", x, y);
}

fn calculate(x:i32, y:i32) -> i32 {
    x+y
}

fn uppercase(var: String) -> String {
    var.to_uppercase()
}

fn uppercase_ref(var: &String) -> String {
    var.to_uppercase()
}

mod utility {
    pub fn cetak_nama(var: &String) {
        println!("NAMA >> {}", var)
    }

    fn test_private() {
        println!("INI PRIVATE FUNCTION");
    }

    pub mod counter {
        use super::test_private;

        pub fn call_test() {
            test_private();
        }

        pub fn increament(var: i32){
            println!("INCREMENT >> {}", var + 1);
        }

        pub fn decfrement(var: i32){
            println!("DECREMENT >> {}", var - 1);
        }
            
    }

}

struct Circle {
    radius: f32,
}

impl Circle{ 
    fn area(&self) -> f32{
       std::f32::consts::PI * self.radius * self.radius
    } 
}

struct  Square {
    long: f32,
    width: f32,
    height: f32,
}

trait Calculate {
    fn volume(&self) -> f32;
    fn wide(&self) -> f32;
}

impl Calculate for Square {
    fn volume(&self) -> f32 {
        self.long * self.width * self.height
    }

    fn wide(&self) -> f32 {
        self.long * self.width
    }
}

struct Game {
    point: i32,
}

impl Drop for Game {
    fn drop(&mut self) {
        println!("The Winner #{}", self.point)
    }
}