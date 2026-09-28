use bd2::prost::Message;
use bd2::proto::proto_net::{SaveUserPositionRequest, SaveUserPositionResponse};
use common::packet_code::PacketCodeType;
use crypto::network::GameResponse;
use sqlx::SqlitePool;
use tracing::info;

pub async fn handle(pool: &SqlitePool, uid: i64, req: SaveUserPositionRequest) -> GameResponse {
    info!(
        "Handling SaveUserPositionRequest: uid={}, pack_position={:?}",
        uid, req.pack_position
    );

    if let Some(pos) = &req.pack_position {
        if let Err(e) = sqlx::query(
            r#"
            INSERT INTO UserPosition (Uid, PackPosition)
            VALUES (?, ?)
            ON CONFLICT(Uid)
            DO UPDATE SET PackPosition = excluded.PackPosition
            "#,
        )
        .bind(uid)
        .bind(pos)
        .execute(pool)
        .await
        {
            eprintln!("Failed to update position: {:?}", e);
        }
    }

    let response = SaveUserPositionResponse {}; // no fields
    let resp_bytes = response.encode_to_vec();

    let (route, code) = PacketCodeType::SaveUserPosition.info();
    GameResponse::success(route, &resp_bytes, code)
}
