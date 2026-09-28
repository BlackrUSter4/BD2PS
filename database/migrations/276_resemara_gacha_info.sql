CREATE TABLE "ResemaraGachaInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "EventIndex" BIGINT,
    "ResemaraPreviewItemInfo" TEXT,
    "IsLock" INTEGER
);