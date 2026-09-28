CREATE TABLE "RoomChatReportUser" (
    "Uid" BIGINT NOT NULL PRIMARY KEY,
    "ReportCount" INTEGER NOT NULL DEFAULT 0,
    "ReportCountResetTime" BIGINT
);
