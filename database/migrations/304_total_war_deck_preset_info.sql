CREATE TABLE "TotalWarDeckPresetInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Slot" INTEGER,
    "PresetName" TEXT,
    "ResourceId" INTEGER,
    "ResourceColor" INTEGER,
    "DeckInfoIndex" TEXT -- References TotalWarDeckInfo.InvenIndex
);