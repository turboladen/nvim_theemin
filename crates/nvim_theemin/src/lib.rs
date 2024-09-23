
pub mod ui;

pub use bit_struct;

pub trait AsGroupName {
    fn as_group_name(&self) -> &'static str;
}
