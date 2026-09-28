CREATE TABLE "ColosseumBattleHistory" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Season" INTEGER NOT NULL DEFAULT 1,
    "IsAttacker" INTEGER NOT NULL,
    "EnemyOwnerIndex" BIGINT,
    "EnemyUserId" TEXT,
    "EnemyVp" INTEGER,
    "EnemyRank" INTEGER,
    "ChangeVp" INTEGER,
    "TimeValue" BIGINT,
    "EnemyTopPercent" REAL,
    "IsNoGame" INTEGER NOT NULL DEFAULT 0
);
