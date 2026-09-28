CREATE TABLE "MonsterHuntDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Team" INTEGER,
    "DeckInfoIndex" TEXT, -- References DeckInfo.InvenIndex
    "BattlePower" INTEGER
);