CREATE TABLE "UserQuest" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "QuestId" INTEGER NOT NULL,
    "PackId" INTEGER,
    "Status" INTEGER DEFAULT 0,          -- 0 = not accepted, 1 = in progress, 2 = completed, 3 = reward claimed
    "Progress" INTEGER DEFAULT 0,        -- quest progress (from questValue[0])
    "RewardClaimed" INTEGER DEFAULT 0,   -- 0 = false, 1 = true
    "LastUpdate" BIGINT                  -- timestamp for daily resets, analytics, etc.
);
