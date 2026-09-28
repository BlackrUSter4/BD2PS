CREATE TABLE "PackInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER,
    "ClearQuestCount" INTEGER,
    "IsPackComplete" INTEGER,
    "QuestLevel" INTEGER,
    "QuestOpt" INTEGER,
    "SubQuestCount" INTEGER,
    "ActiveTime" BIGINT,
    "IsBuy" INTEGER
);