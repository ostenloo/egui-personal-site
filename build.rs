fn main() {
    println!("cargo:rerun-if-changed=blog_posts/");
    println!("cargo:rerun-if-changed=private_blog_posts/");
}
