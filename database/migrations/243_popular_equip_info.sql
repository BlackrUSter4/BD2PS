CREATE TABLE "PopularEquipInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "UniqueCharId" INTEGER,
    "SlotType" INTEGER,
    "UniqueEquipId" INTEGER,
    "UseCount" BIGINT
);