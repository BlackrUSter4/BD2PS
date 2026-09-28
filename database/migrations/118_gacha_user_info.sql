CREATE TABLE "GachaUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GroupId" INTEGER,
    "Point" INTEGER,
    "TotalBuyCount" INTEGER,
    "OneFreePickCount" INTEGER,
    "OneCashPickCount" INTEGER,
    "TenFreePickCount" INTEGER,
    "TenCashPickCount" INTEGER,
    "ExchangeItemCount" INTEGER,
    "ExchangeMileageCount" INTEGER
);