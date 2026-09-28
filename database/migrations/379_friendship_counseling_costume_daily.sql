CREATE TABLE "FriendshipCounselingCostumeDaily" (
    "Uid" BIGINT NOT NULL,
    "CostumeId" INTEGER NOT NULL,
    "Day" TEXT NOT NULL,
    PRIMARY KEY ("Uid", "CostumeId", "Day")
);
