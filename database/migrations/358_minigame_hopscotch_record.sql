CREATE TABLE "MiniGameHopscotchRecord" (
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER NOT NULL,
    "StageId" INTEGER NOT NULL,
    "IsClear" BOOLEAN NOT NULL DEFAULT 0,
    "CapturedArea" INTEGER NOT NULL DEFAULT 0,
    "ClearTime" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("Uid", "EventScheduleId", "StageId")
);
