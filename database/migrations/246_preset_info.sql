CREATE TABLE "PresetInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PresetName" TEXT,
    "PresetResourceId" INTEGER,
    "PresetResourceColor" INTEGER,
    "Slot" INTEGER,
    "DeckInfoIndex" TEXT -- References PresetDeckInfo.InvenIndex
);