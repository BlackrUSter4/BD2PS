-- TotalWarInfo (305_total_war_info.sql, predates this project) had no way to track which
-- score-tier rewards (TotalWarRewardTable, real score-threshold tiers) have already been
-- claimed for this account.
ALTER TABLE "TotalWarInfo" ADD COLUMN "ClaimedRewardIds" TEXT;
