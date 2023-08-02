use reqwest;

use crate::configuration::BotInterface;

pub fn get_random_comic(bot_interface: BotInterface) -> &'static str {
    let body = reqwest::get(bot_interface.bot_settings.xkcd_url);
    "asd"
}
