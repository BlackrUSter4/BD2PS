-- SupporterUsageInfo.BorrowType was TEXT but modeled in Rust as serde_json::Value, which
-- sqlx can neither bind nor decode for a plain column — would have failed on first real
-- use. Never exercised before now (stub), safe to recreate; retyped to INTEGER
-- (Define_SupporterBorrowType is a protobuf enum = i32).
DROP TABLE IF EXISTS "SupporterUsageInfo";
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
    "BorrowType" INTEGER,
    "RewardReceived" INTEGER,
    "UseDate" BIGINT
);
