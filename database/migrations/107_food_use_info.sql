CREATE TABLE "FoodUseInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT,
    "ItemInfoIndex" TEXT -- References ItemInfo.InvenIndex
);