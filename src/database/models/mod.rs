use bitflags::bitflags;

pub mod users;
pub mod tokens;
pub mod posts;

#[derive(Debug, Clone)]
pub struct Permissions(i64);

bitflags! {
    impl Permissions: i64 {
        const MAKE_POST = 1 << 0;
    }
}