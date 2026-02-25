

fn main(){
   // Specify the library search path
    println!("cargo:rustc-link-search=native=lib/x64");
    println!("cargo:rustc-link-search=native=../lib/x64");
    println!("cargo:rustc-link-lib=static=SDL3");
    
    // Optional: Print debug info
    println!("cargo:rerun-if-changed=lib/x64/SDL3.lib");
}