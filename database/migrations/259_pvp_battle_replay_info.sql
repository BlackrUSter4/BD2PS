CREATE TABLE "PvpBattleReplayInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "BlueDeckFullInfoIndex" BIGINT, -- References PvpBattleUserDeckFullInfo.InvenIndex
    "RedDeckFullInfoIndex" BIGINT, -- References PvpBattleUserDeckFullInfo.InvenIndex
    "BattleRandomSeed" INTEGER NOT NULL
);