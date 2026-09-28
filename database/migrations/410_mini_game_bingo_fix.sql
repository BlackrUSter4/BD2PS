-- MiniGameBingoInfo was missing BingoBoard/OpenBingoBoardIndex columns entirely even though
-- the Rust model declared them as non-optional fields (SELECT * would have errored the first
-- time this table was ever queried) -- both are proto `repeated int32` fields, stored as JSON
-- text (no other query needs to filter within them). MiniGameBingoLineInfo.LineType was TEXT
-- modeled as serde_json::Value, which sqlx can neither bind nor decode for a plain column (same
-- recurring bug class fixed elsewhere this session) -- retyped to INTEGER
-- (Define_MiniGameBingoLineType is a protobuf enum = i32). Never exercised before now (stub),
-- safe to recreate both.
DROP TABLE IF EXISTS "MiniGameBingoInfo";
CREATE TABLE "MiniGameBingoInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "ClearCount" INTEGER,
    "BingoBoard" TEXT NOT NULL DEFAULT '[]',
    "OpenBingoBoardIndex" TEXT NOT NULL DEFAULT '[]'
);

DROP TABLE IF EXISTS "MiniGameBingoLineInfo";
CREATE TABLE "MiniGameBingoLineInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "LineType" INTEGER,
    "LineIndex" INTEGER
);
