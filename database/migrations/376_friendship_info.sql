CREATE TABLE "FriendshipInfo" (
    "Uid" BIGINT NOT NULL,
    "CostumeId" INTEGER NOT NULL,
    "Level" INTEGER NOT NULL DEFAULT 1,
    "Exp" INTEGER NOT NULL DEFAULT 0,
    "LastCounselingDate" BIGINT,
    PRIMARY KEY ("Uid", "CostumeId")
);
