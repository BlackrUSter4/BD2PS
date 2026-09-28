-- InvenIndex is the SQLite rowid (globally unique, not per-account) — the client only ever uses
-- it as an opaque handle scoped by its own account, and this avoids needing to compute a
-- per-account next-index ourselves. RecordData is the raw client-uploaded blob; the real client
-- uploads this to S3 and gets back a URL (record_data_s3_url) — this server has no S3, so it
-- stores the bytes itself and serves them back from its own address instead (see
-- spine_interaction_record_data route).
CREATE TABLE "SpineInteractionRecord" (
    "InvenIndex" INTEGER PRIMARY KEY AUTOINCREMENT,
    "Uid" BIGINT NOT NULL,
    "Id" INTEGER NOT NULL DEFAULT 0,
    "Name" TEXT NOT NULL DEFAULT '',
    "RecordData" BLOB
);
