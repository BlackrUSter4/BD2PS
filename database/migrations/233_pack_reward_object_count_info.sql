CREATE TABLE "PackRewardObjectCountInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Type" TEXT,
    "PackId" INTEGER,
    "Count" INTEGER,
    "MaxCount" INTEGER
);