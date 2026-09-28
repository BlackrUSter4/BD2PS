CREATE TABLE "EvilCastleRankingInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "UserRankingInfoIndex" TEXT, -- References EvilCastleRankUserInfo.InvenIndex
    "MyRankingInfoIndex" BIGINT -- References EvilCastleRankUserInfo.InvenIndex
);