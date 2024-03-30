use bitflags::bitflags;

pub mod users;
pub mod tokens;
pub mod posts;


bitflags! {
    struct Permissions: i64 {
        const MAKE_POST = 1 << 0;
    }
}