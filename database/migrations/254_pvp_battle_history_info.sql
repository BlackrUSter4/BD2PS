CREATE TABLE "PvpBattleHistoryInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "BattleIndex" BIGINT,
    "BattleResult" INTEGER,
    "EnemyOwnerIndex" BIGINT,
    "EnemyUserId" TEXT,
    "EnemyVp" INTEGER,
    "EnemyRank" INTEGER,
    "ChangeVp" INTEGER,
    "ContinueWinVp" INTEGER,
    "TimeValue" BIGINT,
    "IsNoGame" INTEGER
);