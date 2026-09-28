CREATE TABLE "GuildInitInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "JoinRecvInfoIndex" TEXT, -- References UserBaseInfo.InvenIndex
    "ActionInfoIndex" TEXT, -- References GuildActionInfo.InvenIndex
    "IsReward" INTEGER,
    "RaidPlayInfoIndex" BIGINT -- References GuildRaidPlayInfo.InvenIndex
);