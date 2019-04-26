use crate::models::map::Map;
use crate::models::player::Player;
use crate::models::record::*;
use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_query;

pub fn has_finished(
    connection: &MysqlConnection,
    time: i32,
    rs_count: i32,
    player_id: &str,
    map_id: &str,
) -> QueryResult<(bool, i32, i32)> {
    use crate::schema::records::dsl as records_table;

    let has_previous: Result<Record, _> = records_table::records
        .find((map_id, player_id))
        .get_result(connection);

    match has_previous {
        Ok(previous_record) => {
            let old = previous_record.time;
            let new = time;

            // update with new time
            if new < old {
                let _updated = diesel::update(&previous_record)
                    .set((
                        records_table::time.eq(new),
                        records_table::respawn_count.eq(rs_count),
                        records_table::try_count.eq(records_table::try_count + 1),
                        records_table::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(connection)?;
            }

            Ok((new < old, old, new))
        }

        _ => {
            let new = Record {
                time,
                respawn_count: rs_count,
                try_count: 1,
                created_at: Utc::now().naive_utc(),
                updated_at: Utc::now().naive_utc(),
                player_id: player_id.to_string(),
                map_id: map_id.to_string(),
            };

            let _inserted_record = diesel::insert_into(records_table::records)
                .values(new)
                .execute(connection)?;

            Ok((true, time, time))
        }
    }
}

pub fn overview(
    connection: &MysqlConnection,
    player_id: &str,
    map_id: &str,
) -> QueryResult<Vec<RankedRecord>> {
    use crate::schema::records::dsl as records_table;

    let query = format!(
        r#"
    SELECT
        RANK() OVER (ORDER BY time) as rank,
        records.player_id,
        players.nickname,
        records.time
    FROM records
    INNER JOIN players ON records.player_id=players.login
    where map_id = "{}"
    ORDER BY time;"#,
        map_id
    );

    let mut records = sql_query(query).load::<RankedRecord>(connection)?;
    let rows = 15;

    let has_record: Result<Record, _> = records_table::records
        .find((map_id, player_id))
        .get_result(connection);

    match has_record {
        Ok(player_record) => {
            let player_idx = records
                .binary_search_by(|record| record.time.cmp(&player_record.time))
                .unwrap();

            if player_idx < rows {
                records.truncate(rows);
                Ok(records)
            } else {
                let mut res: Vec<RankedRecord> = vec![];

                res.extend_from_slice(&records[0..3]);

                let row_minus_top3 = rows - 3;
                let mut start_idx = player_idx - row_minus_top3 / 2;
                let mut end_idx = player_idx + row_minus_top3 / 2;

                if end_idx >= records.len() {
                    start_idx -= records.len() - 1 - end_idx;
                    end_idx = records.len() - 1;
                }

                res.extend_from_slice(&records[start_idx..end_idx]);
                Ok(res)
            }
        }

        _ => {
            records.truncate(rows);
            Ok(records)
        }
    }
}

pub fn latest_records(
    connection: &MysqlConnection,
    offset: i64,
    limit: i64,
) -> QueryResult<Vec<(Record, Player, Map)>> {
    use crate::schema::{maps, players, records};

    let join = records::table
        .inner_join(players::table)
        .inner_join(maps::table);

    let latest_rec = join
        .offset(offset)
        .limit(limit)
        .order_by(records::updated_at.desc())
        .load(connection)?;

    Ok(latest_rec)
}

pub fn map_records(
    connection: &MysqlConnection,
    offset: i64,
    limit: i64,
    map_id: &str,
) -> QueryResult<Option<(Map, Player, Vec<RankedRecord>)>> {
    use crate::schema::{maps, players};

    let map: Option<Map> = maps::table.find(map_id).get_result(connection).optional()?;

    if map.is_none() {
        return Ok(None);
    }

    let query = format!(
        r#"
    SELECT
        RANK() OVER (ORDER BY time) as rank,
        records.player_id,
        players.nickname,
        records.time
    FROM records
    INNER JOIN players ON records.player_id=players.login
    WHERE map_id = "{}"
    ORDER BY time
    LIMIT {}
    OFFSET {};"#,
        map_id, limit, offset
    );

    let cur_map = map.unwrap();

    let records = sql_query(query).load::<RankedRecord>(connection)?;

    let player = players::table
        .find(&cur_map.player_id)
        .get_result(connection)?;
    Ok(Some((cur_map, player, records)))
}

pub fn player_records(
    connection: &MysqlConnection,
    player_id: &str,
) -> QueryResult<Option<(Player, Vec<PlayerRecord>)>> {
    use crate::schema::players;

    let player: Option<Player> = players::table
        .find(player_id)
        .get_result(connection)
        .optional()?;

    if player.is_none() {
        return Ok(None);
    }

    let query = format!(
        r#"
        WITH AllRecords AS (
            SELECT
                RANK() OVER (partition by records.map_id ORDER BY TIME) AS rank,
                records.*,
                maps.name as map_name
            FROM records
            INNER JOIN players ON records.player_id=players.login
            INNER JOIN maps ON records.map_id=maps.maniaplanet_map_id
            WHERE records.map_id IN (
                    SELECT records.map_id
                    FROM records
                    WHERE records.player_id = '{player}'
            )
        )
        SELECT *
        FROM AllRecords
        WHERE player_id = '{player}'
        ORDER BY updated_at DESC
        ;"#,
        player = player_id,
    );

    let cur_player = player.unwrap();
    let records = sql_query(query).load::<PlayerRecord>(connection)?;

    Ok(Some((cur_player, records)))
}
