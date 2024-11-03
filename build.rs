fn main() {
    cxx_build::bridge("src/cpp_binding.rs")
        .file("cpp/src/dancing_link.cpp")
        .std("c++14")
        .compile("dancing-link");

    println!("cargo:rerun-if-changed=src/main.rs");
    println!("cargo:rerun-if-changed=cpp/src/dancing_link.cpp");
    println!("cargo:rerun-if-changed=cpp/include/dancing_link.h");
}