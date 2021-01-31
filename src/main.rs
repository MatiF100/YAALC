mod terminal;
mod db;

#[tokio::main]
async fn main(){
    println!("{:#?}", db::test().await);

    //terminal::create_terminal()?;

}
