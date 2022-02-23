mod append_goshuin_command;
mod create_command;
mod open_command;
mod save_command;

use append_goshuin_command::*;
use create_command::*;
use open_command::*;
use save_command::*;
use serenity::framework::standard::macros::group;

#[group]
#[commands(create, open, save, append_goshuin)]
pub struct General;
