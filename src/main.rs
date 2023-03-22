use std::io;

#[tokio::main]
async fn main() -> io::Result<()> {
    // rustcraft::net::listener::Listener::new()
    //     .bind("0.0.0.0:25565")
    //     .await?;

    let mut c = vec![1, 2, 3, 4, 5, 6, 7, 8, 9];
    let mut a = c.iter();

    println!("one = {}", a.next().unwrap());
    println!("two = {}", a.next().unwrap());
    println!("three = {}", a.next().unwrap());
    let b: Vec<&i32> = a.collect();
    println!("nexts = {:?}", b);
    Ok(())
}
