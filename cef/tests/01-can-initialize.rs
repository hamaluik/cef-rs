#[cfg(test)]
mod test {
    use cef::Cef;

    #[test]
    fn can_initialize() {
        let cef = Cef::initialize();
        drop(cef);
    }
}
