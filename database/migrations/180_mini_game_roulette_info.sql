CREATE TABLE "MiniGameRouletteInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "FreeApCount" INTEGER,
    "ResetTime" BIGINT,
    "IsRewardSpecialItem" INTEGER,
    "TryCount" INTEGER
);