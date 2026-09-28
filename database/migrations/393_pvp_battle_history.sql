CREATE TABLE "PvpBattleHistory" (
    "BattleIndex" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Season" INTEGER NOT NULL,
    "IsAttacker" BOOLEAN NOT NULL,
    "EnemyOwnerIndex" BIGINT,
    "EnemyUserId" TEXT,
    "EnemyVp" INTEGER,
    "EnemyRank" INTEGER,
    "ChangeVp" INTEGER,
    "ContinueWinVp" INTEGER,
    "TimeValue" BIGINT NOT NULL,
    "IsNoGame" BOOLEAN NOT NULL DEFAULT 0,
    "Seed" INTEGER,
    "DeckSnapshotJson" TEXT
);
CREATE INDEX idx_pvpbattlehistory_uid ON "PvpBattleHistory"("Uid");
