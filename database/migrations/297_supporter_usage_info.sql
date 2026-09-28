CREATE TABLE "SupporterUsageInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" BIGINT,
    "BorrowerOwnerIndex" BIGINT,
    "BorrowerUserId" TEXT,
    "BorrowerPortraitCostumeId" INTEGER,
    "BorrowerPortraitDesignId" INTEGER,
    "BorrowerTitleId" INTEGER,
    "SupporterOwnerIndex" BIGINT,
    "SupporterSlotIndex" INTEGER,
    "SupporterCostumeId" INTEGER,
    "SupporterDesignId" INTEGER,
    "BorrowType" TEXT,
    "RewardReceived" INTEGER,
    "UseDate" BIGINT
);