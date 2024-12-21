#[derive(Debug, Clone)]
#[allow(unused)]
pub enum DeriveErr {
    AmbigiousDeserialize(String),
    MissingDiscriminant(String),
    Message(String),
}
