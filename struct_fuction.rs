pub fn contoh_implementasi() {
    // Membuat instance dari struct Rectangle
    let rect1 = Rectangle {
        width: 30,
        height: 50,
    };

    // Memanggil function untuk menghitung luas
    let area = calculate_area(&rect1);

    println!("Luas persegi panjang adalah {}", area);
}

// Membuat struct
struct Rectangle {
    width: u32,
    height: u32,
}

// Membuat function untuk menghitung luas
fn calculate_area(rect: &Rectangle) -> u32 {
    rect.width * rect.height
}
