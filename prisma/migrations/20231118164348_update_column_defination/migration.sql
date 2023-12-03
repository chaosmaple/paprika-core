/*
  Warnings:

  - You are about to alter the column `image` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(50)`.
  - You are about to alter the column `card_name` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(30)`.
  - You are about to alter the column `card_no` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(20)`.
  - You are about to alter the column `rarity` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(10)`.
  - You are about to alter the column `level` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Integer` to `SmallInt`.
  - You are about to alter the column `cost` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Integer` to `SmallInt`.
  - You are about to alter the column `soul` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Integer` to `SmallInt`.
  - You are about to alter the column `text` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(200)`.
  - You are about to alter the column `flavor` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(100)`.
  - You are about to alter the column `illustrator` on the `WSCard` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(20)`.
  - You are about to alter the column `expansion_name` on the `WSExpansion` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(50)`.
  - You are about to alter the column `trait_name` on the `WSTrait` table. The data in that column could be lost. The data in that column will be cast from `Text` to `VarChar(20)`.
  - A unique constraint covering the columns `[product]` on the table `WSProduct` will be added. If there are existing duplicate values, this will fail.

*/
-- AlterTable
ALTER TABLE "WSCard" ALTER COLUMN "image" SET DATA TYPE VARCHAR(50),
ALTER COLUMN "card_name" SET DATA TYPE VARCHAR(30),
ALTER COLUMN "card_no" SET DATA TYPE VARCHAR(20),
ALTER COLUMN "rarity" SET DATA TYPE VARCHAR(10),
ALTER COLUMN "level" SET DATA TYPE SMALLINT,
ALTER COLUMN "cost" SET DATA TYPE SMALLINT,
ALTER COLUMN "soul" SET DATA TYPE SMALLINT,
ALTER COLUMN "text" SET DATA TYPE VARCHAR(200),
ALTER COLUMN "flavor" SET DATA TYPE VARCHAR(100),
ALTER COLUMN "illustrator" SET DATA TYPE VARCHAR(20);

-- AlterTable
ALTER TABLE "WSExpansion" ALTER COLUMN "expansion_name" SET DATA TYPE VARCHAR(50);

-- AlterTable
ALTER TABLE "WSTrait" ALTER COLUMN "trait_name" SET DATA TYPE VARCHAR(20);

-- CreateIndex
CREATE UNIQUE INDEX "WSProduct_product_key" ON "WSProduct"("product");
