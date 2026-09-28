CREATE TABLE "BattleGridTypeItemInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "TeamType" INTEGER,
    "GridIndex" INTEGER,
    "BuffId" INTEGER,
    "IsInfinite" INTEGER,
    "BuffTurn" INTEGER
);