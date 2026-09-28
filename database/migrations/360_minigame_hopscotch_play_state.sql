-- Tracks the in-flight (event_schedule_id, stage_id) between GameStart and GameEnd, since
-- MiniGameHopscotchGameEndRequest carries no stage/event reference of its own.
CREATE TABLE "MiniGameHopscotchPlayState" (
    "Uid" BIGINT PRIMARY KEY,
    "EventScheduleId" INTEGER NOT NULL DEFAULT 0,
    "StageId" INTEGER NOT NULL DEFAULT 0
);
