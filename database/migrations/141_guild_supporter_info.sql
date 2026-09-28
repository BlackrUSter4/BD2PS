CREATE TABLE "GuildSupporterInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "SlotIndex" INTEGER,
    "UserId" TEXT,
    "BattleUseCount" INTEGER,
    "SupporterCharInfoProto" TEXT
);