#[cfg(test)]
mod tests {
    enum _Error {
        _NotANumber(String),
    }

    #[test]
    fn ok() {
        // let a: Result<Option<String>, Error> = todo!();
        // let b: Result<Option<u32>, Error> = todo!();

        // let op = |s: String| -> Result<u32, Error> { s.parse().map_err(|_| Error::NotANumber(s.clone())) };
        // a.apply(op) == b
    }
}
