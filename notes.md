SQL Stuff

SQLX
https://medium.com/@r.das699/an-example-of-connecting-to-a-sqlite-database-using-rust-cdeb363a277a

Rusqlite
https://tedspence.com/investigating-rust-with-sqlite-53d1f9a41112
https://rust-lang-nursery.github.io/rust-cookbook/database/sqlite.html

https://docs.rs/rusqlite/latest/rusqlite/

# Script for sqlite

```sql
CREATE TABLE IF NOT EXISTS "todo" (
	"id"	INTEGER NOT NULL,
	"name"	TEXT NOT NULL,
	"date_added"	REAL NOT NULL DEFAULT current_timestamp,
	"is_done"	NUMERIC NOT NULL DEFAULT 0,
	PRIMARY KEY("id" AUTOINCREMENT)
);
```

========================================================================================================

Sled is a Rust written KV store.
https://docs.rs/sled/latest/sled/

Ditch because it needs the connection to stay open for a long time, otherwise it throws on Linux and MacOS.
Sled references:

https://github.com/atomicdata-dev/atomic-server/blob/master/lib/src/db.rs
https://github.com/IamfromSpace/rust-cycle/blob/master/src/telemetry_db.rs

========================================================================================================

The ./db/ folder has to be created in the repo with a .gitkeep file, so it's also created when the project is ran for
the first time.

If it doesn't existe, Rusqlite won't be able to create a new database.

========================================================================================================

To format dates with UTC-3
extern crate chrono;

use chrono::{Datelike, Timelike, Utc, TimeZone, Local};

fn main() {
// Input string
let input_str = "2023-10-27 12:53:33";

    // Parse the input string as a date and time
    let parsed_datetime = chrono::DateTime::parse_from_str(input_str, "%Y-%m-%d %H:%M:%S")
        .expect("Failed to parse datetime");

    // Adjust the datetime to the UTC-3 time zone
    let utc_minus3 = Utc.ymd(parsed_datetime.year(), parsed_datetime.month(), parsed_datetime.day())
        .and_hms(parsed_datetime.hour(), parsed_datetime.minute(), parsed_datetime.second())
        .with_timezone(&chrono::FixedOffset::east(3 * 3600));

    // Format the datetime in "DD/MM/YYYY-hh:mm" UTC-3 format
    let formatted_datetime = utc_minus3.format("%d/%m/%Y-%H:%M").to_string();

    println!("Formatted datetime in UTC-3: {}", formatted_datetime);

}
