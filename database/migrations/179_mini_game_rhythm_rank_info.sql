CREATE TABLE "MiniGameRhythmRankInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Rank" INTEGER,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "Point" BIGINT
);