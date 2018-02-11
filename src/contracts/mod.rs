pub mod etherdelta;

pub trait NamedFunction {
    fn get_function(&self) -> ContractFunction;
}

pub enum ContractFunction {
    Immutable(String),
    Mutable(String)
}