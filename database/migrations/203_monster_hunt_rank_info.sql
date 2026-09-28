CREATE TABLE "MonsterHuntRankInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "UserRankInfoIndex" TEXT, -- References MonsterHuntRankUserInfo.InvenIndex
    "MyRankInfoIndex" BIGINT -- References MonsterHuntRankUserInfo.InvenIndex
);