CREATE TABLE "PackPreviewInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "QuestInfoIndex" TEXT, -- References QuestInfo.InvenIndex
    "QuestTitleInfoIndex" TEXT, -- References QuestTitleInfo.InvenIndex
    "IsPackEventReward" INTEGER
);