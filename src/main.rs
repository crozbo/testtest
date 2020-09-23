fn main() {
    let p = Package {
        pkgname: "testtest".to_string(),
        pkgver: "0.1.0".to_string(),
        pkgrel: "1".to_string(),
        pkgdesc: "this package do nothig".to_string(),
    };
    println!("Hello, {:?}!", p);
}

#[derive(Debug)]
struct Package {
    // TODO pkgname need to be be Vec because split packages can have more than 1 name
    pkgname: String,
    pkgver: String,
    pkgrel: String,
    pkgdesc: String,
}
