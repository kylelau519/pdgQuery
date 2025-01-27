pub mod pdgdb;
pub mod cli;

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_connect() -> Result<(), Box<dyn std::error::Error>> {
        let conn = pdgdb::connection::connect();
        assert!(conn.is_ok(), "Failed to connect to the database");

        let conn = conn.unwrap();
        assert!(conn.is_autocommit(), "Connection is not in autocommit mode");
        Ok(())
    }

}