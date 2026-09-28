CREATE TABLE "CharVoteRewardClaim" (
    "Uid" BIGINT NOT NULL,
    "EventId" INTEGER NOT NULL,
    "RewardId" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "EventId", "RewardId")
);
