use std::path::PathBuf;

pub enum ContextInfo
{
    Window(String),
    Asset(PathBuf)
}

