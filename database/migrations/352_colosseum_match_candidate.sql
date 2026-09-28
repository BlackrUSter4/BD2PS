CREATE TABLE "ColosseumMatchCandidate" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EnemyOwnerIndex" BIGINT NOT NULL,
    "EnemyUserId" TEXT,
    "EnemyVp" INTEGER,
    "EnemyIsBot" INTEGER NOT NULL DEFAULT 0,
    "EnemyCharIds" TEXT,
    "CreateTime" BIGINT
);
