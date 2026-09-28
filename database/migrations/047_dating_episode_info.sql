CREATE TABLE "DatingEpisodeInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GroupId" INTEGER,
    "DatingPoint" INTEGER,
    "LastClearId" INTEGER,
    "LastMessageGroupId" INTEGER,
    "LastMessageId" INTEGER,
    "LastMessageUpdateTime" BIGINT
);