-- PackRewardObjectCountInfo.Type was TEXT but modeled in Rust as serde_json::Value, which
-- sqlx can neither bind nor decode for a plain column (needs the Json<T> wrapper) — would
-- have failed on first real use. Never exercised before now (stub), so a clean recreate is
-- safe; retyped to INTEGER (Define_PackRewardObjectCountType is a protobuf enum = i32).
DROP TABLE IF EXISTS "PackRewardObjectCountInfo";
CREATE TABLE "PackRewardObjectCountInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Type" INTEGER,
    "PackId" INTEGER,
    "Count" INTEGER,
    "MaxCount" INTEGER
);
