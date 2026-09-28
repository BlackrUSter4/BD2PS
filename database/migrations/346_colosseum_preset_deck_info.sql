CREATE TABLE "ColosseumPresetDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "PresetInfoIndex" INTEGER NOT NULL,
    "CharInvenIndex" BIGINT NOT NULL,
    "Position" INTEGER,
    "Sequence" INTEGER,
    "CostumeInvenIndex" BIGINT,
    "Team" INTEGER
);
