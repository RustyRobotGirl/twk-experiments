
fn main() {
    #[cfg(debug_assertions)]
    dotenvy::dotenv().ok(); // load environment variables in dev
    
    println!("Hello, world!");
}
