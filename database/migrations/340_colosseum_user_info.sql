CREATE TABLE "ColosseumUserInfo" (
    "Uid" BIGINT PRIMARY KEY,
    "Vp" INTEGER NOT NULL DEFAULT 1000,
    "WinCount" INTEGER NOT NULL DEFAULT 0,
    "LoseCount" INTEGER NOT NULL DEFAULT 0,
    "Season" INTEGER NOT NULL DEFAULT 1,
    "ApBuyCount" INTEGER NOT NULL DEFAULT 0,
    "ApBuyResetTime" BIGINT,
    "BattleCountResetTime" BIGINT,
    "MatchRerollCount" INTEGER NOT NULL DEFAULT 0,
    "CurrentBattleEnemyIndex" BIGINT
);
