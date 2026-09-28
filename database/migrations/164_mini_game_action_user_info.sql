CREATE TABLE "MiniGameActionUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "UserId" TEXT,
    "CharId" INTEGER
);