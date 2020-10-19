use std::env;
use dotenv::dotenv;
use serenity::{
    async_trait,
    model::{channel::Message, gateway::Ready},
    prelude::*,
};
use rand::seq::SliceRandom;

const KNOWN_USERS: [&str; 5] = ["reva", "sparsh", "revz", "spoosh", "divy"];
const IS_ADJ: [&str; 9] = ["Mind-blowing!", "Awesome!", "the Best!", "Remarkable!", "Marvellous!", "Majestic!", "Astonishing!", "Superb!", "Spectacular!"];


fn generate_msg_ily(name: &str) -> String {
    if rand::random() {
        let adj: &str = IS_ADJ.to_vec().choose(&mut rand::thread_rng()).unwrap_or(&"Lovely!");
        return name.to_string() + " is " + adj;   
    }
    return "I love you, ".to_owned() + name
}

struct Handler;

#[async_trait]
impl EventHandler for Handler {
    // Set a handler for the `message` event - so that whenever a new message
    // is received - the closure (or function) passed will be called.
    //
    // Event handlers are dispatched through a threadpool, and so multiple
    // events can be dispatched simultaneously.
    async fn message(&self, ctx: Context, msg: Message) {
        // A simple workaround to not respond to itself
        //
        // TODO(littledivy): Check docs for a better way?
        if msg.author.name == "cherry" {
            return
        }
        if KNOWN_USERS.iter().any(|&i| msg.content.contains(i)) {
            if let Err(why) = msg.react(&ctx.http, serenity::model::channel::ReactionType::Unicode(":hearts:".to_string())).await {
                println!("Error reacting to message: {:?}", why);
            }
            if let Err(why) = msg.channel_id.say(&ctx.http, generate_msg_ily(&msg.author.name)).await {
                println!("Error sending message: {:?}", why);
            }
        }
        if msg.content == "!ily" {
            // Sending a message can fail, due to a network error, an
            // authentication error, or lack of permissions to post in the
            // channel, so log to stdout when some error happens, with a
            // description of it.
            if let Err(why) = msg.channel_id.say(&ctx.http, "**I'm up!** Contact `divy#8575` for reporting bugs or security vulnerabilities.").await {
                println!("Error sending message: {:?}", why);
            }
        }
    }

    // Set a handler to be called on the `ready` event. This is called when a
    // shard is booted, and a READY payload is sent by Discord. This payload
    // contains data like the current user's guild Ids, current user data,
    // private channels, and more.
    //
    // In this case, just print what the current user's username is.
    async fn ready(&self, _: Context, ready: Ready) {
        println!("{} is connected!", ready.user.name);
    }
}

#[tokio::main]
async fn main() {
    dotenv().ok();
    // Configure the client with your Discord bot token in the environment.
    let token = env::var("DISCORD_TOKEN")
        .expect("Expected a token in the environment");

    // Create a new instance of the Client, logging in as a bot. This will
    // automatically prepend your bot token with "Bot ", which is a requirement
    // by Discord for bot users.
    let mut client = Client::new(&token)
        .event_handler(Handler)
        .await
        .expect("Err creating client");

    // Finally, start a single shard, and start listening to events.
    //
    // Shards will automatically attempt to reconnect, and will perform
    // exponential backoff until it reconnects.
    if let Err(why) = client.start().await {
        println!("Client error: {:?}", why);
    }
}