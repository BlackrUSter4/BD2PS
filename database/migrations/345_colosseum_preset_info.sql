CREATE TABLE "ColosseumPresetInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Slot" INTEGER NOT NULL,
    "PresetName" TEXT,
    "PresetResourceId" INTEGER,
    "PresetResourceColor" INTEGER
);
