-- FriendInfo (predates this project) had no way to distinguish a confirmed friendship
-- from a pending sent/received request, despite FriendAccept/Refuse/SendRemove/SendList/
-- ReceiveList all needing exactly that distinction.
-- Status: 0 = confirmed friend, 1 = pending (I sent to OwnerIndex), 2 = pending (OwnerIndex
-- sent to me).
ALTER TABLE "FriendInfo" ADD COLUMN "Status" INTEGER NOT NULL DEFAULT 0;
