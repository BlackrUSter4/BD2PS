CREATE TABLE "PvpBattleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Vp" INTEGER,
    "Rank" INTEGER,
    "WinCount" INTEGER,
    "LoseCount" INTEGER,
    "PortraitCostumeId" INTEGER,
    "DeckInfo" TEXT,
    "RewardFlag" INTEGER
);