use super::label::TextView;
use super::RawFd;
use super::{construct_message, send_recv_msg, View};
use serde_json::json;

pub struct Button<'a> {
    id: i32,
    aid: &'a str,
    sock: RawFd,
}

impl<'a> Button<'a> {
    pub fn new(fd: RawFd, aid: &'a str, text: &str, parent: Option<i32>) -> Self {
        let mut args = json!({
            "aid": aid,
            "text": text,
        });

        if let Some(id) = parent {
            args["parent"] = json!(id);
        }

        let ret = send_recv_msg(fd, construct_message("createButton", &args));
        let id: i32 = ret.to_string().parse().unwrap();
        Button { id, aid, sock: fd }
    }
}

impl<'a> TextView for Button<'a> {}

impl<'a> View for Button<'a> {
    fn get_id(&self) -> i32 {
        self.id
    }

    fn get_aid(&self) -> &str {
        self.aid
    }

    fn get_sock(&self) -> RawFd {
        self.sock
    }
}