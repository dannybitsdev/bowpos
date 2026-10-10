CREATE TABLE IF NOT EXISTS auth_access_token_revocations (
    token_id UUID PRIMARY KEY,
    user_id UUID NOT NULL REFERENCES users(id) ON DELETE CASCADE,
    expires_at TIMESTAMPTZ NOT NULL,
    revoked_at TIMESTAMPTZ NOT NULL DEFAULT NOW()
);

CREATE INDEX IF NOT EXISTS idx_auth_access_token_revocations_expires
    ON auth_access_token_revocations (expires_at);