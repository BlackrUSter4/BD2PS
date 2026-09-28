CREATE TABLE "CashShopInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GroupId" INTEGER,
    "ShopId" INTEGER,
    "StartTime" BIGINT,
    "EndTime" BIGINT
);