CREATE TABLE "MatchingClientInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "UserInfo" TEXT,
    "IsRoomMaster" INTEGER,
    "EnterTime" BIGINT,
    "Guid" TEXT
);