use std::{fs, path::PathBuf};

use salvo::{handler, prelude::StatusError, writer::Json, Request, Response};
use uuid::Uuid;

use crate::{config_path, model::reply::GenericReply};

use super::{get_state, APK_FOLDER};

#[handler]
pub async fn apk_upload(req: &mut Request, res: &mut Response) {
    let file_uuid = Uuid::new_v4().to_string();
    let file_name = format!("{}.apk", file_uuid);
    let mut apk_folder = PathBuf::from(config_path());
    apk_folder.pop();
    let file_path = PathBuf::from(format!("{}/{}", apk_folder.to_str().unwrap(), APK_FOLDER));
    let state = get_state();

    fs::create_dir_all(&file_path).unwrap();

    let data = req.form_data().await.unwrap();
    for (name, value) in data.fields.iter() {
        match name.as_str() {
            "name" => {
                info!("Value of name value: {:?}", value);
                (*state.lock().unwrap()).set_app_name(value.to_string());
            }
            "version" => {
                info!("Value of version value: {:?}", value);
                (*state.lock().unwrap()).set_version(value.to_string());
            }
            &_ => {}
        }
    }

    if let Some(file) = data.files.get("files") {
        let mut dest = file_path.clone();
        dest.push(&file_name);
        info!("Copy file from {:?} to {:?}", file.path(), &dest);
        if let Err(e) = fs::copy(file.path(), &dest) {
            trace!("File Copy Failed: {}", e);
            res.set_status_error(StatusError::internal_server_error());
            return;
        } else {
            trace!("File Copy Success");
            (*state.lock().unwrap()).set_last_file(file_name);
        }
    }
    state.lock().unwrap().save();

    res.render(Json(GenericReply::ok(
        "Success apk upload".to_string(),
        "".to_string(),
    )))
}
