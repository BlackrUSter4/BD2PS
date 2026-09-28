-- Tracks which (interaction_group_id, group_id, id) reward triples this account has already
-- claimed, so SpineInteractionReward can't be replayed for repeat grants.
CREATE TABLE "SpineInteractionRewardClaim" (
    "Uid" BIGINT NOT NULL,
    "InteractionGroupId" INTEGER NOT NULL,
    "GroupId" INTEGER NOT NULL,
    "Id" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "InteractionGroupId", "GroupId", "Id")
);
