CREATE TABLE "FishingCollectionInfo" (
    "Uid" BIGINT NOT NULL,
    "FishId" INTEGER NOT NULL,
    "MaxSize" INTEGER NOT NULL,
    "MinSize" INTEGER NOT NULL,
    "CreateTime" BIGINT,
    PRIMARY KEY ("Uid", "FishId")
);
