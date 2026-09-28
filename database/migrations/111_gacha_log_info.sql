CREATE TABLE "GachaLogInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GachaGroupId" INTEGER,
    "GachaId" INTEGER,
    "BuyType" TEXT,
    "GachaCount" INTEGER,
    "GetPoint" INTEGER,
    "GachaType" TEXT,
    "PickupItemId" INTEGER,
    "GachaFixedInfoIndex" TEXT, -- References GachaFixedInfo.InvenIndex
    "RewardInfoBundle" TEXT,
    "LogTime" BIGINT
);