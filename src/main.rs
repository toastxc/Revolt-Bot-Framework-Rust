// serde
//use serde::{Deserialize, Serialize};
use serde_json::{Result};


// structs
#[path = "./lib/message.rs"]
mod message;
use message::*;

#[path = "./lib/auth.rs"]
mod auth;
use auth::*;


#[path = "./lib/user.rs"]
mod user;
use user::*;

#[path = "./lib/br.rs"]
mod br;
use br::*;


// non functional - issue #18
// RevX2
//mod rev_x;
//use rev_x::*;


// misc
use rand::Rng;

// network
use futures_util::{StreamExt, SinkExt};
use tokio_tungstenite::{connect_async, tungstenite::protocol::Message};

// std
use std::{
    io::Read, 
    fs::File, 
//    thread, 
//    time, 
    str::from_utf8
};

use tokio;


// import and deserialize auth.conf
fn conf_file() -> Result<Auth> {

    let mut config_json = File::open("auth.json")
        .expect("File not found");

    let mut data_str = String::new();

     config_json.read_to_string(&mut data_str)
        .expect("Error while reading file");

     let conf: Auth = serde_json::from_str(&data_str).expect("failed to interpret conf");

     Ok(conf)
}

// deserialize messages from ws
fn message_in(raw: String) -> Result<RMessage> {


    let message: Result<RMessage> = serde_json::from_str(&raw);

    match message {
        Err(rmessage) => Err(rmessage),
        Ok(ref _rmessage) =>  Ok(message.unwrap())
    }
}


// import and deserialize bridge.json
fn bridge_init() -> Result<BrConf> {
    
    let mut config_json = File::open("bridge.json")
        .expect("bridge config file not found");

    let mut brconf_str = String::new();
    config_json.read_to_string(&mut brconf_str)
        .expect("Error while reading file");


    let config: BrConf = serde_json::from_str(&brconf_str).expect("failed to interpret brconf");

    Ok(config)
}


#[tokio::main]
async fn main()  {


    println!("booting...");
    // auth files
    let data_in = conf_file();
    let data = match data_in {
        Ok(auth) => auth,
        Err(error) => panic!("Invalid credentials, {error}")
    };

    if data.token == "" {
        panic!("Invalid credentials, bot requires a token");
    }else if data.bot_id == "" {
        panic!("Invalid credentials, bot requires an ID");
    }else if data.sudoers[0] == "" {
        println!("WARN: no sudoers found")
    };
    println!("init: auth.json");
    
    

    // bridge
    let br_in = bridge_init();
    let br = match br_in {
        Err(_br) => panic!("failed to import bridge conf\n"),
        Ok(br) => br
    };
    if br.enabled == true {
        println!("init: bridge.json");
        if br.channel_1.len() != 26{
            println!("WARN: bridge channels may not be valid");
        }else if br.channel_2.len() != 26 {
            println!("WARN: bridge channels may not be valid");
        }else if br.channel_1 == br.channel_2 {
            panic!("bridge channels cannot be the same")
        };
    };

    let token = data.token.clone();

    let url = format!("wss://ws.revolt.chat/?format=json&version=1&token={token}");

 
     websocket(url, data, br).await;

}

