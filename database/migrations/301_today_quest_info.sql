CREATE TABLE "TodayQuestInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "QuestInfoIndex" TEXT, -- References QuestInfo.InvenIndex
    "ClearQuestIds" INTEGER, -- Parallel array with clear_quest_ids, today_quest_id
    "TodayEndTime" BIGINT,
    "TodayQuestId" INTEGER NOT NULL -- Parallel array with clear_quest_ids, today_quest_id
);
