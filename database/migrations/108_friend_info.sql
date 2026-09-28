CREATE TABLE "FriendInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "PortraitCostumeId" INTEGER,
    "PortraitCostumeDesignId" INTEGER,
    "UserId" TEXT,
    "TitleId" INTEGER,
    "Date" BIGINT,
    "GuildBaseInfoIndex" BIGINT, -- References GuildBaseInfo.InvenIndex
    "LastLoginDate" BIGINT,
    "SupporterInfoIndex" TEXT -- References CostumeBaseInfo.InvenIndex
);