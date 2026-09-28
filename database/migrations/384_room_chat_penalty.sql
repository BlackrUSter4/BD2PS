CREATE TABLE "RoomChatPenalty" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "StartTime" BIGINT NOT NULL,
    "EndTime" BIGINT NOT NULL,
    "ReportId" INTEGER
);
