use crate::{config_path, model::reply::GenericReply};
use salvo::{handler, Request, Response, writer::Json};
use std::{fs, path::Path};

use super::{get_state, ApkServiceConfig, CONFIG_FILE};

#[handler]
pub async fn apk_get_data(_req: &mut Request, res: &mut Response) {
    let mut state = get_state().lock().unwrap();
    if state.is_empty() {
        let config_file = format!("{}/{}", config_path(), CONFIG_FILE);
        let cfg_path = Path::new(config_file.as_str());

        if cfg_path.exists() {
            let cfg_raw = fs::read_to_string(cfg_path).unwrap();
            *state = serde_json::from_str::<ApkServiceConfig>(cfg_raw.as_str()).unwrap_or_default();

            res.render(Json(GenericReply::<String>::ok(
                "".to_string(),
                serde_json::to_string(&*state).unwrap(),
            )))
        } else {
            state.save();
            info!("Not exist apk data file");
            res.render(Json(GenericReply::<String>::ok(
                "No apk data".to_string(),
                serde_json::to_string(&*state).unwrap(),
            )));
        }
    } else {
        state.save();
        info!("Apk data is cached");
        res.render(Json(GenericReply::<String>::ok(
            "".to_string(),
            serde_json::to_string(&*state).unwrap(),
        )));
    }
}
