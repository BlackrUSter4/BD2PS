CREATE TABLE "MailHistoryInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "MailInfoIndex" TEXT, -- References MailInfo.InvenIndex
    "TotalCount" INTEGER
);