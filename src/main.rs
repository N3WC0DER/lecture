use diesel::associations::HasTable;
use diesel::prelude::*;
use diesel::r2d2::{ConnectionManager, Pool};
use diesel::{ExpressionMethods, PgConnection, RunQueryDsl};
use dotenv::dotenv;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::macros::BotCommands;
use teloxide::prelude::*;

use crate::db::get_connection_pool;
use crate::models::{NewUser, User};
use crate::schema::users::dsl::users;

mod db;
mod models;
mod schema;

#[tokio::main]
async fn main() {
    dotenv().ok();
    pretty_env_logger::init();
    log::info!("Starting echo bot...");

    let pool = get_connection_pool();
    log::debug!("Connection established.");

    let bot = Bot::from_env();

    log::debug!("Initializing the dispatcher...");
    Dispatcher::builder(bot, schema())
        .dependencies(dptree::deps![InMemStorage::<StartupState>::new(), pool])
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>().branch(
        case![StartupState::ReceivedGroup { group }]
            .branch(case![Command::MyGroup].endpoint(show_group)),
    );

    let message_handler = Update::filter_message()
        .branch(case![StartupState::Start].endpoint(start))
        .branch(case![StartupState::HandleGroup].endpoint(handle_group))
        .branch(command_handler)
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query();

    dialogue::enter::<Update, InMemStorage<StartupState>, StartupState, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

#[derive(BotCommands, Clone)]
#[command(
    rename_rule = "lowercase",
    description = "These commands are supported:"
)]
enum Command {
    #[command(description = "Display first message.")]
    MyGroup,
}

#[derive(Clone, Default)]
enum StartupState {
    #[default]
    Start,
    HandleGroup,
    ReceivedGroup {
        group: String,
    },
}

type StartupDialogue = Dialogue<StartupState, InMemStorage<StartupState>>;
type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;
type DatabasePool = Pool<ConnectionManager<PgConnection>>;

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Unable to handle the message.")
        .await?;
    Ok(())
}

async fn show_group(bot: Bot, group: String, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, group).await?;
    bot.send_message(msg.chat.id, msg.chat.id.0.to_string())
        .await?;
    Ok(())
}

async fn start(
    pool: DatabasePool,
    bot: Bot,
    dialogue: StartupDialogue,
    msg: Message,
) -> HandlerResult {
    use self::schema::users::dsl::*;

    let mut connection = pool.get().unwrap();

    let result = users
        .filter(chat_id.eq(msg.chat.id.0))
        .load::<User>(&mut connection)?;

    if result.is_empty() {
        bot.send_message(msg.chat.id, "Enter the group.").await?;
        dialogue.update(StartupState::HandleGroup).await?;
    } else {
        let result = result.first().unwrap().group.to_string();

        bot.send_message(msg.chat.id, "I welcome you again.")
            .await?;
        dialogue
            .update(StartupState::ReceivedGroup { group: result })
            .await?;
    }

    Ok(())
}

async fn handle_group(
    pool: DatabasePool,
    bot: Bot,
    dialogue: StartupDialogue,
    msg: Message,
) -> HandlerResult {
    match msg.text() {
        None => {
            bot.send_message(msg.chat.id, "Please enter a plain text.")
                .await?;
        }
        Some(text) => {
            bot.send_message(msg.chat.id, format!("Good! Your group: {}", text))
                .await?;

            let new_user = NewUser {
                chat_id: msg.chat.id.0,
                name: msg.chat.username().unwrap(),
                group: text,
            };

            let mut connection = pool.get().unwrap();

            diesel::insert_into(users::table())
                .values(&new_user)
                .execute(&mut connection)?;

            dialogue
                .update(StartupState::ReceivedGroup {
                    group: text.to_string(),
                })
                .await?;
        }
    }

    Ok(())
}
