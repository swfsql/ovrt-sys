//! `CommandSink` static instance and related types and functions.
//!
//! ovrt-sys must register some functions that are called by ovrt.
//! Those functions then must forward some events/callback calls
//! using the static instance of `CommandSink`, since there are no
//! other parameters for the events/callbacks.
//!
//! An UI user, such as an app using the druid framework, must then
//! register itself as the listener of `CommandSink`, by being the
//! first caller of `global_command_sink` and registering itself.

use once_cell::sync::OnceCell;

pub type Listener = fn(cmd: crate::cmd::Command) -> ();

pub struct CommandSink {
    pub(crate) listener: Option<Listener>,
}

impl CommandSink {
    pub fn has_listener(&self) -> bool {
        self.listener.is_some()
    }
    pub fn ignored_log(cmd: crate::cmd::Command) {
        crate::log!("(ignored) command: ", format!("{:?}", cmd));
    }
    pub fn forward_or_ignore_cmd(
        &self,
        cmd: crate::cmd::Command,
    ) {
        match self.listener {
            Some(l) => l(cmd),
            None => Self::ignored_log(cmd),
        };
    }
}

/// This function has two use-cases: getting the `CommandSink` or
/// initializing it.
///
/// The "get it" usage is simple and is used internally by ovrt-sys
/// to forward commands received from events/callbacks/etc.
///
/// The "initialize it" usage needs more elaboration:
///
/// The initialization occurs in the first global call to this
/// function. The caller may decide if a listener should be registered
/// (by passing `Some` `Listener`), otherwise, `None` `Listener` will
/// be registered.  
/// Note: This `None` `Listener` will ignore all commands, but will at
/// least log them.  
/// Note: to see if there is `Some` registered listener, you may call the
/// `has_listener` function on `CommandSink`.
///
/// So with the exception of the first global call, all other remaining
/// calls should pass in `None` as a listener parameter, since the
/// `CommandSink` would already be initialized by the first global call.  
/// Otherwise, if other calls are made passing a new `Listener` as a
/// parameter, that parameter will simply be ignored and `CommandSink` will
/// not change it's already registered `Listener` (even if it's registered
/// as `None`).
pub fn global_command_sink(
    may_set_listener: Option<Listener>,
) -> &'static CommandSink {
    static EVENT_SINK: OnceCell<CommandSink> = OnceCell::new();
    EVENT_SINK.get_or_init(|| CommandSink {
        listener: may_set_listener,
    })
}

/// Submits a new command to be forwarded to the listener, or logged.
pub fn submit_cmd(cmd: crate::cmd::Command) {
    global_command_sink(None).forward_or_ignore_cmd(cmd);
}
