CREATE TABLE "StatueGroupInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER,
    "Season" INTEGER,
    "GroupRank" INTEGER,
    "GuildBaseInfoIndex" BIGINT, -- References GuildBaseInfo.InvenIndex
    "UserStatueInfoIndex" TEXT, -- References StatueInfo.InvenIndex
    "ErrorFlag" INTEGER
);