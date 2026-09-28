CREATE TABLE "EvilCastleRogueLikeRankInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "UserRankInfoIndex" TEXT, -- References EvilCastleRogueLikeRankUserInfo.InvenIndex
    "MyRankInfoIndex" BIGINT -- References EvilCastleRogueLikeRankUserInfo.InvenIndex
);