CREATE TABLE IF NOT EXISTS blocks(
    id INTEGER PRIMARY KEY,
    canon_height INTEGER,
    canon_ledger_digest BLOB,
    hash BLOB UNIQUE NOT NULL,
    previous_block_id INTEGER, -- REFERENCES blocks(id) ON DELETE SET NULL -- can't do cyclic fk ref in sqlite
    previous_block_hash BLOB NOT NULL,
    merkle_root_hash BLOB NOT NULL,
    pedersen_merkle_root_hash BLOB NOT NULL,
    proof BLOB NOT NULL,
    time INTEGER NOT NULL,
    difficulty_target INTEGER NOT NULL,
    nonce INTEGER NOT NULL
);
CREATE INDEX previous_block_id_lookup ON blocks(previous_block_id);
CREATE INDEX previous_block_hash_lookup ON blocks(previous_block_hash);
CREATE INDEX canon_height_lookup ON blocks(canon_height);

CREATE TABLE IF NOT EXISTS transactions(
    id INTEGER PRIMARY KEY,
    transaction_id BLOB UNIQUE NOT NULL,
    network INTEGER NOT NULL,
    ledger_digest BLOB NOT NULL,
    old_serial_number1 BLOB NOT NULL,
    old_serial_number2 BLOB NOT NULL,
    new_commitment1 BLOB NOT NULL,
    new_commitment2 BLOB NOT NULL,
    program_commitment BLOB NOT NULL,
    local_data_root BLOB NOT NULL,
    value_balance INTEGER NOT NULL,
    signature1 BLOB NOT NULL,
    signature2 BLOB NOT NULL,
    new_record1 BLOB NOT NULL,
    new_record2 BLOB NOT NULL,
    proof BLOB NOT NULL,
    memo BLOB NOT NULL,
    inner_circuit_id BLOB NOT NULL
);

CREATE TABLE IF NOT EXISTS transaction_blocks(
    id INTEGER PRIMARY KEY,
    transaction_id INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    block_id INTEGER NOT NULL REFERENCES blocks(id) ON DELETE CASCADE,
    block_order INTEGER NOT NULL
);
CREATE UNIQUE INDEX transaction_block_ordering ON transaction_blocks(block_id, block_order);
CREATE INDEX transaction_block_lookup ON transaction_blocks(transaction_id);

CREATE INDEX blocks_time_lookup ON blocks(time);
CREATE TABLE IF NOT EXISTS peers(
    id INTEGER PRIMARY KEY,
    address TEXT NOT NULL,
    block_height INTEGER NOT NULL,
    first_seen INTEGER,
    last_seen INTEGER,
    last_connected INTEGER,
    blocks_synced_to INTEGER NOT NULL,
    blocks_synced_from INTEGER NOT NULL,
    blocks_received_from INTEGER NOT NULL,
    blocks_sent_to INTEGER NOT NULL,
    connection_attempt_count INTEGER NOT NULL,
    connection_success_count INTEGER NOT NULL,
    connection_transient_fail_count INTEGER NOT NULL
);
CREATE UNIQUE INDEX peer_address_lookup ON peers(address);
CREATE INDEX peer_last_seen_lookup ON peers(last_seen);
