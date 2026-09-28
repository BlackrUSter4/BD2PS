CREATE TABLE "SeasonRewardInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PackId" INTEGER,
    "Season" INTEGER,
    "IsRewardReceived" INTEGER
);