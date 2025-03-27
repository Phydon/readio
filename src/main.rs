#[allow(unused_macros)]
macro_rules! read {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim().parse::<$type>().expect("Parsable");
    };
}

#[allow(unused_macros)]
macro_rules! read_str {
    ($out:ident) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).expect("A String");
        let $out = inner.trim();
    };
}

#[allow(unused_macros)]
macro_rules! read_vec {
    ($out:ident as $type:ty) => {
        let mut inner = String::new();
        std::io::stdin().read_line(&mut inner).unwrap();
        let $out = inner
            .trim()
            .split_whitespace()
            .map(|s| s.parse::<$type>().unwrap())
            .collect::<Vec<$type>>();
    };
}

fn main() {
    println!("Enter u32: ");
    read!(x as u32);
    println!("Enter f64: ");
    read!(y as f64);
    println!("Enter char: ");
    read!(z as char);
    println!("{} {} {}", x, y, z);

    println!("Enter String: ");
    read_str!(s);
    println!("{}", s);

    println!("Enter vec[u32]: ");
    read_vec!(v as u32); // Reads space separated integers and stops when newline is encountered.
    println!("{:?}", v);
}
