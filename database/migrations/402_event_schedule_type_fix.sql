-- EventScheduleInfo.EventType was TEXT but modeled in Rust as serde_json::Value, which
-- sqlx can neither bind nor decode for a plain column — would have failed on first real
-- use. Never exercised before now (stub), safe to recreate; retyped to INTEGER
-- (Define_EventType is a protobuf enum = i32).
DROP TABLE IF EXISTS "EventScheduleInfo";
CREATE TABLE "EventScheduleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER,
    "EventType" INTEGER,
    "EventId" INTEGER,
    "EventSubId" INTEGER,
    "StartDate" BIGINT,
    "EndDate" BIGINT,
    "IsActive" INTEGER
);
