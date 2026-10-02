-- FieldObjectRespawnInfo keyed only on (Uid, FieldObjectGroupId) — but
-- FieldObjectGroupId is a per-pack-local id (see FieldActionObjectGroupTable's
-- PackId doc comment, commit 32a2e19), so two different packs' same-numbered
-- group (e.g. both having a group 601) shared one respawn-timer row per
-- account. Existing rows predate pack-tracking entirely, so there's no way to
-- know which pack they were actually saved under — default to pack 1 (the
-- fallback get_current_pack_id itself already uses) rather than guess further.
ALTER TABLE "FieldObjectRespawnInfo" ADD COLUMN "PackId" INTEGER NOT NULL DEFAULT 1;
