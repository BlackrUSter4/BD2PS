CREATE TABLE "ItemInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT,
    "Id" INTEGER,
    "Type" INTEGER,
    "Count" INTEGER,
    "KeepFlag" INTEGER,
    "TimeValue" BIGINT,
    "PictorialbookInfoIndex" BIGINT, -- References PictorialBookInfo.InvenIndex
    "ExpiryTime" BIGINT,
    "SortId" INTEGER,
    "UseCount" INTEGER
);