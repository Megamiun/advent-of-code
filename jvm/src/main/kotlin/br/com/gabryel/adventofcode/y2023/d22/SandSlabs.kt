package br.com.gabryel.adventofcode.y2023.d22

import br.com.gabryel.adventofcode.util.displace
import br.com.gabryel.adventofcode.util.intersects

fun findNonSingleSupporting(lines: List<String>) =
    SandSlab(lines.map { line -> line.toSlab() }).findNonSingleSupporting()

fun findSumOfCausedFalls(lines: List<String>) = SandSlab(lines.map { line -> line.toSlab() }).findSumOfCausedFalls()

private class SandSlab(lines: List<Slab>) {
    private val fallen = dropAll(lines.sortedBy { it.height.first })

    private val supportedToSupporter =
        fallen.associateWith { slab -> fallen.filter { other -> slab.isSupportedBy(other) } }

    private val singleSupporters = supportedToSupporter
        .filterValues { it.size == 1 }
        .values.flatten().distinct()

    fun findNonSingleSupporting() = fallen.size - singleSupporters.size

    fun findSumOfCausedFalls(): Int {
        val supporterToSupported = mutableMapOf<Slab, List<Slab>>()
        return singleSupporters.sumOf { it.findBlocksWhoWouldFallTogether(supporterToSupported) }
    }

    private fun Slab.findBlocksWhoWouldFallTogether(supporterToSupported: MutableMap<Slab, List<Slab>>): Int {
        val newFalls = mutableSetOf<Slab>()
        val stillToFall = ArrayDeque<Slab>().also {
            it.add(this)
        }

        while (stillToFall.isNotEmpty()) {
            val curr = stillToFall.removeFirst()
            newFalls.add(curr)

            val supportedByCurr = supporterToSupported.getOrPut(curr) { fallen.filter { it.isSupportedBy(curr) } }
            val supportersToSupportedToCurr = supportedByCurr.associateWith { supportedToSupporter[it]!! }

            val thoseWhoWillFall = supportersToSupportedToCurr
                .filter { (_, supports) -> supports.all { it in newFalls } }
                .keys

            stillToFall.addAll(thoseWhoWillFall)
        }

        return newFalls.size - 1
    }

    private fun dropAll(toFall: List<Slab>, fallen: List<Slab> = emptyList()): List<Slab> {
        val current = toFall.firstOrNull()
            ?: return fallen

        val highestBellow = fallen
            .filter { it.height.last < current.height.first }
            .filter(current::intersectsXY)
            .maxOfOrNull { it.height.last } ?: 0

        val displacement = (highestBellow - current.height.first) + 1
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

    val (start, end) = coordinates.map { it.split(",").map(String::toInt) }
    val (startX, startY, startZ) = start
    val (endX, endY, endZ) = end

    return Slab(startX..endX, startY..endY, startZ..endZ)
}
