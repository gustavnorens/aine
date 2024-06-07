use std::process::Command;
use std::env;

use serenity::async_trait;
use serenity::model::channel::Message;
use serenity::model::gateway::Ready;
use serenity::prelude::*;

struct Handler {
    gen_sol_path : String,
    generator : String,
    solution : String
}

impl Handler {
    fn new() -> Self {
        Handler {
            gen_sol_path : env::var("GEN_SOL_PATH").expect("Expected a path in the environment"),
            generator : env::var("GENERATOR").expect("Expected a generator in the environment"),
            solution : env::var("SOLUTION").expect("Expected a solution in the environment")
        }
    }
}

#[async_trait]
impl EventHandler for Handler {
    async fn message(&self, ctx : Context, msg : Message) {
        if msg.content == "!input" { //Make it check only specific channels
            let requestee = msg.author;

            let generated_input = Command::new(format!("{}/{}", self.gen_sol_path, self.generator))
                .arg(requestee.id.get().to_string())
                .output()
                .expect("Failed to get input!");
            
            
            let dm = requestee.create_dm_channel(&ctx.http).await.expect("Error creating dm channel");
            dm.say(&ctx.http, String::from_utf8(generated_input.stdout).expect("Couldn't convert input to String.")).await.expect("Failed to send message");
        }
    }

    async fn ready(&self, _ : Context, ready : Ready) {
        println!("{} is connected!", ready.user.name);
    }
}



#[tokio::main]
async fn main() {
    let token = env::var("TOKEN").expect("Expected a token in the environment");

    let intents = GatewayIntents::GUILD_MESSAGES | GatewayIntents::MESSAGE_CONTENT;


    let mut client = Client::builder(&token, intents).event_handler(Handler::new()).await.expect("Err creating client");

    client.start().await.expect("Couldn't start client");   
}