CREATE TABLE "MonsterInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "MonsterId" INTEGER,
    "BattleDeck" INTEGER,
    "RespawnTime" BIGINT,
    "LifeEndTime" BIGINT,
    "GroupId" INTEGER,
    "ActiveFlag" INTEGER
);