// establishes websocket connection
pub async fn websocket(url: String, authen: Auth, br: BrConf) {

     let (ws_stream, _response) = connect_async(url).await.expect("Failed to connect");
     println!("init: websocket");

     let (mut write, read) = ws_stream.split();

    write.send(Message::Text(r#"{
        "type": "ping",
        "data": 0
      }"#.to_string()+"\\n")).await.unwrap();

    let read_future = read.for_each(|message| async {
         
        let data = message.unwrap().into_data();
       
        let out = from_utf8(&data).unwrap().to_string();

       // moved websocket main to self contained function for ease of use 
       newmain(authen.clone(), out, br.clone()).await;
    });

    read_future.await;
}



// websocket main
// imports messages, cleans them and sends to 
// bridge and message processing
pub async fn newmain(authen: Auth, out: String, br: BrConf) {

    let inval_message = message_in(out.clone());
    let inval2 = message_in(out.clone());
    
    match inval_message {
        Err(_) => return,
        Ok(_) => print!("")

    };

    // removed error prone characters
    let message = message_clean(inval_message.expect("failed to process message"));
    
    
    message_process(authen.clone(), message).await;
    
    // br does not require 'cleaned' messages
    if br.enabled == true {
        br_main(authen.clone(), inval2.unwrap(), br).await;
    };

}
// main for bridge, WIP
pub async fn br_main(auth: Auth, input_message: RMessage, br: BrConf) {


    let (chan1, chan2) = (br.channel_1, br.channel_2);
    // removing feedback loop
    if input_message.author == auth.bot_id && input_message.masquerade != None {
        return
    };


    // channel switch
    let mut chan_rec = String::new();
    if input_message.channel == chan1 {
       chan_rec = chan2;
    }else if input_message.channel == chan2 {
       chan_rec = chan1;
    };


    
    let mut message = input_message.clone();
    
    message.channel = chan_rec;

    let mut br_masq = Masquerade {
        name: None,
        avatar: None,
        colour: None
    };
   
    // masq switch - if user has no masquerade: pull from user info API
    // else - port over masquerade details 
    if input_message.masquerade == None {

        // API get masq
        
        let user = rev_user(auth.clone(), input_message.author.clone()).await.expect("failed to GET user details");

        let pfplink = user.avatar.unwrap().id;

        let pfp = format!("https://autumn.revolt.chat/avatars/{pfplink}");

        br_masq = Masquerade {
            name: Some(user.username),
            avatar: Some(pfp),
            colour: None
        };
        
    }else {
        
        // translate masq
        br_masq = Masquerade {
            name: message.masquerade.as_ref().unwrap().name.clone(),
            avatar: message.masquerade.as_ref().unwrap().avatar.clone(),
            colour: None
        };  

    };

    // message for rev_send
    let payload = RMessagePayload {
        content: message.content.clone(),
        attachments: None,
        replies: wstoapi_reply(input_message.replies).await,
        masquerade: Some(br_masq),
    };

    rev_send(auth, message, payload).await;

}

pub async fn rev_user(auth: Auth, target: String)   -> Result<RUserFetch> {

  //  println!("rev: user");
   
    let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new()
    .get(format!("https://api.revolt.chat/users/{target}"))
    .header("x-bot-token", auth.token)
    .send().await;

    let client_res = match client {
        Ok(_) => client.unwrap().text().await.unwrap(),
        Err(_) => "Err:\n{error}".to_string()
    };


  //  println!("{:?}", client_res);

    let message: Result<RUserFetch> = serde_json::from_str(&client_res);
    match message {
        Ok(_) => return Ok(message.unwrap()),
        Err(_) => return message
    };

}

// main message engine 
pub async fn message_process(data: Auth, message_in: RMessage) {
 
    let content = message_in.content.clone();

    // validity test
    if content == None {
        return
    }else if message_in.author == data.bot_id {
        return
    };

    let message = message_clean(message_in);

    let content_vec =  content.as_ref().expect("failed to split vec").split(' ').collect::<Vec<&str>>();

    let mut content_min1 = String::new();

    for x in 0..content_vec.len() -1 {
        content_min1 += &format!("{} ", content_vec[x + 1])
    };
 
    match &content_vec[0] as &str {
        
        "?Mog" | "?mog"  => send(data, message, ":01G7MT5B978E360NB6VWAS9SJ6:".to_string()).await,
        "?ver" | "?version" => send(data, message, "**Version**\nReywen: `2.0.1`\nRevX: `2.0.2`".to_string()).await,
        "?echo" => send(data, message, content_min1).await,
        "?sendas" => sendas(data, message, content_vec).await,
        _ => return
    };

}

// masq wrapper for rev_send
pub async fn sendas(auth: Auth, message: RMessage, content_vec: Vec<&str>) {

    if content_vec.len() < 3 {
        send(auth, message, "invalid use of sendas".to_string()).await;
        return
    };
    //let from = message._id.clone();
    let masq = content_vec[1];
    let mut content = String::new();
    //content = "placeholder".to_string();

    let link = match masq {
        "bingus"    | "cheese"  | "dad" | 
        "deeznuts"  |  "insert" | "joe_biden" | 
        "valence"   | "walter"  | "woof" => format!("https://toastxc.xyz/TXCS/{masq}.jpg"),
        _ => format!("https://toastxc.xyz/TXCS/default.png")
    };

    for x in 0..content_vec.len() -2 {
        content += &format!(" {}", content_vec[x + 2]);
    };

    let masq_s = Masquerade {
        name: Some(masq.to_string()),
        avatar: Some(link),
        colour: None,
    };

 
    let replier = wstoapi_reply(message.replies.clone()).await;

    let returner = RMessagePayload {
          content: Some(content),
          replies: replier,
          attachments: None,
          masquerade: Some(masq_s)
    };

    rev_send(auth.clone(), message.clone(), returner).await;
    rev_del(auth.clone(), message.clone()).await;
}
// converts websocket replies to API compatible replies
pub async fn wstoapi_reply(input: Option<Vec<String>>) -> Option<Vec<RReplies>> {

    if input == None {
        
        return None
    
    }else {
        
        let mut repstruct = vec![];
        let iter = input.clone()?.len();

        for x in 0..iter {
            
            let input_iter = &input.as_ref().expect("failed to convert input wstoapi")[x];
            
            let reply = RReplies {
                id: input_iter.to_string(),
                mention: false,
            };
            repstruct.push(reply);
        };

        return Some(repstruct)
    };

}

// non masq wrapper for rev_send
pub async fn send(auth: Auth, message: RMessage, content: String) {

    let reply = RReplies {
        id: message._id.clone(),
        mention: false,
    };
    let payload2 = RMessagePayload {
        content: Some(content),
        replies: Some(vec![reply]),
          attachments: None,
          masquerade: None
    };

    rev_send(auth, message, payload2).await;

}


// deletes messages over http
pub async fn rev_del(auth: Auth, message: RMessage) {

    let channel = message.channel;
    let target = message._id;

    let client: std::result::Result<reqwest::Response, reqwest::Error> =
    reqwest::Client::new()
    .delete(format!("https://api.revolt.chat/channels/{channel}/messages/{target}"))
    .header("x-bot-token", auth.token)
    .send().await;

     match client {
        Ok(_) => return,
        Err(_) => println!("Err:\n{:?}", client)
    };


}
// sends messages over http
pub async fn rev_send(auth: Auth, message: RMessage, payload: RMessagePayload)  {
    
    let channel = message.channel;

    let mut random = rand::thread_rng();
    let idem: i64 = random.gen();

    let payload2 = serde_json::to_string(&payload).unwrap();

    let client: std::result::Result<reqwest::Response, reqwest::Error> = 
        reqwest::Client::new()
        .post(format!("https://api.revolt.chat/channels/{channel}/messages"))
        .header("x-bot-token", auth.token)
        .header("Idempotency-Key", idem)
        .header("Content-Type", "application/json")
        .body(payload2)
        .send().await;
 
    match client {
        //Ok(_) => println!("{}", client.unwrap().text().await.unwrap()),
        Ok(_) => return,
        Err(_) => println!("Err:\n{:?}", client)
    };
}


// cleans invalid characters such as \n and \
pub fn message_clean(mut message: RMessage) -> RMessage {

    if message.content == None {
        return message
    };

    let mut out = String::new();

    let iter = message.content.as_ref().unwrap().chars().count();
    let content = message.content.as_ref().unwrap();


    for x in 0..iter {
        let current = content.chars().nth(x);

        if current == Some('\n') {
            out += "\\n";
        }else if current == Some('\\') {
            out += "\\\\";
        }else {
            out += &current.unwrap().to_string();  
        };
    };
    message.content = Some(out);
    return message    
}
