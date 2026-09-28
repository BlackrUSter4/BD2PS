CREATE TABLE "ItemAutoUpgradeInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT,
    "ItemType" INTEGER,
    "ItemId" INTEGER,
    "BeforeLevel" INTEGER,
    "AfterLevel" INTEGER,
    "SortId" INTEGER
);