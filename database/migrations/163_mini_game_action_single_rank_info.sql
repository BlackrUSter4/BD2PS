CREATE TABLE "MiniGameActionSingleRankInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "Rank" INTEGER,
    "Score" REAL,
    "CharId" INTEGER
);