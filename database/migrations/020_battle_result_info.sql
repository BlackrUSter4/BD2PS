CREATE TABLE "BattleResultInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "BattleResult" INTEGER,
    "RedCharInfoIndex" TEXT, -- References BattleCharInfo.InvenIndex
    "BlueCharInfoIndex" TEXT, -- References BattleCharInfo.InvenIndex
    "GridItemIndex" TEXT, -- References BattleGridTypeItemInfo.InvenIndex
    "BattleStatisticsInfoIndex" BIGINT, -- References BattleStatisticsInfo.InvenIndex
    "GolemInfoIndex" BIGINT, -- References BattleGolemInfo.InvenIndex
    "RedDeckOutCharInfoIndex" TEXT, -- References BattleCharInfo.InvenIndex
    "BlueDeckOutCharInfoIndex" TEXT -- References BattleCharInfo.InvenIndex
);