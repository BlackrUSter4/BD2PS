CREATE TABLE "AvatarShopWishListInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ShopId" INTEGER NOT NULL,
    UNIQUE("Uid", "ShopId")
);
