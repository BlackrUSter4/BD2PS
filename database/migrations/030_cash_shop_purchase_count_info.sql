CREATE TABLE "CashShopPurchaseCountInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PurchaseCountInfoIndex" TEXT -- References PurchaseCountInfo.InvenIndex
);