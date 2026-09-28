CREATE TABLE "GuildMemberInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" BIGINT,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "TitleId" INTEGER,
    "PortraitCostumeId" INTEGER,
    "PortraitCostumeDesignId" INTEGER,
    "Role" TEXT,
    "Point" INTEGER,
    "SupporterInfoIndex" TEXT, -- References CostumeBaseInfo.InvenIndex
    "LastLoginDate" BIGINT,
    "UpdateDate" BIGINT,
    "Date" BIGINT
);