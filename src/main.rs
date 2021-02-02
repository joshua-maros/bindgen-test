use std::path::PathBuf;

fn main() {
    let path = PathBuf::from("C:\\msys64\\mingw64\\bin\\libclang.dll");
    let lib = libloading::Library::new(&path);
    if let Err(e) = lib {
        println!("{:?}", e);
    } else {
        panic!("It worked fine?");
    }
}
