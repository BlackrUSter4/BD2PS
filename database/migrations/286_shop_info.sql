CREATE TABLE "ShopInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ShopRemainTime" INTEGER,
    "ShopRandSeed" INTEGER,
    "ProductInfoIndex" TEXT -- References ProductInfo.InvenIndex
);