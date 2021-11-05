CREATE OR REPLACE FUNCTION trigger_set_timestamp()
RETURNS TRIGGER AS $$ BEGIN NEW.updated_at = NOW(); RETURN NEW; END; $$ LANGUAGE plpgsql;
ALTER TABLE 
    users_recipes_browsing_history 
ADD 
    created_at TIMESTAMP NOT NULL DEFAULT CURRENT_TIMESTAMP 
