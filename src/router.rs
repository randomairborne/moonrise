use twilight_model::application::command::Command;
pub struct Router {
    
}

pub struct CommandHierarchy {
    slashes: Vec<Command>,
    user: Vec<Command>,
    message: Vec<Command>
}

