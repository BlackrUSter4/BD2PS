CREATE TABLE "GuildJoinSendInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "JoinSendInfoIndex" TEXT, -- References GuildInfo.InvenIndex
    "ActionInfoIndex" TEXT -- References GuildActionInfo.InvenIndex
);