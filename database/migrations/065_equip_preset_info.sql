CREATE TABLE "EquipPresetInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PresetName" TEXT,
    "Slot" INTEGER,
    "PresetResourceId" INTEGER,
    "PresetResourceColor" INTEGER
);