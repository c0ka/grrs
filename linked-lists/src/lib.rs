
mod persis_linked;
mod single_linked;
mod deque_linked;
mod unsafe_linked;
mod vec_list;

pub use persis_linked::List;

#[cfg(test)]
mod tests {
    #[test]
    fn it_works() {
        assert_eq!(2 + 2, 4);
    }
}
