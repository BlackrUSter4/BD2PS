CREATE TABLE "EvilCastleRogueLikeInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "StateInfoIndex" BIGINT, -- References EvilCastleRogueLikeStateInfo.InvenIndex
    "Level" INTEGER,
    "FloorInfoIndex" TEXT, -- References EvilCastleRogueLikeFloorInfo.InvenIndex
    "DeckInfoIndex" TEXT, -- References DeckInfo.InvenIndex
    "CharInfoIndex" TEXT, -- References CharInfo.InvenIndex
    "CostumeInfoIndex" TEXT, -- References CostumeInfo.InvenIndex
    "RelicInfoIndex" TEXT, -- References RelicInfo.InvenIndex
    "ChoiceInfoIndex" BIGINT, -- References EvilCastleRogueLikeChoiceInfo.InvenIndex
    "ReRoll" INTEGER,
    "GroupId" INTEGER,
    "Id" INTEGER,
    "GrowthInfoIndex" TEXT, -- References EvilCastleRogueLikeGrowthInfo.InvenIndex
    "EventInfoIndex" BIGINT, -- References EvilCastleRogueLikeEventInfo.InvenIndex
    "ShopInfoIndex" BIGINT, -- References EvilCastleRogueLikeShopInfo.InvenIndex
    "BattleLevel" INTEGER,
    "MaxTryLevel" INTEGER,
    "Obsidian" INTEGER,
    "RogueLikeGold" INTEGER,
    "Season" INTEGER,
    "RegularSeason" INTEGER,
    "SeasonReward" INTEGER,
    "MaxRewardLevel" INTEGER,
    "HighestCrystalDamage" BIGINT
);