CREATE TABLE "PvpBattleUserInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "OwnerIndex" BIGINT,
    "Vp" INTEGER,
    "Rank" INTEGER,
    "WinCount" INTEGER,
    "LoseCount" INTEGER,
    "PortraitCostumeId" INTEGER,
    "DeckInfo" TEXT
);