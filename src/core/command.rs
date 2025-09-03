use crate::core::SerializeEnum;

pub trait Command: SerializeEnum {
    fn as_any(&self) -> &dyn std::any::Any;
}

pub trait CommandDispatch<T>
where
    T: Command,
{
    fn push_command(&mut self, command: T);
}

pub trait CommandHandler<T>
where
    T: Command,
{
    fn handle_commands(&mut self, commands: &Vec<T>);
}

pub trait ExtCommandDispatch {
    fn push_ext_commands(&mut self, filter: String, command: Box<dyn Command>);
}

pub trait ExtCommandHandler
{
    fn handle_ext_command(&mut self, commands: Pair<Vec<Box<dyn Command>>);
}

pub struct CommandSystem {
    pub ext_commands: Vec<Box<dyn Command>>,
}