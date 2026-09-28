CREATE TABLE "SupporterBattleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "FriendSupporterInfoIndex" TEXT, -- References SupporterDeckInfo.InvenIndex
    "RecommendedSupporterInfoIndex" TEXT -- References SupporterDeckInfo.InvenIndex
);