CREATE TABLE "RoomChatReportLog" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "ReporterUid" BIGINT NOT NULL,
    "TargetOwnerIndex" BIGINT NOT NULL,
    "ReportId" INTEGER,
    "Text" TEXT,
    "Reason" TEXT,
    "ChatTime" BIGINT,
    "CreatedAt" BIGINT NOT NULL
);
