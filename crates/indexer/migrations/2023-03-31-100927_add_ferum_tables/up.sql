-- CreateTable
CREATE TABLE "highest_processed_block" (
    "block" BIGINT NOT NULL,

    PRIMARY KEY ("block")
);

CREATE TABLE "orders" (
    "counter" BIGINT NOT NULL,
    "owner" VARCHAR(80) NOT NULL,
    "instrument_type" VARCHAR(80) NOT NULL,
    "quote_type" VARCHAR(80) NOT NULL,
    "side" VARCHAR(80) NOT NULL,
    "original_qty" DECIMAL NOT NULL,
    "price" DECIMAL NOT NULL,
    "type" VARCHAR(80) NOT NULL,
    "status" VARCHAR(80) NOT NULL,
    "client_order_id" VARCHAR(80) NOT NULL,
    "cancel_agent" VARCHAR(80) NOT NULL,
    "remaining_qty" DECIMAL NOT NULL,
    "update_count" BIGINT NOT NULL,
    "pagination_id" BIGINT NOT NULL,

    CONSTRAINT "orders_pkey" PRIMARY KEY ("counter","owner")
);

-- CreateTable
CREATE TABLE "quotes" (
    "instrument_type" VARCHAR(80) NOT NULL,
    "quote_type" VARCHAR(80) NOT NULL,
    "chain_timestamp_microseconds" BIGINT NOT NULL,
    "max_Bid" DECIMAL NOT NULL,
    "bid_size" DECIMAL NOT NULL,
    "min_ask" DECIMAL NOT NULL,
    "ask_size" DECIMAL NOT NULL,

    CONSTRAINT "quote_pkey" PRIMARY KEY ("instrument_type","quote_type","chain_timestamp_microseconds")
);

-- CreateTable
CREATE TABLE "executions" (
    "exec_counter" BIGINT NOT NULL,
    "order_counter" BIGINT NOT NULL,
    "order_owner" VARCHAR(80) NOT NULL,
    "price" DECIMAL NOT NULL,
    "qty" DECIMAL NOT NULL,
    "chain_timestamp_microseconds" BIGINT NOT NULL,
    "pagination_id" BIGINT NOT NULL,

    CONSTRAINT "execution_pkey" PRIMARY KEY ("exec_counter","order_counter","order_owner")
);
