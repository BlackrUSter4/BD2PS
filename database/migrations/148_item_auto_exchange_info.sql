CREATE TABLE "ItemAutoExchangeInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OriginalItemType" INTEGER,
    "OriginalItemId" INTEGER,
    "OriginalItemCount" INTEGER,
    "ExchangeItemType" INTEGER,
    "ExchangeItemId" INTEGER,
    "ExchangeItemCount" INTEGER,
    "SortId" INTEGER
);