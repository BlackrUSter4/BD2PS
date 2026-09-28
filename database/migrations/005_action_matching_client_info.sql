CREATE TABLE "ActionMatchingClientInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "UserInfo" TEXT,
    "IsRoomMaster" INTEGER,
    "EnterTime" BIGINT,
    "Guid" TEXT,
    "ActionChar" INTEGER
);