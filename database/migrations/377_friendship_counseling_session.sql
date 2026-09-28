CREATE TABLE "FriendshipCounselingSession" (
    "Uid" BIGINT NOT NULL,
    "CostumeId" INTEGER NOT NULL,
    "SessionId" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "CostumeId", "SessionId")
);
