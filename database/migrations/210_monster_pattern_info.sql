CREATE TABLE "MonsterPatternInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "TargetOwnerIndex" BIGINT,
    "PatternId" INTEGER,
    "DistanceToAttack" REAL
);