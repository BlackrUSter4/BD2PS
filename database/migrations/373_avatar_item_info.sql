CREATE TABLE "AvatarItemInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "ItemCategory" INTEGER NOT NULL,
    "ItemId" INTEGER NOT NULL,
    UNIQUE("Uid", "ItemCategory", "ItemId")
);
