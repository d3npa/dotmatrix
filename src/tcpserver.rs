use crate::{DATA, DISPLAYS};
use crate::copy_str_bytes;

pub struct CommandStatus {
    pub mesg: [u8; 64],
}

impl CommandStatus {
    /// mesg may be up to 64 chars
    pub fn new(mesg: &str) -> Self {
        Self {
            /* unwrap is safe bc mesg: &str is guaranteed to be utf-8 */
            mesg: copy_str_bytes(mesg.as_bytes()).unwrap(),
        }
    }
}

/// commands are single chars, then a space, then args
pub async fn handle_command(argv: &str) -> CommandStatus {
    let (c, a) = argv.split_at(1);
    match c {
        "0" => {
            // echo
            DISPLAYS.set_override(true).await;
            DISPLAYS.alert().await;
            DISPLAYS.panorama(a, true).await;
            DISPLAYS.set_override(false).await;
            CommandStatus::new("[*] echoing message\n")
        }
        "1" => {
            // store clk
            let mut data = DATA.lock().await;
            data.clock = Some(match copy_str_bytes(a.as_bytes()) {
                Ok(v) => v,
                Err(_) => return CommandStatus::new("[*] error :c"),
            });
            CommandStatus::new("[*] clock set! ^-^\n")
        }
        "2" => {
            // store weather
            let mut data = DATA.lock().await;
            data.weather =
                Some(match copy_str_bytes(a.as_bytes()) {
                    Ok(v) => v,
                    Err(_) => return CommandStatus::new("[*] error :c"),
                });
            CommandStatus::new("[*] weather set! ^-^\n")
        }
        _ => {
            // unknown command
            CommandStatus::new("[*] unknown command :c\n")
        }
    }
}
