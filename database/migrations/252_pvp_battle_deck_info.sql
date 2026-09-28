CREATE TABLE "PvpBattleDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "AttackDeckInfoIndex" TEXT, -- References PvpBattleUserDeckInfo.InvenIndex
    "AttackDeckItemInfoIndex" TEXT, -- References ContentsCharItemInfo.InvenIndex
    "DefenseDeckInfoIndex" TEXT, -- References PvpBattleUserDeckInfo.InvenIndex
    "DefenseDeckItemInfoIndex" TEXT -- References ContentsCharItemInfo.InvenIndex
);