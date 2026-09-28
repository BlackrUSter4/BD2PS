CREATE TABLE "PackInGameInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "CharInfoIndex" TEXT, -- References CharInfo.InvenIndex
    "QuestInfoIndex" TEXT, -- References QuestInfo.InvenIndex
    "ClearQuestIds" INTEGER NOT NULL, -- Parallel array with clear_quest_ids, research_object_id, statue_reward_obtain_id
    "Position" TEXT,
    "TalentNpcInfoIndex" TEXT, -- References TalentNpcInfo.InvenIndex
    "MonsterInfoIndex" TEXT, -- References MonsterInfo.InvenIndex
    "TalentObjectInfoIndex" TEXT, -- References TalentObjectInfo.InvenIndex
    "FieldBuffInfoIndex" TEXT, -- References FieldBuffInfo.InvenIndex
    "ReputationInfoIndex" TEXT, -- References ReputationInfo.InvenIndex
    "MapActiveInfoIndex" TEXT, -- References MapActiveInfo.InvenIndex
    "ResearchObjectId" INTEGER NOT NULL, -- Parallel array with clear_quest_ids, research_object_id, statue_reward_obtain_id
    "HuntingGroundInfoIndex" BIGINT, -- References HuntingGroundInfo.InvenIndex
    "TalentSkillInfoIndex" TEXT, -- References TalentSkillInfo.InvenIndex
    "ContentRankStatueInfoIndex" TEXT, -- References ContentRankStatueInfo.InvenIndex
    "StatueRewardObtainId" INTEGER NOT NULL, -- Parallel array with clear_quest_ids, research_object_id, statue_reward_obtain_id
    "RewardInfoBundleIndex" BIGINT -- References RewardInfoBundle.InvenIndex
);