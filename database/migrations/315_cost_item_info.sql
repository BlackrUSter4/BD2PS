CREATE TABLE "CostItemInfo" (
"Index" INTEGER PRIMARY KEY AUTOINCREMENT,
"Uid" BIGINT NOT NULL,
"InvenIndex" BIGINT,
"Id" INTEGER,
"Type" INTEGER,
"Count" INTEGER,
"KeepFlag" INTEGER,
"TimeValue" BIGINT,
"PictorialbookInfoIndex" INTEGER, -- References PictorialBookInfo.InvenIndex
"ExpiryTime" BIGINT,
"SortId" INTEGER,
"UseCount" INTEGER
);
