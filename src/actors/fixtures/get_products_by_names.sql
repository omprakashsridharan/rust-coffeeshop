START TRANSACTION;
INSERT INTO product.products("name", "type", price, image) VALUES ('COFFEE', 1, 19.99,''), ('CHIPS', 2, 1.99,''), ('MILK', 3, 2.99,'');
COMMIT;
