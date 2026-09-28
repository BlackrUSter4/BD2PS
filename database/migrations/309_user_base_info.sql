CREATE TABLE "UserBaseInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "PortraitCostumeId" INTEGER,
    "PortraitCostumeDesignId" INTEGER,
    "UserId" TEXT,
    "TitleId" INTEGER,
    "Date" BIGINT
);