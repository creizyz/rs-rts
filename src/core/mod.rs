pub mod camera;
pub mod input;

pub trait SerializableEnum {
    fn to_string(&self) -> &'static str;
    fn from_string(string: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait Serializable {
    fn to_string(&self) -> String;
    fn from_string(string: &str) -> Option<Self>
    where
        Self: Sized;
}

pub trait Action: SerializableEnum {

}
