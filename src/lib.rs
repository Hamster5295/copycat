mod config;

use kovi::{
    PluginBuilder as plugin, RuntimeBot, bot::runtimebot::kovi_api::SetAccessControlList,
    event::GroupMsgEvent, log::info, tokio::sync::Mutex,
};
use std::{
    collections::HashMap,
    sync::{Arc, OnceLock},
};

use crate::config::CONFIG;

static BOT: OnceLock<Arc<RuntimeBot>> = OnceLock::new();
static STATE: OnceLock<Arc<Mutex<HashMap<i64, State>>>> = OnceLock::new();

struct State {
    text: String,
    count: u32,
    sender: i64,
}

#[kovi::plugin]
async fn init() {
    let bot = BOT.get_or_init(|| plugin::get_runtime_bot());
    let config = config::init(bot).await.unwrap();

    STATE.get_or_init(|| Arc::new(Mutex::new(HashMap::new())));

    match &config.allow_groups {
        Some(groups) => {
            bot.set_plugin_access_control("copycat", true).unwrap();
            bot.set_plugin_access_control_list(
                "copycat",
                true,
                SetAccessControlList::Adds(groups.clone()),
            )
            .unwrap();
        }
        None => {
            bot.set_plugin_access_control("copycat", false).unwrap();
        }
    }

    plugin::on_group_msg(on_group_msg);
}

async fn on_group_msg(event: Arc<GroupMsgEvent>) {
    let state = STATE.get().unwrap().clone();
    let config = CONFIG.get().unwrap();

    let msgs = event.message.get("text");
    if msgs.len() > 1
        || msgs.is_empty()
        || !event.message.clone().into_iter().all(|m| m.type_ == "text")
    {
        info!("[copycat] Not a plain text message, ignored.");
        return;
    }

    let msg = msgs[0].data["text"].as_str().unwrap().to_string();

    {
        let mut s = state.lock().await;
        let s = match s.get_mut(&event.group_id) {
            Some(s) => s,
            None => {
                s.insert(
                    event.group_id,
                    State {
                        text: String::new(),
                        count: 0,
                        sender: -1,
                    },
                );
                s.get_mut(&event.group_id).unwrap()
            }
        };

        if s.text == msg {
            if s.sender != event.sender.user_id {
                s.count += 1;
                info!(
                    "[copycat] Received repeated message, current count: {} / {}",
                    s.count, config.repeat_after
                );
            }
        } else {
            s.text = msg;
            s.count = 1;
            s.sender = event.sender.user_id;
        }

        if s.count == config.repeat_after {
            BOT.get().unwrap().send_group_msg(event.group_id, &s.text);
            s.count += 1;

            info!(
                "[copycat] Meow! Current count: {} / {}",
                s.count, config.repeat_after
            );
        }
    }
}
