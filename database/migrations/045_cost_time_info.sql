CREATE TABLE "CostTimeInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "LastChargeTime" BIGINT,
    "CostItemInfoIndex" BIGINT -- References ItemInfo.InvenIndex
);