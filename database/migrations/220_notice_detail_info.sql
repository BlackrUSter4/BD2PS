CREATE TABLE "NoticeDetailInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "NoticeInfoIndex" BIGINT -- References NoticeInfo.InvenIndex
);