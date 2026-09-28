CREATE TABLE QuestLevelInfo (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "PackId" INTEGER NOT NULL DEFAULT 1,
    "QuestLevel" INTEGER,
    "ClearQuest" INTEGER,
    "QuestOpt" INTEGER,
    "IsLevelComplete" INTEGER,
    UNIQUE("Uid", "PackId")
);
