use std::path::PathBuf;

pub enum ContextInfo
{
    Window(String),
    Asset{ registry_path: PathBuf }
}

