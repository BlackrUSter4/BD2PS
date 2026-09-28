CREATE TABLE "PvpSeasonRewardClaim" (
    "Uid" BIGINT NOT NULL,
    "Season" INTEGER NOT NULL,
    "ClaimedAt" BIGINT NOT NULL,
    PRIMARY KEY ("Uid", "Season")
);
