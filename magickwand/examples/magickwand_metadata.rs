fn main() {
    println!("{}\nPackage: {}\nVersion: {}\nQuantum: {} {}",
        magickwand::get_copyright().unwrap(),
        magickwand::get_package_name().unwrap(),
        magickwand::get_version().unwrap(),
        magickwand::get_quantum_depth().unwrap(),
        magickwand::get_quantum_range().unwrap()
    )
}