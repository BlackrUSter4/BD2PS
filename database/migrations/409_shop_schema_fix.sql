-- ShopInfo had no ShopId column at all even though every request in this cluster is scoped
-- by shop_id (ShopTable has multiple distinct shops) — a single account-wide row couldn't
-- represent more than one shop's rotation. Recreated per-shop, plus a new ShopProductInfo
-- table for real per-product purchase-count tracking (replaces the old vague
-- "ProductInfoIndex" TEXT comma-list, same fix shape as this session's other rounds). Never
-- exercised before now (stub), safe to recreate.
DROP TABLE IF EXISTS "ShopInfo";
CREATE TABLE "ShopInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ShopId" INTEGER NOT NULL,
    "ShopRemainTime" INTEGER,
    "ShopRandSeed" INTEGER
);

CREATE TABLE "ShopProductInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ShopId" INTEGER NOT NULL,
    "ProductId" INTEGER NOT NULL,
    "BuyCount" INTEGER NOT NULL DEFAULT 0
);
