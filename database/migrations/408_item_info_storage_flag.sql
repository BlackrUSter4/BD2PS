-- ItemInfo is the account's real live inventory table (heavily used elsewhere) — additive
-- ALTER only, never drop/recreate. The pre-existing ItemStorageInfo scaffold used a vague
-- JSON-array-of-InvenIndex design with no real per-item flag; a plain boolean column on the
-- item itself is simpler and matches how every other "which items are where" question in this
-- project is actually answered (a real column, not a side index table).
ALTER TABLE "ItemInfo" ADD COLUMN "IsStorage" INTEGER NOT NULL DEFAULT 0;
