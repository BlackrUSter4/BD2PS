CREATE TABLE "PvpDeckInfo" (
    "Id" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "DeckType" INTEGER NOT NULL,
    "CharInvenIndex" BIGINT NOT NULL,
    "Position" INTEGER NOT NULL DEFAULT 0,
    "Sequence" INTEGER,
    "CostumeInvenIndex" BIGINT
);
CREATE INDEX idx_pvpdeckinfo_uid_type ON "PvpDeckInfo"("Uid", "DeckType");

CREATE TABLE "PvpDeckMeta" (
    "Uid" BIGINT NOT NULL,
    "DeckType" INTEGER NOT NULL,
    "ItemInfoJson" TEXT NOT NULL DEFAULT '[]',
    "BattlePower" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("Uid", "DeckType")
);
