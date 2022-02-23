mod create_command;
mod edit_command;

use create_command::*;
use edit_command::*;
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
#[commands(create, edit)]
pub struct General;
