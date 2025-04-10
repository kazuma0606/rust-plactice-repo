cargo run -p plactice-2 -- --file plactice-2/src/test.json 
<!-- with schema success pattern -->
cargo run -p plactice-2 -- --file plactice-2/src/test.json --schema plactice-2/src/schema.json 
<!-- with schema failed pattern -->
cargo run -p plactice-2 -- --file plactice-2/src/test2.json --schema plactice-2/src/schema.json 