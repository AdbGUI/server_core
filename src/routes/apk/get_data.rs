use crate::{config_path, errors::Error, model::reply::GenericReply};
use actix_web::web::{self, Json};
use std::{fs, path::Path};

use super::{get_state, ApkServiceConfig, CONFIG_FILE};

pub async fn apk_get_data() -> actix_web::Result<Json<GenericReply<String>>, Error> {
    let mut state = get_state().lock().unwrap();
    if state.is_empty() {
        let config_file = format!("{}/{}", config_path(), CONFIG_FILE);
        let cfg_path = Path::new(config_file.as_str());

        if cfg_path.exists() {
            let cfg_raw = fs::read_to_string(cfg_path).unwrap();
            *state = serde_json::from_str::<ApkServiceConfig>(cfg_raw.as_str()).unwrap_or_default();

            Ok(web::Json(GenericReply::<String>::ok(
                "".to_string(),
                serde_json::to_string(&*state).unwrap(),
            )))
        } else {
            Ok(web::Json(GenericReply::<String>::err_internal(
                "No apk uploaded".to_string(),
                "".to_string(),
            )))
        }
    } else {
        Ok(web::Json(GenericReply::<String>::ok(
            "".to_string(),
            serde_json::to_string(&*state).unwrap(),
        )))
    }
}
