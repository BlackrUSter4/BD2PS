CREATE TABLE "KnockbackInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "KnockbackDir" TEXT,
    "KnockbackValue" REAL,
    "KnockbackSpeed" REAL,
    "IsGuard" INTEGER
);