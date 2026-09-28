CREATE TABLE "SubscribeAttendanceInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "TicketId" INTEGER,
    "ReservedDate" BIGINT,
    "ExpiryDate" BIGINT
);