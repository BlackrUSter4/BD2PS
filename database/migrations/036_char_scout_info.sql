CREATE TABLE "CharScoutInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "AppearCharId" INTEGER NOT NULL, -- Parallel array with appear_char_id, scout_complete_char_id
    "UseResetCount" INTEGER,
    "NextAutoResetTime" BIGINT,
    "ScoutCompleteCharId" INTEGER NOT NULL -- Parallel array with appear_char_id, scout_complete_char_id
);