use crate::models::map::Map;
use crate::models::player::Player;
use crate::models::record::*;
use chrono::Utc;
use diesel::prelude::*;
use diesel::sql_query;

fn update_ranks(connection: &MysqlConnection, map_id: &str) -> QueryResult<usize> {
    let query = format!(
        r#"
UPDATE
	records,
	(
	select
		RANK() OVER (ORDER BY time) as rank,
		records.player_id,
		records.map_id
	from records
	where records.map_id = '{}'
	) as RankedRecords
SET
	records.rank = RankedRecords.rank
WHERE records.map_id = RankedRecords.map_id and records.player_id = RankedRecords.player_id;
"#,
        map_id
    );

    sql_query(query).execute(connection)
}

pub fn has_finished(
    connection: &MysqlConnection,
    time: i32,
    rs_count: i32,
    player_id: &str,
    map_id: &str,
) -> QueryResult<(bool, i32, i32)> {
    use crate::schema::{maps, players, records};

    let map: Option<Map> = maps::table.find(map_id).get_result(connection).optional()?;
    if let None = map {
        diesel::insert_into(maps::table)
            .values((
                maps::maniaplanet_map_id.eq(map_id),
                maps::name.eq("Unknwown map"),
                maps::player_id.eq("smokegun"),
            ))
            .execute(connection)?;
    }

    let player: Option<Player> = players::table
        .find(player_id)
        .get_result(connection)
        .optional()?;
    if let None = player {
        diesel::insert_into(players::table)
            .values((
                players::login.eq(player_id),
                players::nickname.eq(player_id),
            ))
            .execute(connection)?;
    }

    let has_previous: Result<Record, _> = records::table
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
                        records::time.eq(new),
                        records::respawn_count.eq(rs_count),
                        records::try_count.eq(records::try_count + 1),
                        records::updated_at.eq(Utc::now().naive_utc()),
                    ))
                    .execute(connection)?;
                update_ranks(connection, map_id)?;
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
                rank: 0,
            };

            let _inserted_record = diesel::insert_into(records::table)
                .values(new)
                .execute(connection)?;

            update_ranks(connection, map_id)?;

            Ok((true, time, time))
        }
    }
}

pub fn overview(
    connection: &MysqlConnection,
    player_id: &str,
    map_id: &str,
) -> QueryResult<Vec<RankedRecord>> {
    use crate::schema::records;

    let query = format!(
        r#"
    SELECT
        records.rank,
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
    let mut rows = 15;

    let has_record: Option<Record> = records::table
        .find((map_id, player_id))
        .get_result(connection)
        .optional()?;

    match has_record {
        Some(player_record) => {
            let player_idx = records
                .binary_search_by(|record| record.time.cmp(&player_record.time))
                .unwrap();

            if player_idx < rows {
                records.truncate(rows);
                Ok(records)
            } else {
                let mut res: Vec<RankedRecord> = Vec::with_capacity(rows);

                res.extend_from_slice(&records[0..3]);

                let row_minus_top3 = rows - 3;
                let mut start_idx = player_idx - row_minus_top3 / 2;
                let mut end_idx = player_idx + row_minus_top3 / 2;

                if end_idx >= records.len() {
                    start_idx -= end_idx - records.len();
                    end_idx = records.len();
                }

                res.extend_from_slice(&records[start_idx..end_idx]);
                Ok(res)
            }
        }

        _ => {
            // We keep one row at the end for the player
            rows -= 1;

            if records.len() > rows {
                let mut res: Vec<RankedRecord> = Vec::with_capacity(rows);
                res.extend_from_slice(&records[..rows-3]);
                res.extend_from_slice(&records[records.len()-3..]);
                Ok(res)
            } else {
                records.truncate(rows);
                Ok(records)
            }
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
) -> QueryResult<Option<(Map, Player, Vec<(Record, Player)>)>> {
    use crate::schema::{maps, players, records};

    let map: Option<Map> = maps::table.find(map_id).get_result(connection).optional()?;

    if map.is_none() {
        return Ok(None);
    }

    let cur_map = map.unwrap();

    let join = records::table.inner_join(players::table);

    let records = join
        .offset(offset)
        .limit(limit)
        .order_by(records::time)
        .filter(records::map_id.eq(map_id))
        .load(connection)?;

    let player = players::table
        .find(&cur_map.player_id)
        .get_result(connection)?;

    Ok(Some((cur_map, player, records)))
}

pub fn player_records(
    connection: &MysqlConnection,
    player_id: &str,
) -> QueryResult<Option<(Player, Vec<(Record, Map)>)>> {
    use crate::schema::{maps, players, records};

    let player: Option<Player> = players::table
        .find(player_id)
        .get_result(connection)
        .optional()?;

    if player.is_none() {
        return Ok(None);
    }

    let cur_player = player.unwrap();
    let records = records::table
        .inner_join(maps::table)
        .filter(records::player_id.eq(player_id))
        .order_by(records::updated_at.desc())
        .load::<(Record, Map)>(connection)?;

    Ok(Some((cur_player, records)))
}
