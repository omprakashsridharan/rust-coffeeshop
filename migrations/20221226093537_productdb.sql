START TRANSACTION;
DO $EF$ BEGIN IF NOT EXISTS(
  SELECT 1
  FROM pg_namespace
  WHERE nspname = 'product'
) THEN CREATE SCHEMA product;
END IF;
END $EF$;
CREATE EXTENSION IF NOT EXISTS "uuid-ossp";
CREATE TABLE product.products (
  id uuid NOT NULL DEFAULT (uuid_generate_v4()),
  "name" text NOT NULL,
  "type" integer NOT NULL,
  price DECIMAL(12, 2) NOT NULL,
  "image" text NOT NULL,
  created timestamp with time zone NOT NULL DEFAULT (now()),
  updated timestamp with time zone NULL,
  CONSTRAINT pk_product_products PRIMARY KEY (id)
);
CREATE UNIQUE INDEX ix_product_products_id ON product.products (id);
CREATE UNIQUE INDEX ix_product_products_name ON product.products ("name");
CREATE UNIQUE INDEX ix_product_products_type ON product.products ("type");
COMMIT;
