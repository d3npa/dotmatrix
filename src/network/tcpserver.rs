use cyw43::{Control, NetDriver};
use embassy_net::tcp::TcpSocket;
use embassy_net::Stack;
use embassy_time::Duration;
use embedded_io_async::Write;

use crate::copy_str_bytes;
use crate::{DATA, DISPLAYS};

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

pub async fn listen(
    stack: &'static Stack<NetDriver<'static>>,
    mut ctrl: Control<'static>,
) {
    let mut rx_buffer = [0; 4096];
    let mut tx_buffer = [0; 4096];
    let mut buf = [0; 4096];

    loop {
        let mut socket = TcpSocket::new(stack, &mut rx_buffer, &mut tx_buffer);
        socket.set_timeout(Some(Duration::from_secs(10)));

        ctrl.gpio_set(0, false).await;

        if socket.accept(1234).await.is_err() {
            continue;
        }

        ctrl.gpio_set(0, true).await;

        if let Err(_e) = socket.write_all(b"[*] welcome~\n").await {
            break;
        }

        loop {
            let n = match socket.read(&mut buf).await {
                Ok(0) => break, // eof
                Ok(n) => n,
                Err(_e) => break,
            };

            buf[n] = 0;

            if let Ok(string) = crate::get_null_term_string(&buf) {
                if string.is_empty() {
                    continue;
                }

                let status = handle_command(string.trim()).await;
                if socket.write_all(&status.mesg).await.is_err() {
                    break;
                }
            }
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
            data.weather = Some(match copy_str_bytes(a.as_bytes()) {
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
