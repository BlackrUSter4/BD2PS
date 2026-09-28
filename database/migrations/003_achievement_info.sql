CREATE TABLE "AchievementInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GroupId" INTEGER,
    "Value" BIGINT,
    "MaxClearId" INTEGER,
    "ContentsGroup" INTEGER
);