CREATE TABLE "GuildActionInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Type" TEXT,
    "Time" BIGINT,
    "GuildId" BIGINT,
    "GuildName" TEXT,
    "IsNotify" INTEGER,
    "Role" TEXT
);