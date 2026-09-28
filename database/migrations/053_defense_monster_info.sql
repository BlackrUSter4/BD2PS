CREATE TABLE "DefenseMonsterInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "MonsterId" INTEGER,
    "MonsterIndex" INTEGER,
    "SpawnTicks" BIGINT,
    "DespwanTicks" BIGINT,
    "Health" INTEGER
);