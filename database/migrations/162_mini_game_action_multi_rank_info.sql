CREATE TABLE "MiniGameActionMultiRankInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "UserInfoIndex" TEXT, -- References MiniGameActionUserInfo.InvenIndex
    "Rank" INTEGER,
    "Score" REAL
);