CREATE TABLE "CashBonusInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GroupId" INTEGER NOT NULL,
    "ContentsGroupId" INTEGER NOT NULL,
    "BuyCount" INTEGER NOT NULL DEFAULT 0,
    "RewardedIds" TEXT,
    UNIQUE ("Uid", "GroupId", "ContentsGroupId")
);

CREATE TABLE "ChatSettingInfo" (
    "Uid" BIGINT PRIMARY KEY NOT NULL,
    "AutoTranslateFlag" INTEGER NOT NULL DEFAULT 0,
    "GlobalChatFlag" INTEGER NOT NULL DEFAULT 0,
    "VisualFlag" INTEGER NOT NULL DEFAULT 0
);

CREATE TABLE "ContentOpenInfo" (
    "Uid" BIGINT NOT NULL,
    "Type" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "Type")
);

CREATE TABLE "DailyStoryClearInfo" (
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "Id")
);

CREATE TABLE "FieldEventSpawnProgressInfo" (
    "Uid" BIGINT PRIMARY KEY NOT NULL,
    "StartTime" BIGINT,
    "EventScheduleId" INTEGER,
    "SpawnEventId" INTEGER,
    "GroupId" INTEGER,
    "CaughtInfo" TEXT
);

CREATE TABLE "FieldEventSpawnDailyCount" (
    "Uid" BIGINT NOT NULL,
    "Date" TEXT NOT NULL,
    "NormalCount" INTEGER NOT NULL DEFAULT 0,
    "SpecialCount" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("Uid", "Date")
);

CREATE TABLE "FireWorksRewardInfo" (
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER NOT NULL,
    "GroupId" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "EventScheduleId", "GroupId")
);

CREATE TABLE "MasterTitleInfo" (
    "Uid" BIGINT PRIMARY KEY NOT NULL,
    "Name" TEXT,
    "Month" INTEGER,
    "Day" INTEGER
);

CREATE TABLE "NpcQuizClearInfo" (
    "Uid" BIGINT NOT NULL,
    "EventUid" INTEGER NOT NULL,
    "GroupId" INTEGER NOT NULL,
    "Id" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "EventUid", "GroupId", "Id")
);

CREATE TABLE "SquareRewardInfo" (
    "Uid" BIGINT PRIMARY KEY NOT NULL,
    "LastClaimDate" TEXT
);

CREATE TABLE "TacticsBingoDeckInfo" (
    "Uid" BIGINT PRIMARY KEY NOT NULL,
    "DeckInfo" TEXT
);

CREATE TABLE "TacticsBingoClearInfo" (
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER NOT NULL,
    "GroupId" INTEGER NOT NULL,
    "ClearStage" TEXT,
    "EventFlag" INTEGER,
    PRIMARY KEY ("Uid", "EventScheduleId", "GroupId")
);

CREATE TABLE "AgeGateInfo" (
    "Uid" BIGINT PRIMARY KEY NOT NULL,
    "IsJp" INTEGER,
    "Year" INTEGER,
    "Month" INTEGER,
    "Day" INTEGER
);
