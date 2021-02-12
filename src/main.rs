mod terminal;
mod anilist;

#[tokio::main]
async fn main(){
    println!("{:#?}", anilist::test().await);

    //terminal::create_terminal()?;

}
