SELECT
    classes.id,
    classes.hash
FROM manifest_classes as classes
WHERE classes.hash IN ({})