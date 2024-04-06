fn main() {
    println!("cargo::rustc-cdylib-link-arg=/DEF:sppc.def");
    println!("cargo::rustc-cdylib-link-arg=/Brepro");
    println!("cargo::rustc-cdylib-link-arg=/emittoolversioninfo:no");
}
