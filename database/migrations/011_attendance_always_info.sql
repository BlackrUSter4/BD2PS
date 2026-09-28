CREATE TABLE "AttendanceAlwaysInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "AttendanceGroupId" INTEGER,
    "AttendanceCount" INTEGER
);