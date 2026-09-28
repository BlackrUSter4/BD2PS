CREATE TABLE "BattleSession" (
    "Uid" BIGINT PRIMARY KEY,
    "BattleIndex" INTEGER,
    "GroupId" INTEGER,
    "MonsterId" INTEGER,
    "PackId" INTEGER,
    "BattleDeck" INTEGER,
    "BattleMode" INTEGER,
    "MonsterHuntId" INTEGER,
    "StageMagicGroupId" INTEGER,
    "StageMagicId" INTEGER,
    "StageMagicLevel" INTEGER,
    "RandomSeed" INTEGER,
    "CreatedAt" BIGINT NOT NULL
);
