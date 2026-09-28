CREATE TABLE "CashMailInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "MailInfoIndex" TEXT, -- References MailInfo.InvenIndex
    "TotalCount" INTEGER,
    "MaxInvenIndex" BIGINT
);