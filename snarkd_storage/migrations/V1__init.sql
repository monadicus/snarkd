CREATE TABLE blocks(
    id INTEGER PRIMARY KEY,
    canon_height INTEGER,
    hash BLOB UNIQUE NOT NULL,
    previous_block_id INTEGER, -- REFERENCES blocks(id) ON DELETE SET NULL -- can't do cyclic fk ref in sqlite
    previous_block_hash BLOB NOT NULL,
    -- previous_state_root BLOB NOT NULL,
    -- transactions_root BLOB NOT NULL,
    nonce INTEGER NOT NULL,
    network INTEGER NOT NULL,
    -- round BIGINT NOT NULL,
    height INTEGER NOT NULL,
    coinbase_target BIGINT NOT NULL,
    -- proof_target BIGINT NOT NULL,
    timestamp BIGINT NOT NULL,
    -- challenge BLOB NOT NULL,
    -- response BLOB NOT NULL,
    -- compute_key_public_key_signature BLOB NOT NULL,
    -- compute_key_public_randomness_signature BLOB NOT NULL,
    -- compute_key_secret_key_program BLOB NOT NULL
);

CREATE INDEX previous_block_id_lookup ON blocks(previous_block_id);
CREATE INDEX previous_block_hash_lookup ON blocks(previous_block_hash);
CREATE INDEX canon_height_lookup ON blocks(canon_height);

CREATE TABLE transactions(
    id INTEGER PRIMARY KEY,
    transaction_id BLOB UNIQUE NOT NULL,
    execute_edition INTEGER,
    transaction_type TEXT NOT NULL
);

CREATE TABLE transaction_blocks(
    id INTEGER PRIMARY KEY,
    transaction_id INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    block_id INTEGER NOT NULL REFERENCES blocks(id) ON DELETE CASCADE,
    block_order INTEGER NOT NULL
);
CREATE UNIQUE INDEX transaction_block_ordering ON transaction_blocks(block_id, block_order);
CREATE INDEX transaction_block_lookup ON transaction_blocks(transaction_id);

CREATE TABLE transitions(
    id INTEGER PRIMARY KEY,
    transaction_id INTEGER NOT NULL REFERENCES transactions(id) ON DELETE CASCADE,
    transaction_order INTEGER NOT NULL,
    transition_id BLOB UNIQUE NOT NULL,
    program_name BLOB NOT NULL,
    program_network BLOB NOT NULL,
    function_name BLOB NOT NULL,
    inputs BLOB NOT NULL,
    outputs BLOB NOT NULL,
    finalize BLOB,
    proof BLOB NOT NULL,
    tpk BLOB NOT NULL,
    tcm BLOB NOT NULL,
    fee BIGINT NOT NULL,
    deployment_id INTEGER REFERENCES deployments(id) ON DELETE CASCADE
);

CREATE TABLE deployments (
    id INTEGER PRIMARY KEY,
    edition INTEGER NOT NULL,
    program BLOB NOT NULL,
    verifying_key_id BLOB NOT NULL,
    verifying_key BLOB NOT NULL,
    certificate BLOB NOT NULL,
);

CREATE TABLE peers(
    id INTEGER PRIMARY KEY,
    address TEXT NOT NULL,
    last_peer_direction TEXT NOT NULL,
    block_height INTEGER NOT NULL,
    first_seen INTEGER,
    last_seen INTEGER,
    last_connected INTEGER,
    blocks_synced_to INTEGER NOT NULL,
    blocks_synced_from INTEGER NOT NULL,
    blocks_received_from INTEGER NOT NULL,
    blocks_sent_to INTEGER NOT NULL,
    connection_fail_count INTEGER NOT NULL,
    connection_success_count INTEGER NOT NULL
);
CREATE UNIQUE INDEX peer_address_lookup ON peers(address);
CREATE INDEX peer_last_seen_lookup ON peers(last_seen);
