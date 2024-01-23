use teloxide::dispatching::dialogue::InMemStorage;
use teloxide::prelude::Dialogue;

#[derive(Clone, Default)]
pub enum StartupState {
    #[default]
    Start,
    HandleInstituteID,
    ReceivedInstituteID {
        institute_id: i32,
    },
}

pub type StartupDialogue = Dialogue<StartupState, InMemStorage<StartupState>>;
