CREATE TABLE "PvpBattleHistoryDeckInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "UserDeckFullInfoIndex" BIGINT, -- References PvpBattleUserDeckFullInfo.InvenIndex
    "EnemyDeckFullInfoIndex" BIGINT -- References PvpBattleUserDeckFullInfo.InvenIndex
);