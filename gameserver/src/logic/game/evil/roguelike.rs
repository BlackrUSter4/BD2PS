//! Shared roguelike-run helpers used by every `evil_castle_rogue_like_*` handler.
//!
//! Floor/room generation here is real and data-driven (RLFloorTable's real
//! roomGroupId/roomRatio weights pick a room group per slot, RLRoomTable
//! supplies the real room definition) but ROOMS_PER_FLOOR itself is a
//! documented placeholder — no captured table states an explicit room count
//! per floor, only per-slot weights for a not-otherwise-specified number of
//! slots.
use bd2::proto::proto_net::{EvilCastleRogueLikeFloorInfo as FloorInfoMsg, EvilCastleRogueLikeRoomInfo as RoomInfoMsg, EvilCastleRogueLikeStateInfo};
use data::exceldb;
use database::db::evil::{evil_castle_rogue_like_floor_info, evil_castle_rogue_like_room_info};
use sqlx::SqlitePool;

pub const ROOMS_PER_FLOOR: i32 = 8;

/// Very small xorshift-ish PRNG seeded from the current time — good enough
/// for room/shop rolls, no need to pull in a crate dependency for this.
pub fn rand_u32(seed: &mut u64) -> u32 {
    *seed ^= *seed << 13;
    *seed ^= *seed >> 7;
    *seed ^= *seed << 17;
    (*seed & 0xFFFF_FFFF) as u32
}

pub fn new_seed(salt: i64) -> u64 {
    let now = chrono::Utc::now().timestamp_nanos_opt().unwrap_or(0) as u64;
    now ^ (salt as u64).wrapping_mul(0x9E3779B97F4A7C15)
}

fn weighted_pick(seed: &mut u64, ids: &[i32], weights: &[i32]) -> Option<i32> {
    let total: i32 = weights.iter().sum();
    if total <= 0 || ids.is_empty() {
        return ids.first().copied();
    }
    let roll = (rand_u32(seed) % total as u32) as i32;
    let mut acc = 0;
    for (id, w) in ids.iter().zip(weights.iter()) {
        acc += w;
        if roll < acc {
            return Some(*id);
        }
    }
    ids.last().copied()
}

/// Generate and persist a floor's rooms for the given run level/floor
/// number. Real room definitions from RLRoomTable, real weighted selection
/// from RLFloorTable — only the fixed room count is a placeholder.
pub async fn generate_floor(pool: &SqlitePool, uid: i64, level: i32, floor: i32) -> Vec<RoomInfoMsg> {
    let db = exceldb::get();
    let floor_def = db
        .rlfloortable
        .by_group(level)
        .nth((floor - 1).max(0) as usize)
        .or_else(|| db.rlfloortable.by_group(level).last())
        .or_else(|| db.rlfloortable.all().first());

    let mut seed = new_seed(uid + floor as i64);
    let mut rooms = vec![];

    if let Some(fd) = floor_def {
        for number in 1..=ROOMS_PER_FLOOR {
            let group_id = weighted_pick(&mut seed, &fd.room_group_id, &fd.room_ratio).unwrap_or(0);
            let room = db
                .rlroomtable
                .by_group(group_id)
                .nth((rand_u32(&mut seed) as usize) % db.rlroomtable.by_group(group_id).count().max(1));
            let room_id = room.map(|r| r.id).unwrap_or(0);

            let _ = evil_castle_rogue_like_room_info::insert(
                pool,
                &database::models::game::evil::evil_castle_rogue_like_room_info::EvilCastleRogueLikeRoomInfo {
                    index: 0,
                    uid,
                    floor,
                    number: Some(number),
                    group_id: Some(group_id),
                    id: Some(room_id),
                    is_clear: Some(0),
                },
            )
            .await;

            rooms.push(RoomInfoMsg {
                number: Some(number),
                group_id: Some(group_id),
                id: Some(room_id),
                is_clear: Some(0),
            });
        }
    }
    let _ = evil_castle_rogue_like_floor_info::mark_visited(pool, uid, floor).await;

    rooms
}

/// Read back the currently-generated rooms for a floor (already persisted).
pub async fn get_floor(pool: &SqlitePool, uid: i64, floor: i32) -> FloorInfoMsg {
    let rooms = evil_castle_rogue_like_room_info::get_by_floor(pool, uid, floor)
        .await
        .unwrap_or_default();
    FloorInfoMsg {
        number: Some(floor),
        room_info: rooms
            .into_iter()
            .map(|r| RoomInfoMsg {
                number: r.number,
                group_id: r.group_id,
                id: r.id,
                is_clear: r.is_clear,
            })
            .collect(),
    }
}

pub async fn get_all_floors(pool: &SqlitePool, uid: i64, up_to: i32) -> Vec<FloorInfoMsg> {
    let mut out = vec![];
    for f in 1..=up_to.max(1) {
        out.push(get_floor(pool, uid, f).await);
    }
    out
}

pub fn default_state(floor: i32, room: i32) -> EvilCastleRogueLikeStateInfo {
    EvilCastleRogueLikeStateInfo {
        floor: Some(floor),
        room: Some(room),
        state: Some(0),
    }
}
