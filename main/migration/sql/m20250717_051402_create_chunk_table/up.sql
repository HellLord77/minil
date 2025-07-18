CREATE TRIGGER `trg_chunk_before_object_delete`
    BEFORE DELETE
    ON `object`
    FOR EACH ROW
BEGIN
    DELETE FROM `chunk` WHERE `object_id` = OLD.`id`;
END;

CREATE TRIGGER `trg_chunk_before_part_delete`
    BEFORE DELETE
    ON `part`
    FOR EACH ROW
BEGIN
    DELETE FROM `chunk` WHERE `part_id` = OLD.`id`;
END;
