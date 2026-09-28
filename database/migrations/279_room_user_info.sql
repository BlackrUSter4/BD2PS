CREATE TABLE "RoomUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "Ip" TEXT,
    "Port" TEXT,
    "NatType" INTEGER
);