To setup the database run `cat schema.sql | sqlite3 twitter.db`
To run the application run `cat schema.sql | sqlite3 twitter.db && cargo run`
To test the application run `cat schema.sql | sqlite3 fake.db && cargo test --test-threads=1`