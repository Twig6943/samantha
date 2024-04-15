use base64::{engine::general_purpose::URL_SAFE, Engine};
use serde::{Deserialize, Serialize};
use steamworks::Client;

use crate::utils::request_current_stats;

#[derive(Serialize, Deserialize, Debug)]
pub struct Achievement {
    name: String,
    unlocked: bool,
    screen_name: Option<String>,
    description: Option<String>,
    hidden: bool,
    image: String,
}

#[tauri::command]
pub async fn get_achievement_list(appid: u32) -> Result<Vec<Achievement>, String> {
    let init = Client::init_app(appid);
    match init {
        Ok((client, single_client)) => {
            request_current_stats(&client, single_client).await;
            let ach_array: Vec<Achievement> = client
                .user_stats()
                .get_achievement_names()
                .unwrap_or(vec![])
                .into_iter()
                .map(|ach_name| {
                    let stats = client.user_stats();
                    let achievement = stats.achievement(&ach_name);
                    return Achievement {
                        name: ach_name,
                        unlocked: achievement.get().unwrap_or(false),
                        image: URL_SAFE.encode(achievement.get_achievement_icon().unwrap_or_default()),
                        screen_name: if let Ok(scr_name) = achievement.get_achievement_display_attribute("name")
                        {
                            Some(scr_name.to_string())
                        } else {
                            None
                        },
                        description: if let Ok(desc) = achievement.get_achievement_display_attribute("desc")
                        {
                            Some(desc.to_string())
                        } else {
                            None
                        },
                        hidden: achievement
                            .get_achievement_display_attribute("hidden")
                            .unwrap_or("0")
                            == "1",
                    };
                })
                .collect();
            return Ok(ach_array);
        },
        Err(e) => return Err(e.to_string())
    }
}
