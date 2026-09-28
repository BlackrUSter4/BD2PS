CREATE TABLE "ContentRankStatueInfo" (
    "Index" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER,
    "Season" INTEGER,
    "ErrorFlag" INTEGER,
    "StatueGroupInfoIndex" TEXT -- References StatueGroupInfo.InvenIndex
);