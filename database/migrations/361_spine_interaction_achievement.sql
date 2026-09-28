-- One row per unlocked motion. Normalized rather than storing the repeated motion_id list as
-- JSON in a single column, matching this project's established convention (e.g. IbDeck) of one
-- row per repeated-field element.
CREATE TABLE "SpineInteractionAchievement" (
    "Uid" BIGINT NOT NULL,
    "InteractionGroupId" INTEGER NOT NULL,
    "GroupId" INTEGER NOT NULL,
    "PointId" INTEGER NOT NULL,
    "MotionId" INTEGER NOT NULL,
    PRIMARY KEY ("Uid", "InteractionGroupId", "GroupId", "PointId", "MotionId")
);
