use std::collections::HashMap;

use diesel::prelude::*;
use diesel::{ExpressionMethods, RunQueryDsl};
use dotenv::dotenv;
use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::dispatching::{dialogue, UpdateHandler};
use teloxide::prelude::*;
use teloxide::types::{InlineKeyboardButton, InlineKeyboardMarkup};

use crate::commands::Command;
use crate::db::{get_connection_pool, DatabasePool};
use crate::models::*;
use crate::states::{StartupDialogue, StartupState};

mod commands;
mod db;
mod models;
mod schema;
mod states;

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
        .error_handler(LoggingErrorHandler::with_custom_text(
            "An error has occurred in the dispatcher",
        ))
        .enable_ctrlc_handler()
        .build()
        .dispatch()
        .await;
}

fn schema() -> UpdateHandler<Box<dyn std::error::Error + Send + Sync + 'static>> {
    use dptree::case;

    let command_handler = teloxide::filter_command::<Command, _>().branch(
        case![StartupState::ReceivedInstituteID { institute_id }]
            .branch(case![Command::MyInstitute].endpoint(show_group)),
    );

    let message_handler = Update::filter_message()
        .branch(case![StartupState::Start].endpoint(start))
        .branch(command_handler)
        .branch(dptree::endpoint(invalid_state));

    let callback_query_handler = Update::filter_callback_query()
        .branch(case![StartupState::HandleInstituteID].endpoint(handle_institute));

    dialogue::enter::<Update, InMemStorage<StartupState>, StartupState, _>()
        .branch(message_handler)
        .branch(callback_query_handler)
}

type HandlerResult = Result<(), Box<dyn std::error::Error + Send + Sync>>;

async fn invalid_state(bot: Bot, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, "Unable to handle the message.")
        .await?;
    Ok(())
}

async fn show_group(bot: Bot, institute_id: i32, msg: Message) -> HandlerResult {
    bot.send_message(msg.chat.id, institute_id.to_string())
        .await?;
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
        .limit(1)
        .load::<User>(&mut connection)?;

    if result.is_empty() {
        let institutes = ["ONE", "TWO", "THREE"]
            .map(|institute| InlineKeyboardButton::callback(institute, institute));

        bot.send_message(msg.chat.id, "Choose the name of the institute.")
            .reply_markup(InlineKeyboardMarkup::new([institutes]))
            .await?;

        dialogue.update(StartupState::HandleInstituteID).await?;
    } else {
        let result = result.first().unwrap().institute_id;

        bot.send_message(msg.chat.id, "I welcome you again.")
            .await?;
        dialogue
            .update(StartupState::ReceivedInstituteID {
                institute_id: result,
            })
            .await?;
    }

    Ok(())
}

async fn handle_institute(
    pool: DatabasePool,
    bot: Bot,
    dialogue: StartupDialogue,
    query: CallbackQuery,
) -> HandlerResult {
    use self::schema::*;

    let institutes = HashMap::from([
        ("ONE".to_string(), 1),
        ("TWO".to_string(), 2),
        ("THREE".to_string(), 3),
    ]);

    let institute_id = institutes[&query.data.expect("wtf?")];

    bot.answer_callback_query(query.id).await?;

    let new_user = if let Some(Message { id, chat, .. }) = query.message {
        bot.edit_message_text(
            chat.id,
            id,
            format!("Good! Your institute: {}", &query.data.expect("wtf?")),
        )
        .await?;

        User {
            chat_id: chat.id.0,
            username: chat.username().unwrap().to_string(),
            moderator: false,
            institute_id,
            course: 0,
            direction_id: 0,
            notification: false,
        }
    } else {
        panic!("wtf2?");
    };

    let mut connection = pool.get().unwrap();

    diesel::insert_into(users::table)
        .values(&new_user)
        .execute(&mut connection)?;

    dialogue
        .update(StartupState::ReceivedInstituteID { institute_id })
        .await?;

    Ok(())
}
