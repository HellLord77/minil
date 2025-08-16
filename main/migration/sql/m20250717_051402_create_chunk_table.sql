ALTER TABLE chunk
    DROP CONSTRAINT fk_chunk_upload_part;

ALTER TABLE chunk
    ADD CONSTRAINT fk_chunk_upload_part FOREIGN KEY (upload_part_id) REFERENCES upload_part(id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED;

ALTER TABLE chunk
    DROP CONSTRAINT fk_chunk_version_part;

ALTER TABLE chunk
    ADD CONSTRAINT fk_chunk_version_part FOREIGN KEY (version_part_id) REFERENCES version_part(id) ON DELETE CASCADE DEFERRABLE INITIALLY DEFERRED;
