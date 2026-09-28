CREATE TABLE "GuildRaidSupporterDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "SlotIndex" INTEGER,
    "Position" INTEGER,
    "Sequence" INTEGER
);