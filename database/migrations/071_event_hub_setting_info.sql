CREATE TABLE "EventHubSettingInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Slot" INTEGER,
    "HubContentType" INTEGER
);