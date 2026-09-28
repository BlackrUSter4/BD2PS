CREATE TABLE "RoomClientInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "Guid" TEXT,
    "EnterTime" BIGINT
);