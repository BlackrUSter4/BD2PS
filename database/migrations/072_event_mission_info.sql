CREATE TABLE "EventMissionInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventId" BIGINT,
    "GroupId" INTEGER,
    "Id" INTEGER,
    "Value" INTEGER,
    "IsComplete" INTEGER
);