pub trait Scope: Drop {
    fn close(mut self);
}