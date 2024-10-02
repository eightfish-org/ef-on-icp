# Start the local replica (run this in a separate terminal)
#dfx start --clean

# Deploy the canister to the local replica
#dfx deploy

#CANISTER_ID=$(dfx canister id your_canister_name)
CANISTER_ID=bd3sg-teaaa-aaaaa-qaaba-cai
# Register a new protocol
dfx canister call $CANISTER_ID register_protocol '("protocol2")'

# Get registered protocols
dfx canister call $CANISTER_ID get_registered_protocols

# Record a heartbeat
dfx canister call $CANISTER_ID record_heartbeat '("protocol1", "2024-03-15T10:30:00Z")'

# Get the last heartbeat for a specific protocol
dfx canister call $CANISTER_ID get_last_heartbeat '("protocol1")'

# Get all heartbeats
dfx canister call $CANISTER_ID get_all_heartbeats

# Try to register a protocol with an unauthorized principal (this should fail)
dfx identity new unauthorized_user
dfx identity use unauthorized_user
dfx canister call $CANISTER_ID register_protocol '("unauthorized_protocol")'

# Switch back to the default identity
dfx identity use default
