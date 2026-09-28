CREATE TABLE "FriendshipCounselingDaily" (
    "Uid" BIGINT NOT NULL,
    "Day" TEXT NOT NULL,
    "TotalCount" INTEGER NOT NULL DEFAULT 0,
    "CompleteRewardGranted" INTEGER NOT NULL DEFAULT 0,
    PRIMARY KEY ("Uid", "Day")
);
