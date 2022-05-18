--
-- CreateActiveAdminComments : 20210306222803
--

BEGIN;

CREATE TABLE "active_admin_comments" ("id" bigserial primary key, "namespace" character varying, "body" text, "resource_type" character varying, "resource_id" bigint, "author_type" character varying, "author_id" bigint, "created_at" timestamp(6) NOT NULL, "updated_at" timestamp(6) NOT NULL);

CREATE INDEX "index_active_admin_comments_on_resource" ON "active_admin_comments" ("resource_type", "resource_id");

CREATE INDEX "index_active_admin_comments_on_author" ON "active_admin_comments" ("author_type", "author_id");

CREATE INDEX "index_active_admin_comments_on_namespace" ON "active_admin_comments" ("namespace");

INSERT INTO schema_migrations (version) VALUES ('20210306222803');
