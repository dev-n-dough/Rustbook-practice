async fn print_world(){
    println!("world");
}

#[tokio::main]
async fn main() {
    let instance = print_world();

    println!("hello");

    instance.await; // e triggers execution of the `print_world` function
}
