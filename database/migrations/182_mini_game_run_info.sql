CREATE TABLE "MiniGameRunInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InfoIndex" TEXT -- References MiniGameInfo.InvenIndex
);