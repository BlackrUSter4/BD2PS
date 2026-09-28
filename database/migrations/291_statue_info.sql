CREATE TABLE "StatueInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER,
    "Season" INTEGER,
    "Rank" INTEGER,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "PortraitCostumeId" INTEGER,
    "PortraitCostumeDesignId" INTEGER
);