CREATE TABLE "EquipInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT,
    "UseChar" BIGINT,
    "KeepFlag" INTEGER,
    "LockFlag" INTEGER,
    "BaseInfo" TEXT
);