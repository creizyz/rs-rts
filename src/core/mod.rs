pub trait FromString {
    fn from_string(s: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait SerializeEnum: ToString + FromString {

}

pub trait Serialize: ToString + FromString {

}

pub trait Action: SerializeEnum {

}

pub mod camera;
pub mod input;
pub mod command;