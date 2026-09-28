CREATE TABLE "SupporterSlotInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "SlotIndex" INTEGER,
    "CostumeId" INTEGER,
    "Power" INTEGER,
    "BattleUseCount" INTEGER,
    "SupporterCharInfo" TEXT,
    "Date" BIGINT
);