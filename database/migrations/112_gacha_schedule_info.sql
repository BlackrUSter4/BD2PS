CREATE TABLE "GachaScheduleInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "GroupId" INTEGER,
    "StartTime" BIGINT,
    "EndTime" BIGINT,
    "IsGachaFreeCountBonus" INTEGER,
    "IsGachaCashCountBonus" INTEGER
);