package br.com.gabryel.adventofcode.y2023.d22

import br.com.gabryel.adventofcode.y2023.displace
import br.com.gabryel.adventofcode.y2023.intersects
import br.com.gabryel.adventofcode.util.readLines


fun main() {
    listOf("sample", "input").forEach { file ->
        val slabs = readLines(2023, 22, file).map { line -> line.toSlab() }

        val slabsStats = SandSlab(slabs)

        println("[Destructable Slabs ][$file] ${slabsStats.findNonSingleSupporting()}")
        println("[Sum of Caused Falls][$file] ${slabsStats.findSumOfCausedFalls()}")
    }
}

private class SandSlab(lines: List<Slab>) {
    private val fallen = dropAll(lines.sortedBy { it.height.first })

    private val supporterToSupported = fallen.associateWith { slab -> fallen.filter { other -> other.isSupportedBy(slab) } }
    private val supportedToSupporter = fallen.associateWith { slab -> fallen.filter { other -> slab.isSupportedBy(other) } }

    val singleSupporters = supportedToSupporter
        .filterValues { it.size == 1 }
        .values.flatten().distinct()

    fun findNonSingleSupporting() = fallen.size - singleSupporters.size

    fun findSumOfCausedFalls() = singleSupporters.sumOf { it.findBlocksWhoWouldFallTogether() }

    private fun Slab.findBlocksWhoWouldFallTogether(): Int {
        val fallen = mutableSetOf<Slab>()
        val stillToFall = ArrayDeque<Slab>().also {
            it.add(this)
        }

        while (stillToFall.isNotEmpty()) {
            val curr = stillToFall.removeFirst()
            fallen.add(curr)

            val supportedByCurr = supporterToSupported[curr]!!
            val supportersToSupportedToCurr = supportedByCurr.associateWith { supportedToSupporter[it]!! }

            val thoseWhoWillFall = supportersToSupportedToCurr
                .filter { (_, supports) -> supports.all { it in fallen } }
                .keys

            stillToFall.addAll(thoseWhoWillFall)
        }

        return fallen.size - 1
    }

    private fun dropAll(toFall: List<Slab>, fallen: List<Slab> = emptyList()): List<Slab> {
        val current = toFall.firstOrNull()
            ?: return fallen

        val biggestHeightBelow = fallen
            .filter { it.height.last < current.height.first }
            .filter(current::intersectsXY)
            .maxOfOrNull { it.height.last }
            ?: 0

        val displacement = (biggestHeightBelow - current.height.first) + 1
        val fallenBlock = current.copy(height = current.height.displace(displacement))

        return dropAll(toFall.drop(1), fallen + fallenBlock)
    }
}

private data class Slab(val width: IntRange, val depth: IntRange, val height: IntRange) {
    fun isSupportedBy(other: Slab) = isOneLevelAbove(other) && intersectsXY(other)

    fun intersectsXY(other: Slab) = other.width.intersects(width) && other.depth.intersects(depth)

    fun isOneLevelAbove(other: Slab) = height.first == other.height.last + 1
}

private fun String.toSlab(): Slab {
    val coordinates = split("~")

    val (start, end) = coordinates.map { it.split(",").map { it.toInt() } }
    val (startX, startY, startZ) = start
    val (endX, endY, endZ) = end

    return Slab(startX .. endX, startY .. endY, startZ .. endZ)
}
