ALTER TABLE version_part
    DROP CONSTRAINT fk_version_part_version;

ALTER TABLE version_part
    ADD CONSTRAINT fk_version_part_version FOREIGN KEY (version_id) REFERENCES version(id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED;
