CREATE TABLE "PvpCurrentMatch" (
    "Uid" BIGINT PRIMARY KEY,
    "EnemyOwnerIndex" BIGINT,
    "EnemyUserId" TEXT,
    "EnemyVp" INTEGER,
    "EnemyIsBot" BOOLEAN NOT NULL DEFAULT 0,
    "EnemyCharIds" TEXT,
    "BattleRandomSeed" INTEGER,
    "CreatedAt" BIGINT
);
