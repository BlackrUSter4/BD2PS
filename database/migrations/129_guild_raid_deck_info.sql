CREATE TABLE "GuildRaidDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "DeckInfoIndex" TEXT, -- References DeckInfo.InvenIndex
    "SupporterDeckInfoIndex" TEXT, -- References GuildRaidSupporterDeckInfo.InvenIndex
    "IsSupporterDeckUpdate" INTEGER
);