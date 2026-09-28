CREATE TABLE "PopularCostumeInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InfoIndex" TEXT -- References PopularCostumeCountInfo.InvenIndex
);