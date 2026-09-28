CREATE TABLE "MyRoomUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "PortraitCostumeId" INTEGER,
    "PrimaryMyRoomId" INTEGER,
    "ItemInfoIndex" TEXT, -- References ItemInfo.InvenIndex
    "CostumeInfoIndex" TEXT, -- References CostumeInfo.InvenIndex
    "TrophyInfoIndex" TEXT, -- References MyRoomTrophyInfo.InvenIndex
    "MyRoomIndex" TEXT, -- References MyRoomInfo.InvenIndex
    "MyRoomLikeCount" INTEGER,
    "MyRoomLikeDate" BIGINT,
    "PortraitCostumeDesignId" INTEGER
);