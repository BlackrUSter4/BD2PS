-- EventHubSettingInfo.event_uid and EventRewardHistoryInfo.reward_id are non-optional
-- Rust model fields with no backing column (SELECT * would fail at runtime), and
-- EventHubSettingInfo had no FK back to its owning EventHubInfo row. Never exercised
-- before now (stub), safe to recreate.
DROP TABLE IF EXISTS "EventHubSettingInfo";
CREATE TABLE "EventHubSettingInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "HubInfoIndex" INTEGER NOT NULL,
    "Slot" INTEGER,
    "HubContentType" INTEGER,
    "EventUid" INTEGER NOT NULL
);

DROP TABLE IF EXISTS "EventRewardHistoryInfo";
CREATE TABLE "EventRewardHistoryInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventScheduleId" INTEGER,
    "EventGroupId" INTEGER,
    "RewardId" INTEGER NOT NULL
);
