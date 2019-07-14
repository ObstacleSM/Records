table! {
    maps (maniaplanet_map_id) {
        maniaplanet_map_id -> Varchar,
        name -> Varchar,
        player_id -> Varchar,
    }
}

table! {
    players (login) {
        login -> Varchar,
        nickname -> Varchar,
    }
}

table! {
    records (map_id, player_id) {
        rank -> Unsigned<Integer>,
        time -> Integer,
        respawn_count -> Integer,
        try_count -> Integer,
        created_at -> Datetime,
        updated_at -> Datetime,
        player_id -> Varchar,
        map_id -> Varchar,
    }
}

joinable!(maps -> players (player_id));
joinable!(records -> maps (map_id));
joinable!(records -> players (player_id));

allow_tables_to_appear_in_same_query!(maps, players, records,);
