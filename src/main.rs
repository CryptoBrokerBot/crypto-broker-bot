use dotenv;
use std::env;
use serenity::async_trait;
use serenity::client::{Client, Context, EventHandler};
use serenity::model::channel::Message;
use serenity::framework::standard::{
    StandardFramework,
    CommandResult,
    DispatchError,
    macros::{
        command,
        group,
        hook
    }
};

pub mod types;
pub mod api;

#[group]
#[commands(ping,list,daily_reward,balance,purchase)]
struct General;

struct Handler;

#[async_trait]
impl EventHandler for Handler {}

#[tokio::main]
async fn main() {
    dotenv::dotenv().ok();

    let framework = StandardFramework::new()
        .configure(|c| c.prefix("~")) // set the bot's prefix to "~"
        .group(&GENERAL_GROUP);
    
    let api_base_url = dotenv::var("CB_APIBASEURL").expect("CB_APIBASEURL is missing");
    let api_key = dotenv::var("CB_APIKEY").expect("CB_APIKEY is missing");
    
    // Login with a bot token from the environment
    let token = dotenv::var("CB_DISCORDTOKEN").expect("CB_DISCORDTOKEN is missing");
    let mut client = Client::builder(token)
        .type_map_insert::<types::ApiKeyTypeMapKey>(api_key)
        .type_map_insert::<types::ApiBaseUrlTypeMapKey>(api_base_url)
        .event_handler(Handler)
        .framework(framework)
        .await
        .expect("Error creating client");
    
    // start listening for events by starting a single shard
    if let Err(why) = client.start().await {
        println!("An error occurred while running the client: {:?}", why);
    }
}

async fn api_connection_details(ctx : &Context) -> types::ApiConnectionDetails {
    let map = (*ctx.data).read().await;
    let api_key = map.get::<types::ApiKeyTypeMapKey>().expect("Missing API key from TypeMap.").clone();
    let api_base_url = map.get::<types::ApiBaseUrlTypeMapKey>().expect("Missing API base url from TypeMap.").clone();
    types::ApiConnectionDetails {
        api_key, api_base_url
    }
}

#[command]
async fn ping(ctx: &Context, msg: &Message) -> CommandResult {
    msg.reply(&ctx.http, "Pong").await?;
    Ok(())
}

#[command]
#[aliases(daily)]
async fn daily_reward(ctx : &Context, msg : &Message) -> CommandResult {
    let connection_details = api_connection_details(&ctx).await;
    if let Err(err) = api::daily_reward(msg.author.id.0, &connection_details).await {
        println!("{}", err);
    }
    msg.reply(&ctx.http, "You have claimed your daily reward!").await?;
    Ok(())
}

#[command]
#[aliases(bal)]
async fn balance(ctx: &Context, msg: &Message) -> CommandResult {
    let api_details = api_connection_details(&ctx).await;
    let balance_result = api::balance(msg.author.id.0, &api_details).await;
    match balance_result {
        Ok(balance) => { msg.reply(&ctx.http, format!("Your balance is ${:.2}", balance.balance)).await?; },
        Err(e) => println!("err: {}", e)
    }
    Ok(())
}

#[command]
#[aliases(buy)]
async fn purchase(ctx : &Context, msg : &Message) -> CommandResult {
    let api_details = api_connection_details(&ctx).await;
    let args : Vec<&str> = msg.content.split_whitespace().collect();
    if args.len() < 3 {
        msg.reply(&ctx.http, "Expected: ~purchase [amount] [symbol], i.e. ~purchase 10 btc").await?;
        return Ok(());
    }
    if let Ok(qty) = args[1].parse::<u64>() {
        let symbol = args[2].clone();
        if let Err(e) = api::buy(msg.author.id.0, qty, symbol, &api_details).await {
            println!("err: {}", e);
            msg.reply(&ctx.http, "Failed to perform transaction. Please try again later.").await?;
        } else {
            msg.reply(&ctx.http, "Transaction complete.").await?;
        }
    } else {
        msg.reply(&ctx.http, "Invalid amount. Expected: ~purchase [amount] [symbol], i.e. ~purchase 10 btc").await?;
    }
    Ok(()) 
}

#[command]
async fn list(ctx: &Context, msg: &Message) -> CommandResult {
    let args : Vec<&str> = msg.content.split_whitespace().collect();
    let mut page : usize = 1;
    if args.len() > 1 {
        page = args.iter().skip(1).next().unwrap().parse::<usize>()?;
        if page > 8 {
            page = 8;
        }
    }
    let api_details = api_connection_details(&ctx).await;
    let top_list = api::list(&api_details).await?;
    msg.channel_id.send_message(&ctx.http, move |m| {
        m.embed(move |e| {
            let fields : Vec<(String, String, bool)> = top_list.iter().skip((page - 1) * 25).take(25).map(|cd| (format!("({}) {}", cd.symbol, cd.name), format!("{:.05}", cd.price), true)).collect();
            e.title("Top 25 Cryptocurrencies");
            e.description(format!("Page {} of 8", page));
            e.fields(fields);
            e
        })
    }).await?;
    Ok(())
}
