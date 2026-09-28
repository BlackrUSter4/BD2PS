CREATE TABLE "NoticeInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER,
    "NoticeType" TEXT,
    "Title" TEXT,
    "Thumbnail" TEXT,
    "StartTime" BIGINT,
    "EndTime" BIGINT,
    "WebUrl" TEXT,
    "NoticeContentsInfoIndex" TEXT, -- References NoticeContentsInfo.InvenIndex
    "PromotionBannerId" INTEGER,
    "IsPin" INTEGER,
    "Sort" INTEGER,
    "SubType" INTEGER,
    "IsInvert" INTEGER
);
