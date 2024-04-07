pub fn contoh_ownership() {
    // ownership,
    let s = String::from("hello");
    takes_ownership(s);

    //reference
    let s1 = String::from("hello");
    let len = calculate_length(&s1);
    println!("reference => Panjang string '{}': {}", s1, len);

    //borrowing
    let mut s = String::from("hello");
    modify_string(&mut s); 
    println!("borrowing => {}", s);

    //clone
    let s1 = String::from("hello");
    let s2 = s1.clone(); 
    println!("clone => s1 = {}, s2 = {}", s1, s2);

    //copy
    let x = 5;
    let y = x; 
    println!("copy => x = {}, y = {}", x, y);

    //scope
    let s = "hello";
    {
        let s = "world"; 
        println!("scope => {}", s); 
    } 
    println!("scope => {}", s); 

    //mutable
    let mut s = String::from("hello");
    s.push_str(", world!");
    println!("mutable =>{}", s);

    //dangling
    let reference_to_nothing = dangle();
    println!("dangling => {}", reference_to_nothing);

    //slice
    let s = String::from("hello world");
    let hello = &s[0..5]; // Membuat slice dari string
    let world = &s[6..]; // Membuat slice dari string tanpa batas akhir
    println!("slice => {} {}", hello, world); // hello world

}

fn takes_ownership(some_string: String) {
    println!("ownership => {}", some_string);
}

fn calculate_length(s: &String) -> usize { // Mengambil referensi string
    s.len()
}

fn modify_string(s: &mut String) {
    s.push_str(" world");
}

fn dangle() -> String {
    let s = String::from("hello");
    s // Error: Mengembalikan reference ke string yang berakhir di akhir scope
}