-- Add migration script here
CREATE TABLE "public"."account" (
    "id" text NOT NULL,
    "user_id" text NOT NULL,
    "type" text NOT NULL,
    "provider" text NOT NULL,
    "provider_account_id" text NOT NULL,
    "refresh_token" text,
    "access_token" text,
    "expires_at" integer,
    "token_type" text,
    "scope" text,
    "id_token" text,
    "session_state" text,
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."session" (
    "id" text NOT NULL,
    "session_token" text NOT NULL,
    "user_id" text NOT NULL,
    "expires" timestamp(3) NOT NULL,
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."users" (
    "id" text NOT NULL,
    "name" text,
    "email" text,
    "email_verified" timestamp(3),
    "image" text,
    PRIMARY KEY ("id")
);

CREATE TABLE "public"."verification_token" (
    "identifier" text NOT NULL,
    "token" text NOT NULL,
    "expires" timestamp(3) NOT NULL,
    PRIMARY KEY ("identifier","token")
);

CREATE UNIQUE INDEX "account.provider_provider_account_id" ON "public"."account"("provider","provider_account_id");
