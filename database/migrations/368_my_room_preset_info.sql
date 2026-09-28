CREATE TABLE "MyRoomPresetInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PresetType" INTEGER NOT NULL,
    "Slot" INTEGER NOT NULL,
    "Name" TEXT,
    "SourceOwnerIndex" BIGINT,
    "ItemInfoJson" TEXT,
    "RoomInfoJson" TEXT,
    UNIQUE("Uid", "PresetType", "Slot")
);
