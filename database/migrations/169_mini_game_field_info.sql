CREATE TABLE "MiniGameFieldInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InfoIndex" TEXT -- References MiniGameInfo.InvenIndex
);