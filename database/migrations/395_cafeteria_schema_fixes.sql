-- Fixes latent scaffolding bugs in CafeteriaInfo (024_cafeteria_info.sql): the model
-- referenced DailyRegularCostumeId/RewardedDailyRegularCostumeId columns that were never
-- created (repeated int32 fields in the proto, stored here as JSON-array TEXT), and there
-- was no column at all to track the one-time "introduction story" reward claim.
ALTER TABLE "CafeteriaInfo" ADD COLUMN "DailyRegularCostumeIds" TEXT;
ALTER TABLE "CafeteriaInfo" ADD COLUMN "RewardedDailyRegularCostumeIds" TEXT;
ALTER TABLE "CafeteriaInfo" ADD COLUMN "IntroStoryRewardClaimed" INTEGER NOT NULL DEFAULT 0;
