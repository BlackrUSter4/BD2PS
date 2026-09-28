CREATE TABLE "BattleStatisticsInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "AttackDamageInfoIndex" TEXT, -- References BattleDamageInfo.InvenIndex
    "ReceivedDamageInfoIndex" TEXT -- References BattleDamageInfo.InvenIndex
);