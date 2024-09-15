use substreams::{errors::Error, pb::substreams::Clock};
use substreams_entity_change::{pb::entity::EntityChanges, tables::Tables};

#[substreams::handlers::map]
pub fn graph_out(clock: Clock) -> Result<EntityChanges, Error> {
    let mut tables = Tables::new();
    let timestamp = clock.timestamp.as_ref().unwrap();
    let seconds = clock.timestamp.as_ref().unwrap().seconds;

    tables
        .create_row("Test", timestamp.to_string().as_str())
        .set("str", clock.number.to_string())
        .set("bytes", timestamp.to_string().as_bytes().to_vec())
        .set("bool", seconds % 2 == 0)
        .set("int", seconds as i32)
        // .set("timestamp", timestamp)
        .set_bigint("big_int", &seconds.to_string())
        .set_bigdecimal("big_dec", &0.to_string());

    Ok(tables.to_entity_changes())
}
