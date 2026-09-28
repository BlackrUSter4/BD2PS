CREATE TABLE "UserPosition" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL UNIQUE,
    "PackId" INTEGER,
    "PackPosition" TEXT
);
