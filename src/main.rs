fn main() {
    let p = Package::new(
        "testtest".to_string(),
        "0.1.0".to_string(),
        "1".to_string(),
        "this package do nothig".to_string(),
    );
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

impl Package {
    fn new(pkgname: String, pkgver: String, pkgrel: String, pkgdesc: String) -> Self {
        Package {
            pkgname,
            pkgver,
            pkgrel,
            pkgdesc,
        }
    }
}
