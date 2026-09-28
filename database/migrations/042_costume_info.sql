CREATE TABLE "CostumeInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT,
    "Id" INTEGER,
    "Level" INTEGER,
    "UseChar" BIGINT,
    "PictorialbookInfoIndex" TEXT, -- References PictorialBookInfo.InvenIndex
    "SortId" INTEGER,
    "UseMyRoomCount" INTEGER,
    "PotentialId" TEXT,
    "DesignId" INTEGER
);
