use crate::app;
use oauth2::basic::BasicClient;
use oauth2::{AccessToken, AuthUrl, ClientId, CsrfToken};
use std::io::{BufRead, BufReader, Write};
use std::net::TcpListener;
use std::time::Duration;
use url::Url;
use webbrowser;

//Authorization function, returning AuthToken as defined in app.rs
pub fn auth() -> Option<app::AuthToken> {
    //Preparing data for Oauth request
    let anilist_client_id = ClientId::new("4899".to_owned());
    let auth_url = AuthUrl::new("https://anilist.co/api/v2/oauth/authorize".to_string())
        .expect("Invalid authorization endpoint URL");

    //Setting up authorization client
    let client = BasicClient::new(anilist_client_id, None, auth_url, None);

    // Generate the authorization URL to which we'll redirect the user.
    let (authorize_url, csrf_state) = client
        .authorize_url(CsrfToken::new_random)
        .use_implicit_flow()
        .url();

    //Opening authorization url in default system browser
    webbrowser::open(authorize_url.as_str()).unwrap();
    println!("Waiting for authorization...");

    // A very naive implementation of the redirect server.
    //It listens for respones at localhost, at port configured in API settings
    let listener = TcpListener::bind("127.0.0.1:8080").unwrap();
    for stream in listener.incoming() {
        if let Ok(mut stream) = stream {
            let token;
            let state;
            let exp_time;
            {
                let mut reader = BufReader::new(&stream);

                let mut request_line = String::new();
                reader.read_line(&mut request_line).unwrap();

                //println!("{:?}", request_line);
                let redirect_url = request_line.split_whitespace().nth(1).unwrap();
                let url = Url::parse(&("http://localhost".to_string() + redirect_url)).unwrap();

                //If URL contains no ? but #, it is parsed to ? using JS script
                if url.query().is_none() {
                    let message = "
<!doctype html>
<html lang='en'>
<head>
<meta charset='utf-8'>

    <title>Parsing Token...</title>
    <meta name='description' content='Token Parser'>
</head>
<body>
<script>
    let query = window.location.hash.slice(1)
    window.open('http://localhost:8080/?' + query,'_self')
</script>
</body>
</html>
                    ";
                    let response = format!(
                        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                        message.len(),
                        message
                    );
                    stream.write_all(response.as_bytes()).unwrap();
                }
                //If URL does contain ?, it is being treated as anilist response
                else {
                    let message = "
<!doctype html>
<html lang='en'>
<head>
<meta charset='utf-8'>

    <title>Token Parsed!</title>
    <meta name='description' content='Token Parser'>
</head>
<body>
    <p>You can now return to your client</p>
</body>
</html>
                    ";
                    let response = format!(
                        "HTTP/1.1 200 OK\r\ncontent-length: {}\r\n\r\n{}",
                        message.len(),
                        message
                    );
                    stream.write_all(response.as_bytes()).unwrap();

                    //Getting the Auth Token from the query
                    let code_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "access_token"
                        })
                        .unwrap();

                    let (_, value) = code_pair;
                    token = AccessToken::new(value.into_owned());

                    //Getting the expiration time from the query
                    let code_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "expires_in"
                        })
                        .unwrap();

                    let (_, value) = code_pair;
                    exp_time = Duration::from_secs(value.parse::<u64>().unwrap()).as_secs();

                    //Getting the state, to check for response correctness
                    let state_pair = url
                        .query_pairs()
                        .find(|pair| {
                            let &(ref key, _) = pair;
                            key == "state"
                        })
                        .unwrap();
                    let (_, value) = state_pair;
                    state = CsrfToken::new(value.into_owned());

                    /*
                    println!(
                        "Anilist returned the following token:\n{}\n",
                        token.secret()
                    );
                    */
                    //Printing recieved and expected state
                    println!(
                        "Anilist returned the following state:\n{} (expected `{}`)\n",
                        state.secret(),
                        csrf_state.secret()
                    );

                    //Returning AuthToken created based on recived token and it's expiration time
                    return Some(app::AuthToken::from_args(token, exp_time));
                }
            }
        }
    }
    None
}
