mod create_command;
mod open_command;

use create_command::*;
use open_command::*;
use serenity::{
    framework::standard::{
        buckets::{LimitedFor, RevertBucket},
        help_commands,
        macros::{check, command, group, help, hook},
        Args, CommandGroup, CommandOptions, CommandResult, DispatchError, HelpOptions, Reason,
        StandardFramework,
    },
    model::{channel::Message, id::UserId},
    prelude::*,
};

#[group]
#[commands(create, open)]
pub struct General;
