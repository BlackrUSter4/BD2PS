CREATE TABLE "CashProductInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GroupId" INTEGER,
    "Id" INTEGER,
    "SaleGroup" INTEGER,
    "StartTime" BIGINT,
    "EndTime" BIGINT,
    "EndDelayMinutes" INTEGER,
    "EventIndex" BIGINT
);