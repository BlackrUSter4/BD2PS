CREATE TABLE "GuildInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GuildBaseInfoIndex" BIGINT, -- References GuildBaseInfo.InvenIndex
    "JoinType" TEXT,
    "Message" TEXT,
    "UpdateDate" BIGINT,
    "Date" BIGINT,
    "MemberCount" INTEGER,
    "DeleteRemainingTime" BIGINT,
    "NoticeUpdateDate" BIGINT
);