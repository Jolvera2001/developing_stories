use bevy::prelude::*;
use serde::Deserialize;
use std::collections::HashMap;

#[derive(Deserialize, Asset, TypePath)]
pub struct DialogCollection {
    dialogs: HashMap<String, Dialog>,
}

#[derive(Deserialize)]
pub struct Dialog {
    speaker: String,
    text: Vec<String>,
}

#[derive(Resource)]
pub struct ActiveDialogs {
    handle: Handle<DialogCollection>,
}
