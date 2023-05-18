mod hashing_email;
mod hashing_files;
mod hashing_strings;

fn main() {
    let mut buf = hashing_strings::hash_strings(b"backpack");
    println!("hashing string\n{:02X?}\n", buf);

    buf = hashing_files::hash_files("my_img.jpg")
        .unwrap_or_else(|error| panic!("hashing files\n{:?}\n", error));
    println!("hasing files\n{:02X?}\n", buf);

    let email_body = "Hello World, I think Blockchains could be the tech of the future.";
    let secret_phrase = "bolognese";
    buf = hashing_email::hash_email(email_body, secret_phrase);
    println!("hasing emails\n{:02X?}\n", buf);
}
