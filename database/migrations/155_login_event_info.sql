CREATE TABLE "LoginEventInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "LoginEventId" INTEGER,
    "NextLoginTime" BIGINT
);