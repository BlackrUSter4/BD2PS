CREATE TABLE "ItemStorageInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ItemInfoIndex" TEXT -- References ItemInfo.InvenIndex
);