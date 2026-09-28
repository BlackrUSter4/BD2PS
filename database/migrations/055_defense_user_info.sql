CREATE TABLE "DefenseUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "State" INTEGER,
    "EnemyCount" INTEGER,
    "Wave" INTEGER,
    "TotalEnemyKillCount" INTEGER,
    "NetworkState" INTEGER,
    "NetworkPing" INTEGER
);