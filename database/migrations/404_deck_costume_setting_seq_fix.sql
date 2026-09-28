-- DeckCostumeSettingInfo.costume_inven_index_seq is a non-optional Rust model field with no
-- backing column (SELECT * would have errored the first time this table was ever queried).
-- The proto field is `repeated int64`, so one row per (Uid, CharInvenIndex, seq value) is used,
-- same one-row-per-list-element pattern as EventHubSettingInfo/EventRewardHistoryInfo. Never
-- exercised before now (stub), safe to recreate.
DROP TABLE IF EXISTS "DeckCostumeSettingInfo";
CREATE TABLE "DeckCostumeSettingInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CharInvenIndex" BIGINT,
    "CostumeInvenIndexSeq" BIGINT NOT NULL
);
