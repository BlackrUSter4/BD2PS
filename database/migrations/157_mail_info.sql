CREATE TABLE "MailInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "InvenIndex" BIGINT,
    "Type" INTEGER,
    "MailId" INTEGER,
    "SenderText" TEXT,
    "TitleText" TEXT,
    "MessageText" TEXT,
    "RewardExpireTime" BIGINT,
    "ItemType" INTEGER NOT NULL, -- Parallel array with item_type, item_id, item_count
    "ItemId" INTEGER NOT NULL, -- Parallel array with item_type, item_id, item_count
    "ItemCount" INTEGER NOT NULL, -- Parallel array with item_type, item_id, item_count
    "IsOpen" INTEGER,
    "OpenTime" BIGINT,
    "CreateTime" BIGINT,
    "HistoryDeleteTime" BIGINT,
    "IsCash" INTEGER
);
