mod hashing_files;
mod hashing_strings;

fn main() {
    let mut buf = hashing_strings::hash_strings(b"backpack");
    println!("hashing string\n{:02X?}\n", buf);

    buf = hashing_files::hash_files("my_img.jpg")
        .unwrap_or_else(|error| panic!("hashing files\n{:?}\n", error));
    println!("hasing files\n{:02X?}\n", buf);
}
