-- CreateEnum
CREATE TYPE "WSCardType" AS ENUM ('Character', 'Climax', 'Event');

-- CreateEnum
CREATE TYPE "WSCardSide" AS ENUM ('W', 'S');

-- CreateEnum
CREATE TYPE "WSCardColor" AS ENUM ('Red', 'Blue', 'Green', 'Yellow', 'Purple', 'Colorless');

-- CreateEnum
CREATE TYPE "WSCardTrigger" AS ENUM ('None', 'Soul', 'DoubleSoul', 'Pool', 'Comeback', 'Return', 'Draw', 'Treasure', 'Shot', 'Gate', 'Choice', 'Standby');

-- CreateTable
CREATE TABLE "WSCard" (
    "id" SERIAL NOT NULL,
    "image" TEXT NOT NULL,
    "card_name" TEXT NOT NULL,
    "card_no" TEXT NOT NULL,
    "product_id" INTEGER NOT NULL,
    "expansion_id" TEXT NOT NULL,
    "rarity" TEXT NOT NULL,
    "side" "WSCardSide" NOT NULL,
    "card_type" "WSCardType" NOT NULL,
    "color" "WSCardColor" NOT NULL,
    "level" INTEGER NOT NULL,
    "cost" INTEGER NOT NULL,
    "power" INTEGER NOT NULL,
    "soul" INTEGER NOT NULL,
    "trigger" "WSCardTrigger" NOT NULL,
    "text" TEXT NOT NULL DEFAULT '-',
    "flavor" TEXT DEFAULT '-',
    "illustrator" TEXT DEFAULT '-',

    CONSTRAINT "WSCard_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "WSProduct" (
    "id" SERIAL NOT NULL,
    "product" TEXT NOT NULL,

    CONSTRAINT "WSProduct_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "WSExpansion" (
    "expansion_id" TEXT NOT NULL,
    "expansion_name" TEXT NOT NULL,

    CONSTRAINT "WSExpansion_pkey" PRIMARY KEY ("expansion_id")
);

-- CreateTable
CREATE TABLE "WSTrait" (
    "id" SERIAL NOT NULL,
    "trait_name" TEXT NOT NULL,

    CONSTRAINT "WSTrait_pkey" PRIMARY KEY ("id")
);

-- CreateTable
CREATE TABLE "WSTraitsOnCards" (
    "trait_id" INTEGER NOT NULL,
    "card_id" INTEGER NOT NULL,

    CONSTRAINT "WSTraitsOnCards_pkey" PRIMARY KEY ("trait_id","card_id")
);

-- AddForeignKey
ALTER TABLE "WSCard" ADD CONSTRAINT "WSCard_product_id_fkey" FOREIGN KEY ("product_id") REFERENCES "WSProduct"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "WSCard" ADD CONSTRAINT "WSCard_expansion_id_fkey" FOREIGN KEY ("expansion_id") REFERENCES "WSExpansion"("expansion_id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "WSTraitsOnCards" ADD CONSTRAINT "WSTraitsOnCards_trait_id_fkey" FOREIGN KEY ("trait_id") REFERENCES "WSTrait"("id") ON DELETE RESTRICT ON UPDATE CASCADE;

-- AddForeignKey
ALTER TABLE "WSTraitsOnCards" ADD CONSTRAINT "WSTraitsOnCards_card_id_fkey" FOREIGN KEY ("card_id") REFERENCES "WSCard"("id") ON DELETE RESTRICT ON UPDATE CASCADE;
