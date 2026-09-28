CREATE TABLE "CafeteriaInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Level" INTEGER,
    "RewardReceiptTime" BIGINT,
    "SpawnTime" BIGINT,
    "OngoingManageId" INTEGER,
    "DailyConnectionCostumeId" INTEGER,
    "CanGetPhoneNumber" INTEGER,
    "DailyNpcRewardCurrencyCount" INTEGER
);