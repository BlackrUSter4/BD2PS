CREATE TABLE "BattleGolemInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Level" INTEGER,
    "Gauge" REAL,
    "RemainTurn" INTEGER,
    "ReserveCostumeId" INTEGER,
    "Key" INTEGER NOT NULL,
    "Value" INTEGER NOT NULL
);